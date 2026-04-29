use eframe::egui;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex, atomic::Ordering},
    thread,
};

use crate::{
    algo::{
        alt::landmarks::alt_potential, arc_flags::arc_flags_astar::arc_flags_astar, arc_flags::bidirecional::bidirectional_arcflags, astar::{
            astar,
            bidirectional::bidirectional_astar,
            heuristics::{earth_dist, middle_dist, rev},
        }, dijkstra::{bidirectional::bidirectional_dijkstra, dijkstra}
    },
    graph::Graph,
};

// ── Algorithm selector ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlgoChoice {
    Dijkstra,
    Astar,
    Alt,
    BidirectionalDijkstra,
    BidirectionalAstar,
    BidirectionalAstarMiddle,
    ArcFlags,
    ArcFlagsAlt,
    BidirectionalArcFlags,
}

impl AlgoChoice {
    const ALL: &'static [AlgoChoice] = &[
        AlgoChoice::Dijkstra,
        AlgoChoice::Astar,
        AlgoChoice::Alt,
        AlgoChoice::BidirectionalDijkstra,
        AlgoChoice::BidirectionalAstar,
        AlgoChoice::BidirectionalAstarMiddle,
        AlgoChoice::ArcFlags,
        AlgoChoice::ArcFlagsAlt,
        AlgoChoice::BidirectionalArcFlags
    ];

    fn label(self) -> &'static str {
        match self {
            AlgoChoice::Dijkstra => "Dijkstra",
            AlgoChoice::Astar => "A*",
            AlgoChoice::Alt => "Random Alt",
            AlgoChoice::BidirectionalDijkstra => "Bidirectional Dijkstra",
            AlgoChoice::BidirectionalAstar => "Bidirectional A*",
            AlgoChoice::BidirectionalAstarMiddle => "Bidirectional A* (middle)",
            AlgoChoice::ArcFlags => "Arc flags",
            AlgoChoice::ArcFlagsAlt => "Arc flags with random alt potential",
            AlgoChoice::BidirectionalArcFlags => "Bidirectional arc flags",
        }
    }
}

// ── App state ─────────────────────────────────────────────────────────────────

const RESIZE_COUNTDOWN_THRESHOLD: u8 = 4;
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
}

impl VisApp {
    pub fn new(
        graph: Arc<Graph>,
        big_vertices: HashSet<u32>,
        source: u32,
        sink: u32,
    ) -> Self {
        let color_snapshot = vec![egui::Color32::LIGHT_RED; graph.size];
        Self {
            graph,
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
            color_snapshot,
        }
    }

    fn precompute(&mut self, size: egui::Vec2) {
        if self.graph.vertices.is_empty() {
            return;
        }
        let min_lat = self.graph.vertices.iter().map(|v| v.coords.0).fold(f32::INFINITY,    f32::min);
        let max_lat = self.graph.vertices.iter().map(|v| v.coords.0).fold(f32::NEG_INFINITY, f32::max);
        let min_lon = self.graph.vertices.iter().map(|v| v.coords.1).fold(f32::INFINITY,    f32::min);
        let max_lon = self.graph.vertices.iter().map(|v| v.coords.1).fold(f32::NEG_INFINITY, f32::max);
        let lat_range = max_lat - min_lat;
        let lon_range = max_lon - min_lon;

        self.vertex_pos = self.graph.vertices.iter().map(|v| {
            let x = (v.coords.1 - min_lon) / lon_range;
            let y = (v.coords.0 - min_lat) / lat_range;
            egui::pos2(x * size.x, size.y - y * size.y)
        }).collect();
    }

