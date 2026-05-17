use eframe::egui;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex, atomic::Ordering},
    thread,
};

use crate::{
    algo::{alt, alt_arc_flags, astar, astar_arc_flags, astar_bidirectional, astar_bidirectional_arc_flags, dijkstra, dijkstra_arc_flags, dijkstra_bidirectional}, graph::Graph
};

// ── Algorithm selector ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlgoChoice {
    Dijkstra,
    DijkstraArcFlags,
    Astar,
    Alt,
    DijkstraBidirectional,
    AstarBidirectional,
    AstarArcFlags,
    AltArcFlags,
    AstarBidirectionalArcFlags,
}

impl AlgoChoice {
    const ALL: &'static [AlgoChoice] = &[
        AlgoChoice::Dijkstra,
        AlgoChoice::DijkstraArcFlags,
        AlgoChoice::Astar,
        AlgoChoice::Alt,
        AlgoChoice::DijkstraBidirectional,
        AlgoChoice::AstarBidirectional,
        AlgoChoice::AstarArcFlags,
        AlgoChoice::AltArcFlags,
        AlgoChoice::AstarBidirectionalArcFlags
    ];

    fn label(self) -> &'static str {
        match self {
            AlgoChoice::Dijkstra => "Dijkstra",
            AlgoChoice::DijkstraArcFlags => "Dijkstra arc flags",
            AlgoChoice::Astar => "A*",
            AlgoChoice::Alt => "Random Alt",
            AlgoChoice::DijkstraBidirectional => "Bidirectional Dijkstra",
            AlgoChoice::AstarBidirectional => "Bidirectional A*",
            AlgoChoice::AstarArcFlags => "Arc flags",
            AlgoChoice::AltArcFlags => "Arc flags with alt potential",
            AlgoChoice::AstarBidirectionalArcFlags => "Bidirectional arc flags",
        }
    }
}

// ── App state ─────────────────────────────────────────────────────────────────

const RESIZE_COUNTDOWN_THRESHOLD: u8 = 4;
const LOD_COOLDOWN_THRESHOLD: u8 = 5;
const CLICK_RADIUS: f32 = 8.0;


/// Which endpoint the user is currently assigning via map click.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Assigning {
    Source,
    Sink,
    None,
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let i = (h * 6.0).floor() as u32;
    let f = h * 6.0 - i as f32;
    let (p, q, t) = (v * (1.0 - s), v * (1.0 - f * s), v * (1.0 - (1.0 - f) * s));
    let (r, g, b) = match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn region_color(region_id: u32) -> egui::Color32 {
    let h = ((region_id.wrapping_mul(2654435761)) >> 16) as f32 / 65535.0;
    let (r, g, b) = hsv_to_rgb(h, 0.70, 0.90);
    egui::Color32::from_rgb(r, g, b)
}

pub struct VisApp {
    pub graph: Arc<Graph>,
    drawn_indices: Vec<usize>,
    vertex_pos: Vec<egui::Pos2>,
    last_size: egui::Vec2,
    resize_countdown: u8,
    big_vertices: HashSet<u32>,

    // button options
    show_regions: bool,
    show_landmarks: bool,

    // user-controlled state
    source: u32,
    sink: u32,
    algo: AlgoChoice,
    source_input: String,
    sink_input: String,

    // click-assignment mode
    assigning: Assigning,

    // result
    result: Arc<Mutex<Option<f32>>>,
    running: bool,

    color_snapshot: Vec<egui::Color32>,

    // zoom / pan
    zoom: f32,
    pan: egui::Vec2,
    base_stride: usize,
    lod_countdown: u8,
    last_lod_zoom: f32,

    // coordinate transform (set in precompute)
    min_lat: f32,
    min_lon: f32,
    lat_range: f32,
    lon_range: f32,
    world_size: egui::Vec2,
}

