use eframe::egui;
use std::sync::{Arc, Mutex};

use crate::graph::Graph;

pub struct GraphApp {
    pub graph: Arc<Graph>,
    vertex_pos: Vec<egui::Pos2>,               // precomputed positions
    edge_cache: Vec<(egui::Pos2, egui::Pos2)>, // precomputed edges
    last_size: egui::Vec2,
    resize_countdown: u8,
}

const RESIZE_COUNTDOWN_THRESHOLD: u8 = 4;

impl GraphApp {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self {
            graph,
            vertex_pos: Vec::new(),
            edge_cache: Vec::new(),
            last_size: egui::Vec2::ZERO,
            resize_countdown: 0,
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

        // compute vertex positions
        self.vertex_pos = self
            .graph
            .vertices
            .iter()
            .map(|v| to_screen(v.coords.0, v.coords.1))
            .collect();

        // compute edges
        self.edge_cache.clear();
        for vertex in &self.graph.vertices {
            for (target_label, _) in &vertex.connections {
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
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let size = ctx.available_rect().size();
        // precompute positions if empty
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

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            // draw edges (cached)
            for (p1, p2) in &self.edge_cache {
                painter.line_segment(
                    [*p1, *p2],
                    egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
                );
            }

            // draw vertices using precomputed positions
            for (i, pos) in self.vertex_pos.iter().enumerate() {
                let cur_color = self.graph.vertices[i].color.lock().unwrap();
                painter.circle_filled(*pos, 1.5, *cur_color);
            }
        });

        ctx.request_repaint(); // continue animation
    }
}