    fn vertex_at(&self, pos: egui::Pos2) -> Option<usize> {
        self.vertex_pos
            .iter()
            .enumerate()
            .filter_map(|(i, &vpos)| {
                let dist = vpos.distance(pos);
                if dist <= CLICK_RADIUS {
                    Some((i, dist))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(i, _)| i)
    }

    /// Deterministic colour from a u32 region id — spreads hues evenly.
    fn region_color(region_id: u32) -> egui::Color32 {
        // Cheap integer hash → HSV hue
        let h = ((region_id.wrapping_mul(2654435761)) >> 16) as f32 / 65535.0;
        // HSV (h, 0.7, 0.9) → RGB
        let (r, g, b) = hsv_to_rgb(h, 0.7, 0.9);
        egui::Color32::from_rgb(r, g, b)
    }

    /// Reset vertex colors and spawn the selected algorithm in a background thread.
    fn run_algorithm(&mut self) {
        for v in &self.graph.vertices {
            v.recolor(egui::Color32::LIGHT_RED);
        }
        self.color_snapshot = vec![egui::Color32::LIGHT_RED; self.graph.size];
        *self.result.lock().unwrap() = None;
        self.running = true;

        let graph = self.graph.clone();
        let source = self.source;
        let sink = self.sink;
        let algo = self.algo;
        let result = self.result.clone();

        thread::spawn(move || {
            let dist = match algo {
                AlgoChoice::Dijkstra => dijkstra(&graph, source, sink, true),
                AlgoChoice::Astar => astar(&graph, source, sink, true, earth_dist),
                AlgoChoice::Alt => astar(&graph, source, sink, true, alt_potential),
                AlgoChoice::BidirectionalDijkstra => {
                    bidirectional_dijkstra(&graph, source, sink, true)
                }
                AlgoChoice::BidirectionalAstar => {
                    bidirectional_astar(&graph, source, sink, true, earth_dist, rev(earth_dist))
                }
                AlgoChoice::BidirectionalAstarMiddle => {
                    let heura = middle_dist(earth_dist);
                    bidirectional_astar(&graph, source, sink, true, heura.0, heura.1)
                }
                AlgoChoice::ArcFlags => {
                    arc_flags_astar(&graph, source, sink, true, earth_dist)
                }
                AlgoChoice::ArcFlagsAlt => {
                    arc_flags_astar(&graph, source, sink, true, alt_potential)
                }
                AlgoChoice::BidirectionalArcFlags => {
                    bidirectional_arcflags(&graph, source, sink, true, earth_dist, rev(earth_dist))
                }
            };
            *result.lock().unwrap() = dist.distance;
        });
    }
}

// ── eframe::App ───────────────────────────────────────────────────────────────

impl eframe::App for VisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            for (i, v) in self.graph.vertices.iter().enumerate() {
                let [r, g, b, a] = v.color.load(Ordering::Relaxed).to_be_bytes();
                self.color_snapshot[i] = egui::Color32::from_rgba_premultiplied(r, g, b, a);
            }
        }
        // ── Side panel ────────────────────────────────────────────────────────
        egui::SidePanel::left("controls")
            .min_width(230.0)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.heading("Graph Pathfinding");
                ui.add_space(10.0);