impl VisApp {
    pub fn new(
        graph: Arc<Graph>,
        big_vertices: HashSet<u32>,
        source: u32,
        sink: u32,
    ) -> Self {
        Self {
            graph,
            drawn_indices: Vec::new(),
            vertex_pos: Vec::new(),
            show_regions: false,
            last_size: egui::Vec2::ZERO,
            resize_countdown: 0,
            big_vertices,
            show_landmarks: false,
            source,
            sink,
            algo: AlgoChoice::Dijkstra,
            source_input: source.to_string(),
            sink_input: sink.to_string(),
            assigning: Assigning::None,
            result: Arc::new(Mutex::new(None)),
            running: false,
            color_snapshot: Vec::new(),
            zoom: 1.0,
            pan: egui::Vec2::ZERO,
            base_stride: 1,
            lod_countdown: 0,
            last_lod_zoom: 1.0,
            min_lat: 0.0,
            min_lon: 0.0,
            lat_range: 1.0,
            lon_range: 1.0,
            world_size: egui::Vec2::new(1.0, 1.0),
        }
    }
    fn edge_zoom_threshold(&self) -> f32 {
        // ~1k nodes → threshold 2.0  (tiny graph, show edges almost immediately)
        // ~30k nodes → threshold 6.0  (city)
        // ~300k nodes → threshold 30.0 (country, only when zoomed to city level)
        let t = (self.graph.size as f32).log10().clamp(3.0, 6.0); // log10: 3=1k, 6=1M
        let norm = (t - 3.0) / 3.0; // 0.0 … 1.0
        2.0 + norm * norm * 28.0     // quadratic: 2.0 … 30.0
    }
    /// Project a (lat, lon) coordinate into world space.
    fn project(&self, coords: (f32, f32)) -> egui::Pos2 {
        let x = (coords.1 - self.min_lon) / self.lon_range;
        let y = (coords.0 - self.min_lat) / self.lat_range;
        egui::pos2(x * self.world_size.x, self.world_size.y - y * self.world_size.y)
    }

    fn precompute(&mut self, size: egui::Vec2) {
        if self.graph.vertices.is_empty() {
            return;
        }
        let min_lat = self.graph.vertices.iter().map(|v| v.coords.0).fold(f32::INFINITY,    f32::min);
        let max_lat = self.graph.vertices.iter().map(|v| v.coords.0).fold(f32::NEG_INFINITY, f32::max);
        let min_lon = self.graph.vertices.iter().map(|v| v.coords.1).fold(f32::INFINITY,    f32::min);
        let max_lon = self.graph.vertices.iter().map(|v| v.coords.1).fold(f32::NEG_INFINITY, f32::max);

        self.min_lat   = min_lat;
        self.min_lon   = min_lon;
        self.lat_range = (max_lat - min_lat).max(f32::EPSILON);
        self.lon_range = (max_lon - min_lon).max(f32::EPSILON);
        self.world_size = size;

        const LARGE_GRAPH_THRESHOLD: usize = 30_000;
        self.base_stride = if self.graph.size > LARGE_GRAPH_THRESHOLD {
            (self.graph.size / LARGE_GRAPH_THRESHOLD).max(1)
        } else {
            1
        };

        self.zoom = 1.0;
        self.pan  = egui::Vec2::ZERO;
        self.update_lod(size);
    }

    fn update_lod(&mut self, viewport_size: egui::Vec2) {
        let effective_stride = ((self.base_stride as f32 / self.zoom) as usize).max(1);

        let margin = 40.0 / self.zoom;
        let world_min_x = self.pan.x - margin;
        let world_min_y = self.pan.y - margin;
        let world_max_x = self.pan.x + viewport_size.x / self.zoom + margin;
        let world_max_y = self.pan.y + viewport_size.y / self.zoom + margin;

        let mut new_indices = Vec::new();
        let mut new_pos     = Vec::new();

        for (i, v) in self.graph.vertices.iter().enumerate() {
            if i % effective_stride != 0 { continue; }
            let p = self.project(v.coords);
            if p.x < world_min_x || p.x > world_max_x { continue; }
            if p.y < world_min_y || p.y > world_max_y { continue; }
            new_indices.push(i);
            new_pos.push(p);
        }

        self.color_snapshot = new_indices.iter().map(|&i| {
            let [r, g, b, a] = self.graph.vertices[i].color.load(Ordering::Relaxed).to_be_bytes();
            egui::Color32::from_rgba_premultiplied(r, g, b, a)
        }).collect();

        self.drawn_indices = new_indices;
        self.vertex_pos    = new_pos;
        self.last_lod_zoom = self.zoom;
        self.lod_countdown = 0;
    }

    fn clamp_pan(&mut self, viewport_size: egui::Vec2) {
        let view_w = viewport_size.x / self.zoom;
        let view_h = viewport_size.y / self.zoom;
        self.pan.x = self.pan.x.clamp(-(view_w * 0.5), self.world_size.x - view_w * 0.5);
        self.pan.y = self.pan.y.clamp(-(view_h * 0.5), self.world_size.y - view_h * 0.5);
    }

