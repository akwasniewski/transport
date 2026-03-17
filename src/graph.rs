use eframe::egui;
use ordered_float::OrderedFloat;
use std::{collections::HashMap, fs, sync::Mutex};
#[derive(Debug)]
pub struct Vertex {
    pub(crate) label: usize,
    pub(crate) connections: HashMap<usize, OrderedFloat<f64>>,
    pub(crate) coords: (f64, f64),
    pub(crate) color: Mutex<egui::Color32>,
}
impl Vertex {
    pub fn new(label: usize) -> Self {
        let connections: HashMap<usize, OrderedFloat<f64>> = HashMap::new();
        Self {
            label,
            connections,
            coords: (0.0, 0.0),
            color: Mutex::new(egui::Color32::LIGHT_RED),
        }
    }
    pub fn set_coords(&mut self, lat: f64, lon: f64) {
        self.coords = (lat, lon);
    }
    pub fn recolor(&self, new_color: egui::Color32) {
        let mut color = self.color.lock().unwrap();
        *color = new_color;
    }
}
#[derive(Debug)]
pub struct Graph {
    pub size: usize,
    pub vertices: Vec<Vertex>,
}
impl Graph {
    pub fn new(size: usize) -> Self {
        let mut vertices = Vec::new();
        for i in 0..size {
            vertices.push(Vertex::new(i));
        }
        Self { size, vertices }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, travel_time: OrderedFloat<f64>) {
        self.vertices[from].connections.insert(to, travel_time);
    }
    pub fn from_snap(snap: &str) -> Self {
        let mut res = Graph::new(9765);
        for line in snap.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                panic!("Expected 3 values per line, got {}", parts.len());
            }
            let u: usize = parts[0].parse().expect("Failed to parse u");
            let v: usize = parts[1].parse().expect("Failed to parse v");
            let length: OrderedFloat<f64> = parts[2]
                .parse::<f64>()
                .map(OrderedFloat)
                .expect("Failed to parse length");
            res.add_edge(u, v, length);
        }
        res
    }
    pub fn add_coords(&mut self, input: &str) {
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                panic!("Expected 3 values per line, got {}", parts.len());
            }
            let id: usize = parts[0].parse().expect("Failed to parse vertex id");
            let lat: f64 = parts[1].parse().expect("Failed to parse latitude");
            let lon: f64 = parts[2].parse().expect("Failed to parse longitude");
            self.vertices[id].set_coords(lat, lon);
        }
    }
    pub fn from_files(snap_path: &str, coords_path: &str) -> Graph {
        let snap_data = fs::read_to_string(snap_path)
            .unwrap_or_else(|_| panic!("Failed to read SNAP file: {snap_path}"));
        let coords_data = fs::read_to_string(coords_path)
            .unwrap_or_else(|_| panic!("Failed to read coords file: {coords_path}"));

        let mut graph = Graph::from_snap(&snap_data);
        graph.add_coords(&coords_data);
        graph
    }
}
