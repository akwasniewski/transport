use eframe::egui;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex, atomic::Ordering},
    thread,
};

use crate::{
    algo::{
        alt::landmarks::alt_potential,
        astar::{
            astar,
            bidirectional::bidirectional_astar,
            heuristics::{earth_dist, middle_dist, rev},
        },
        dijkstra::{bidirectional::bidirectional_dijkstra, dijkstra},
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
}

impl AlgoChoice {
    const ALL: &'static [AlgoChoice] = &[
        AlgoChoice::Dijkstra,
        AlgoChoice::Astar,
        AlgoChoice::Alt,
        AlgoChoice::BidirectionalDijkstra,
        AlgoChoice::BidirectionalAstar,
        AlgoChoice::BidirectionalAstarMiddle,
    ];

    fn label(self) -> &'static str {
        match self {
            AlgoChoice::Dijkstra => "Dijkstra",
            AlgoChoice::Astar => "A*",
            AlgoChoice::Alt => "Random Alt",
            AlgoChoice::BidirectionalDijkstra => "Bidirectional Dijkstra",
            AlgoChoice::BidirectionalAstar => "Bidirectional A*",
            AlgoChoice::BidirectionalAstarMiddle => "Bidirectional A* (middle)",
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

pub struct VisApp {
    pub graph: Arc<Graph>,
    vertex_pos: Vec<egui::Pos2>,
    edge_cache: Vec<(egui::Pos2, egui::Pos2)>,
    last_size: egui::Vec2,
    resize_countdown: u8,
    big_vertices: HashSet<usize>,

    // user-controlled state
    source: usize,
    sink: usize,
    algo: AlgoChoice,
    source_input: String,
    sink_input: String,

    // click-assignment mode
    assigning: Assigning,

    // result
    result: Arc<Mutex<Option<f64>>>,
    running: bool,

    color_snapshot: Vec<egui::Color32>,
}

impl VisApp {
    pub fn new(
        graph: Arc<Graph>,
        big_vertices: HashSet<usize>,
        source: usize,
        sink: usize,
    ) -> Self {
        let color_snapshot = vec![egui::Color32::LIGHT_RED; graph.size];
        Self {
            graph,
            vertex_pos: Vec::new(),
            edge_cache: Vec::new(),
            last_size: egui::Vec2::ZERO,
            resize_countdown: 0,
            big_vertices,
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

        let min_lat = self
            .graph
            .vertices
            .iter()
            .map(|v| v.coords.0)
            .fold(f64::INFINITY, f64::min);
        let max_lat = self
            .graph
            .vertices
            .iter()
            .map(|v| v.coords.0)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_lon = self
            .graph
            .vertices
            .iter()
            .map(|v| v.coords.1)
            .fold(f64::INFINITY, f64::min);
        let max_lon = self
            .graph
            .vertices
            .iter()
            .map(|v| v.coords.1)
            .fold(f64::NEG_INFINITY, f64::max);
        let lat_range = max_lat - min_lat;
        let lon_range = max_lon - min_lon;

        let to_screen = |lat: f64, lon: f64| -> egui::Pos2 {
            let x = (lon - min_lon) / lon_range;
            let y = (lat - min_lat) / lat_range;
            egui::pos2((x as f32) * size.x, size.y - (y as f32) * size.y)
        };

        self.vertex_pos = self
            .graph
            .vertices
            .iter()
            .map(|v| to_screen(v.coords.0, v.coords.1))
            .collect();

        self.edge_cache.clear();
        for vertex in &self.graph.vertices {
            for (target_label, _) in &vertex.edges {
                if let Some(target) = self
                    .graph
                    .vertices
                    .iter()
                    .find(|v| v.label == *target_label)
                {
                    let p1 = to_screen(vertex.coords.0, vertex.coords.1);
                    let p2 = to_screen(target.coords.0, target.coords.1);
                    self.edge_cache.push((p1, p2));
                }
            }
        }
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
                                self.source = v;
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
                                self.sink = v;
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
                dot(ui, egui::Color32::GREEN, "Source");
                dot(ui, egui::Color32::RED, "Sink");
                dot(ui, egui::Color32::LIGHT_BLUE, "Visited");
                dot(ui, egui::Color32::LIGHT_RED, "Unvisited");

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);

                // Count vertices that have been visited (color != LIGHT_RED).
            });

        // ── Main canvas ───────────────────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();

            if self.vertex_pos.is_empty()
                || self.edge_cache.is_empty()
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
                                self.source = idx;
                                self.source_input = idx.to_string();
                            }
                            Assigning::Sink => {
                                self.sink = idx;
                                self.sink_input = idx.to_string();
                            }
                            Assigning::None => {}
                        }
                        self.assigning = Assigning::None;
                    }
                }
            }

            let painter = ui.painter_at(rect);

            // Draw edges — shift from local into screen space.
            for (p1, p2) in &self.edge_cache {
                painter.line_segment(
                    [*p1 + offset, *p2 + offset],
                    egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
                );
            }

            // Draw vertices — shift from local into screen space.
            for (i, pos) in self.vertex_pos.iter().enumerate() {
                let screen_pos = *pos + offset;
                let is_source = i == self.source;
                let is_sink = i == self.sink;
                let is_big = self.big_vertices.contains(&i);
                let base_r = if is_big { 5.0_f32 } else { 1.5_f32 };

                if is_source || is_sink {
                    let color = if is_source {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    };
                    let r = base_r.max(5.0);
                    painter.circle_stroke(screen_pos, r + 4.0, egui::Stroke::new(2.0, color));
                    painter.circle_filled(screen_pos, r, color);
                } else {
                    let cur_color = self
                        .color_snapshot
                        .get(i)
                        .copied()
                        .unwrap_or(egui::Color32::LIGHT_RED);
                    painter.circle_filled(screen_pos, base_r, cur_color);
                }
            }
        });

        ctx.request_repaint();
    }
}