    fn vertex_at_world(&self, pos: egui::Pos2) -> Option<usize> {
        let radius = CLICK_RADIUS / self.zoom;
        self.vertex_pos.iter().enumerate()
            .filter_map(|(slot, &vpos)| {
                let d = vpos.distance(pos);
                if d <= radius { Some((slot, d)) } else { None }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(slot, _)| slot)
    }

    fn run_algorithm(&mut self) {
        for v in &self.graph.vertices {
            v.recolor(egui::Color32::LIGHT_RED);
        }
        self.color_snapshot = vec![egui::Color32::LIGHT_RED; self.drawn_indices.len()];
        *self.result.lock().unwrap() = None;
        self.running = true;

        let graph  = self.graph.clone();
        let source = self.source;
        let sink   = self.sink;
        let algo   = self.algo;
        let result = self.result.clone();

        thread::spawn(move || {
            let dist = match algo {
                AlgoChoice::Dijkstra                 => dijkstra(&graph, source, sink),
                AlgoChoice::Astar                    => astar(&graph, source, sink),
                AlgoChoice::Alt                      => alt(&graph, source, sink),
                AlgoChoice::DijkstraBidirectional    => dijkstra_bidirectional(&graph, source, sink),
                AlgoChoice::DijkstraArcFlags         => dijkstra_arc_flags(&graph, source, sink),
                AlgoChoice::AstarBidirectional       => astar_bidirectional(&graph, source, sink),
                AlgoChoice::AstarArcFlags            => astar_arc_flags(&graph, source, sink),
                AlgoChoice::AltArcFlags              => alt_arc_flags(&graph, source, sink),
                AlgoChoice::AstarBidirectionalArcFlags => astar_bidirectional_arc_flags(&graph, source, sink),
            };
            *result.lock().unwrap() = dist.distance;
        });
    }
    fn draw_edges(&self, painter: &egui::Painter, rect: egui::Rect, w2s: &impl Fn(egui::Pos2) -> egui::Pos2) {
        let alpha = ((self.zoom - self.edge_zoom_threshold()) / 4.0).clamp(0.0, 1.0);
        let a = (alpha * 180.0) as u8;
        let stroke = egui::Stroke::new(0.8, egui::Color32::from_rgba_premultiplied(120, 140, 160, a));

        let margin = 60.0 / self.zoom;
        let world_min_x = self.pan.x - margin;
        let world_min_y = self.pan.y - margin;
        let world_max_x = self.pan.x + rect.width()  / self.zoom + margin;
        let world_max_y = self.pan.y + rect.height() / self.zoom + margin;

        for (&i, &a_world) in self.drawn_indices.iter().zip(self.vertex_pos.iter()) {
            let v = &self.graph.vertices[i];
            for edge in v.edges.iter() {
                let b_world = self.project(self.graph.vertices[edge.1.to as usize].coords);
                // cull edges whose far endpoint is outside viewport
                if b_world.x < world_min_x || b_world.x > world_max_x
                || b_world.y < world_min_y || b_world.y > world_max_y {
                    continue;
                }
                painter.line_segment([w2s(a_world), w2s(b_world)], stroke);
            }
        }
    }

}



// ── eframe::App ───────────────────────────────────────────────────────────────

impl eframe::App for VisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            for (slot, &i) in self.drawn_indices.iter().enumerate() {
                let [r, g, b, a] = self.graph.vertices[i].color.load(Ordering::Relaxed).to_be_bytes();
                self.color_snapshot[slot] = egui::Color32::from_rgba_premultiplied(r, g, b, a);
            }
        }

        // ── Side panel ────────────────────────────────────────────────────────
        egui::SidePanel::left("controls")
            .min_width(230.0)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.heading("Graph Pathfinding");
                ui.add_space(10.0);

                ui.label("Source vertex");
                ui.horizontal(|ui| {
                    let r = ui.add(
                        egui::TextEdit::singleline(&mut self.source_input).desired_width(80.0),
                    );
                    if r.lost_focus() {
                        if let Ok(v) = self.source_input.trim().parse::<usize>() {
                            if v < self.graph.size {
                                self.source = v as u32;
                            }
                        }
                        self.source_input = self.source.to_string();
                    }
                    let picking = self.assigning == Assigning::Source;
                    if ui.selectable_label(picking, "📍 Pick").clicked() {
                        self.assigning = if picking { Assigning::None } else { Assigning::Source };
                    }
                });

                ui.add_space(6.0);

                ui.label("Sink vertex");
                ui.horizontal(|ui| {
                    let r = ui.add(egui::TextEdit::singleline(&mut self.sink_input).desired_width(80.0));
                    if r.lost_focus() {
                        if let Ok(v) = self.sink_input.trim().parse::<usize>() {
                            if v < self.graph.size {
                                self.sink = v as u32;
                            }
                        }
                        self.sink_input = self.sink.to_string();
                    }
                    let picking = self.assigning == Assigning::Sink;
                    if ui.selectable_label(picking, "📍 Pick").clicked() {
                        self.assigning = if picking { Assigning::None } else { Assigning::Sink };
                    }
                });

                if self.assigning != Assigning::None {
                    let what = match self.assigning {
                        Assigning::Source => "source",
                        Assigning::Sink   => "sink",
                        Assigning::None   => "",
                    };
                    ui.add_space(4.0);
                    ui.colored_label(egui::Color32::YELLOW, format!("⬅ Click a vertex on the map to set {what}"));
                }

                ui.add_space(10.0);

                ui.label("Algorithm");
                egui::ComboBox::from_id_salt("algo_combo")
                    .selected_text(self.algo.label())
                    .width(210.0)
                    .show_ui(ui, |ui| {
                        for &choice in AlgoChoice::ALL {
                            ui.selectable_value(&mut self.algo, choice, choice.label());
                        }
                    });

                ui.add_space(12.0);

                let finished = self.result.lock().unwrap().is_some();
                let can_run  = !self.running || finished;
                if ui.add_enabled(can_run, egui::Button::new("▶  Run").min_size(egui::vec2(210.0, 30.0))).clicked() {
                    self.running = false;
                    self.run_algorithm();
                }

                ui.add_space(8.0);

                if self.running {
                    let guard = self.result.lock().unwrap();
                    match *guard {
                        None => { ui.horizontal(|ui| { ui.spinner(); ui.label("Running…"); }); }
                        Some(d) => {
                            drop(guard);
                            self.running = false;
                            ui.label(format!("✔ Distance: {d:.2}"));
                        }
                    }
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);

                let dot = |ui: &mut egui::Ui, color: egui::Color32, text: &str| {
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                        ui.painter().circle_filled(rect.center(), 6.0, color);
                        ui.label(text);
                    });
                };

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);

                ui.label("Visualisation");
                let region_btn_text = if self.show_regions { "🗺  Hide Regions" } else { "🗺  Show Regions" };
                if ui.add(egui::Button::new(region_btn_text).min_size(egui::vec2(210.0, 28.0))).clicked() {
                    self.show_regions = !self.show_regions;
                    if !self.show_regions {
                        for (slot, &i) in self.drawn_indices.iter().enumerate() {
                            let [r, g, b, a] = self.graph.vertices[i].color.load(Ordering::Relaxed).to_be_bytes();
                            self.color_snapshot[slot] = egui::Color32::from_rgba_premultiplied(r, g, b, a);
                        }
                    }
                }

                let has_landmarks = !self.graph.landmarks.is_empty();
                let lm_btn_text   = if self.show_landmarks { "★  Hide Landmarks" } else { "★  Show Landmarks" };
                if ui.add_enabled(has_landmarks, egui::Button::new(lm_btn_text).min_size(egui::vec2(210.0, 28.0))).clicked() {
                    self.show_landmarks = !self.show_landmarks;
                }
                if !has_landmarks {
                    ui.colored_label(egui::Color32::GRAY, "Run an ALT algorithm first");
                }

                ui.label("Legend");
                dot(ui, egui::Color32::GREEN,      "Source");
                dot(ui, egui::Color32::RED,        "Sink");
                dot(ui, egui::Color32::LIGHT_BLUE, "Visited");
                dot(ui, egui::Color32::LIGHT_RED,  "Unvisited");
                dot(ui, egui::Color32::YELLOW,     "Landmark");

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);
            });

        // ── Main canvas ───────────────────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();

            if self.vertex_pos.is_empty() || self.resize_countdown == RESIZE_COUNTDOWN_THRESHOLD {
                self.resize_countdown = 0;
                self.precompute(size);
            } else if size != self.last_size {
                self.resize_countdown = 1;
            } else if self.resize_countdown != 0 {
                self.resize_countdown += 1;
            }
            self.last_size = size;

            let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

            // ── Zoom ─────────────────────────────────────────────────────────
            let scroll = ctx.input(|i| i.smooth_scroll_delta);
            if scroll.y != 0.0 && rect.contains(ctx.input(|i| i.pointer.hover_pos().unwrap_or_default())) {
                let zoom_factor   = (scroll.y * 0.005).exp();
                let cursor_screen = ctx.input(|i| i.pointer.hover_pos()).unwrap_or(rect.center());
                let cursor_world  = self.pan + (cursor_screen - rect.min) / self.zoom;
                self.zoom = (self.zoom * zoom_factor).clamp(0.5, 1000.0);
                self.pan  = cursor_world - (cursor_screen - rect.min) / self.zoom;
                self.clamp_pan(size);

                let ratio = (self.zoom / self.last_lod_zoom).max(self.last_lod_zoom / self.zoom);
                if ratio > 1.15 {
                    self.lod_countdown = 1;
                }
            }

            // ── Pan ──────────────────────────────────────────────────────────
            if response.dragged() && self.assigning == Assigning::None {
                self.pan -= response.drag_delta() / self.zoom;
                self.clamp_pan(size);
                if response.drag_delta().length() > 20.0 && self.lod_countdown == 0 {
                    self.lod_countdown = 1;
                }
            }

            // ── LOD cooldown ─────────────────────────────────────────────────
            if self.lod_countdown != 0 {
                self.lod_countdown += 1;
                if self.lod_countdown >= LOD_COOLDOWN_THRESHOLD {
                    self.update_lod(size);
                }
            }

  
            // ── Click picking ─────────────────────────────────────────────────
            if response.clicked() && self.assigning != Assigning::None {
                if let Some(cursor) = response.interact_pointer_pos() {
                    let world_click = egui::pos2(
                        self.pan.x + (cursor.x - rect.min.x) / self.zoom,
                        self.pan.y + (cursor.y - rect.min.y) / self.zoom,
                    );
                    if let Some(slot) = self.vertex_at_world(world_click) {
                        let idx = self.drawn_indices[slot];
                        match self.assigning {
                            Assigning::Source => { self.source = idx as u32; self.source_input = idx.to_string(); }
                            Assigning::Sink   => { self.sink   = idx as u32; self.sink_input   = idx.to_string(); }
                            Assigning::None   => {}
                        }
                        self.assigning = Assigning::None;
                    }
                }
            }

            let w2s = |world: egui::Pos2| -> egui::Pos2 {
                rect.min + egui::vec2(
                    (world.x - self.pan.x) * self.zoom,
                    (world.y - self.pan.y) * self.zoom,
                )
            };

            let painter = ui.painter_at(rect);

            if self.zoom >= self.edge_zoom_threshold() {
                self.draw_edges(&painter, rect, &w2s);
            }
            // ── Draw sampled vertices ─────────────────────────────────────────
            for (slot, &i) in self.drawn_indices.iter().enumerate() {
                if i == self.source as usize || i == self.sink as usize { continue; }
                let cur_color = if self.show_regions {
                    self.graph.regions.as_ref()
                        .and_then(|r| r.get(i as u32))
                        .map(|&rid| region_color(rid))
                        .unwrap_or(egui::Color32::LIGHT_RED)
                } else {
                    self.color_snapshot[slot]
                };
                let screen_pos = w2s(self.vertex_pos[slot]);
                let r = if self.big_vertices.contains(&(i as u32)) { 5.0 } else { 1.5 };
                painter.circle_filled(screen_pos, r, cur_color);
            }

            
            // ── Source / sink (always visible) ────────────────────────────────
            for (idx, color) in [(self.source, egui::Color32::GREEN), (self.sink, egui::Color32::RED)] {
                let screen_pos = w2s(self.project(self.graph.vertices[idx as usize].coords));
                painter.circle_stroke(screen_pos, 9.0, egui::Stroke::new(2.0, color));
                painter.circle_filled(screen_pos, 5.0, color);
            }

            // ── Landmarks ────────────────────────────────────────────────────
            if self.show_landmarks {
                for &lm_id in self.graph.landmarks.keys() {
                    let screen_pos = w2s(self.project(self.graph.vertices[lm_id as usize].coords));
                    painter.circle_filled(screen_pos, 8.0, egui::Color32::YELLOW);
                    painter.circle_stroke(screen_pos, 8.0, egui::Stroke::new(2.0, egui::Color32::GOLD));
                }
            }
        });

        ctx.request_repaint();
    }
}