                // ── Source ───────────────────────────────────────────────────────
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
                        // re-sync display in case the value was clamped / invalid
                        self.source_input = self.source.to_string();
                    }
                    let picking = self.assigning == Assigning::Source;
                    if ui.selectable_label(picking, "📍 Pick").clicked() {
                        self.assigning = if picking {
                            Assigning::None
                        } else {
                            Assigning::Source
                        };
                    }
                });

                ui.add_space(6.0);

                // ── Sink ─────────────────────────────────────────────────────────
                ui.label("Sink vertex");
                ui.horizontal(|ui| {
                    let r = ui
                        .add(egui::TextEdit::singleline(&mut self.sink_input).desired_width(80.0));
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
                        self.assigning = if picking {
                            Assigning::None
                        } else {
                            Assigning::Sink
                        };
                    }
                });

                // Picking hint
                if self.assigning != Assigning::None {
                    let what = match self.assigning {
                        Assigning::Source => "source",
                        Assigning::Sink => "sink",
                        Assigning::None => "",
                    };
                    ui.add_space(4.0);
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        format!("⬅ Click a vertex on the map to set {what}"),
                    );
                }

                ui.add_space(10.0);

                // ── Algorithm ────────────────────────────────────────────────────
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

                // ── Run ──────────────────────────────────────────────────────────
                let finished = self.result.lock().unwrap().is_some();
                let can_run = !self.running || finished;
                if ui
                    .add_enabled(
                        can_run,
                        egui::Button::new("▶  Run").min_size(egui::vec2(210.0, 30.0)),
                    )
                    .clicked()
                {
                    self.running = false;
                    self.run_algorithm();
                }

                ui.add_space(8.0);

                // ── Result ───────────────────────────────────────────────────────
                if self.running {
                    let guard = self.result.lock().unwrap();
                    match *guard {
                        None => {
                            ui.horizontal(|ui| {
                                ui.spinner();
                                ui.label("Running…");
                            });
                        }
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

                // ── Legend ───────────────────────────────────────────────────────
                ui.label("Legend");
                let dot = |ui: &mut egui::Ui, color: egui::Color32, text: &str| {
                    ui.horizontal(|ui| {
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                        ui.painter().circle_filled(rect.center(), 6.0, color);
                        ui.label(text);
                    });
                };

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);

                // ── Regions ──────────────────────────────────────────────────
                ui.label("Visualisation");
                let region_btn_text = if self.show_regions {
                    "🗺  Hide Regions"
                } else {
                    "🗺  Show Regions"
                };
                if ui
                    .add(egui::Button::new(region_btn_text)
                        .min_size(egui::vec2(210.0, 28.0)))
                    .clicked()
                {
                    self.show_regions = !self.show_regions;
                    // When turning off, restore the algorithm colour snapshot.
                    if !self.show_regions {
                        for (i, v) in self.graph.vertices.iter().enumerate() {
                            let [r, g, b, a] = v.color.load(Ordering::Relaxed).to_be_bytes();
                            self.color_snapshot[i] =
                                egui::Color32::from_rgba_premultiplied(r, g, b, a);
                        }
                    }
                }

                let has_landmarks = !self.graph.landmarks.is_empty();
                let lm_btn_text = if self.show_landmarks { "★  Hide Landmarks" } else { "★  Show Landmarks" };
                if ui
                    .add_enabled(
                        has_landmarks,
                        egui::Button::new(lm_btn_text).min_size(egui::vec2(210.0, 28.0)),
                    )
                    .clicked()
                {
                    self.show_landmarks = !self.show_landmarks;
                }
                if !has_landmarks {
                    ui.colored_label(egui::Color32::GRAY, "Run an ALT algorithm first");
                }

                dot(ui, egui::Color32::GREEN, "Source");
                dot(ui, egui::Color32::RED, "Sink");
                dot(ui, egui::Color32::LIGHT_BLUE, "Visited");
                dot(ui, egui::Color32::LIGHT_RED, "Unvisited");
                dot(ui, egui::Color32::YELLOW, "Landmark");

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);

                // Count vertices that have been visited (color != LIGHT_RED).
            });

        // ── Main canvas ───────────────────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();

            if self.vertex_pos.is_empty()
                || self.resize_countdown == RESIZE_COUNTDOWN_THRESHOLD
            {
                self.resize_countdown = 0;
                self.precompute(size);
            } else if size != self.last_size {
                self.resize_countdown = 1;
            } else if self.resize_countdown != 0 {
                self.resize_countdown += 1;
            }
            self.last_size = size;

            let (rect, response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::click());

            // vertex_pos is in [0..size] local space; offset by rect.min for screen coords.
            let offset = rect.min.to_vec2();

            // Map-click handler for source/sink picking.
            if response.clicked() && self.assigning != Assigning::None {
                if let Some(click_pos) = response.interact_pointer_pos() {
                    // Convert screen click back to local space before hit-testing.
                    let click_pos = click_pos - offset;
                    if let Some(idx) = self.vertex_at(click_pos) {
                        match self.assigning {
                            Assigning::Source => {
                                self.source = idx as u32;
                                self.source_input = idx.to_string();
                            }
                            Assigning::Sink => {
                                self.sink = idx as u32;
                                self.sink_input = idx.to_string();
                            }
                            Assigning::None => {}
                        }
                        self.assigning = Assigning::None;
                    }
                }
            }

            let painter = ui.painter_at(rect);

            const LARGE_GRAPH_THRESHOLD: usize = 30_000;
            let stride = if self.graph.size > LARGE_GRAPH_THRESHOLD {
                (self.graph.size / LARGE_GRAPH_THRESHOLD).max(1)
            } else {
                1
            };

            for (i, &pos) in self.vertex_pos.iter().enumerate() {
                let is_source = i == self.source as usize;
                let is_sink   = i == self.sink   as usize;

                let cur_color = if self.show_regions {
                    self.graph.regions.as_ref()
                        .and_then(|r| r.get(i as u32))
                        .map(|&rid| region_color(rid))
                        .unwrap_or(egui::Color32::LIGHT_RED)
                } else {
                    self.color_snapshot.get(i).copied().unwrap_or(egui::Color32::LIGHT_RED)
                };

                // Skip uninteresting vertices to stay within the threshold budget.
                if !is_source && !is_sink && i % stride != 0 {
                    continue;
                }

                let screen_pos = pos + offset;

                if is_source || is_sink {
                    let color = if is_source { egui::Color32::GREEN } else { egui::Color32::RED };
                    painter.circle_stroke(screen_pos, 9.0, egui::Stroke::new(2.0, color));
                    painter.circle_filled(screen_pos, 5.0, color);
                } else {
                    let r = if self.big_vertices.contains(&(i as u32)) { 5.0 } else { 1.5 };
                    painter.circle_filled(screen_pos, r, cur_color);
                }
            }
            if self.show_landmarks {
                for &lm_id in self.graph.landmarks.keys() {
                    let idx = lm_id as usize;
                    if let Some(&pos) = self.vertex_pos.get(idx) {
                        let screen_pos = pos + offset;
                        painter.circle_filled(screen_pos, 8.0, egui::Color32::YELLOW);
                        painter.circle_stroke(screen_pos, 8.0, egui::Stroke::new(2.0, egui::Color32::GOLD));
                    }
                }
            }
                  
        });

        ctx.request_repaint();
    }
}


