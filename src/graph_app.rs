use eframe::egui;
use std::sync::{Arc, Mutex};

use crate::graph::Graph;

pub struct GraphApp {
    pub graph: Arc<Mutex<Graph>>,
    pub tick: usize,
    vertex_pos: Vec<egui::Pos2>,               // precomputed positions
    edge_cache: Vec<(egui::Pos2, egui::Pos2)>, // precomputed edges
}

impl GraphApp {
    pub fn new(graph: Arc<Mutex<Graph>>) -> Self {
        Self {
            graph,
            tick: 0,
            vertex_pos: Vec::new(),
            edge_cache: Vec::new(),
        }
    }

    fn precompute(&mut self, size: egui::Vec2) {
        let graph = self.graph.lock().unwrap();
        if graph.vertices.is_empty() {
            return;
        }

        let min_lat = graph
            .vertices
            .iter()
            .map(|v| v.coords.0)
            .fold(f64::INFINITY, f64::min);
        let max_lat = graph
            .vertices
            .iter()
            .map(|v| v.coords.0)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_lon = graph
            .vertices
            .iter()
            .map(|v| v.coords.1)
            .fold(f64::INFINITY, f64::min);
        let max_lon = graph
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

        // compute vertex positions
        self.vertex_pos = graph
            .vertices
            .iter()
            .map(|v| to_screen(v.coords.0, v.coords.1))
            .collect();

        // compute edges
        self.edge_cache.clear();
        for vertex in &graph.vertices {
            for (target_label, _) in &vertex.connections {
                if let Some(target) = graph.vertices.iter().find(|v| v.label == *target_label) {
                    let p1 = to_screen(vertex.coords.0, vertex.coords.1);
                    let p2 = to_screen(target.coords.0, target.coords.1);
                    self.edge_cache.push((p1, p2));
                }
            }
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let size = ctx.available_rect().size();

        // precompute positions if empty
        if self.vertex_pos.is_empty() || self.edge_cache.is_empty() {
            self.precompute(size);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let graph = self.graph.lock().unwrap();

            // draw edges (cached)
            for (p1, p2) in &self.edge_cache {
                painter.line_segment(
                    [*p1, *p2],
                    egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
                );
            }

            // draw vertices using precomputed positions
            for (i, pos) in self.vertex_pos.iter().enumerate() {
                painter.circle_filled(*pos, 3.0, graph.vertices[i].color);
            }
        });

        ctx.request_repaint(); // continue animation
    }
}
