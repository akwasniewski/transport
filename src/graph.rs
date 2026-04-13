use eframe::egui;
use ordered_float::OrderedFloat;
use std::{
    collections::HashMap,
    fs,
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(Debug)]
pub struct LandmarkData {
    pub to: Vec<OrderedFloat<f64>>,
    pub from: Vec<OrderedFloat<f64>>,
}

#[derive(Debug)]
pub struct Vertex {
    pub(crate) label: usize,
    pub(crate) edges: HashMap<usize, OrderedFloat<f64>>,
    pub(crate) edges_rev: HashMap<usize, OrderedFloat<f64>>,
    pub(crate) coords: (f64, f64),
    pub(crate) color: AtomicU32,
}

impl Vertex {
    pub fn new(label: usize) -> Self {
        let init_color = u32::from_be_bytes(egui::Color32::LIGHT_RED.to_array());
        Self {
            label,
            edges: HashMap::new(),
            edges_rev: HashMap::new(),
            coords: (0.0, 0.0),
            color: AtomicU32::new(init_color),
        }
    }
    pub fn set_coords(&mut self, lat: f64, lon: f64) {
        self.coords = (lat, lon);
    }
    pub fn recolor(&self, new_color: egui::Color32) {
        self.color
            .store(u32::from_be_bytes(new_color.to_array()), Ordering::Relaxed);
    }
}

#[derive(Debug)]
pub struct Graph {
    pub size: usize,
    pub vertices: Vec<Vertex>,
    pub(crate) landmarks: HashMap<usize, LandmarkData>,
    pub(crate) regions: Option<Vec<usize>>,
    pub(crate) edge_region_flags: Option<Vec<HashMap<usize, Vec<bool>>>>,
    pub(crate) edge_region_flags_rev: Option<Vec<HashMap<usize, Vec<bool>>>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        let mut vertices = Vec::new();
        for i in 0..size {
            vertices.push(Vertex::new(i));
        }
        Self {
            size,
            vertices,
            landmarks: HashMap::new(),
            regions: None,
            edge_region_flags: None,
            edge_region_flags_rev: None
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, travel_time: OrderedFloat<f64>) {
        self.vertices[from].edges.insert(to, travel_time);
    }
    pub fn add_reverse_edges(&mut self, from: usize, to: usize, travel_time: OrderedFloat<f64>) {
        self.vertices[from].edges_rev.insert(to, travel_time);
    }
    pub fn from_snap(snap: &str) -> Self {
        let mut lines = snap.lines();

        let n: usize = lines
            .next()
            .expect("snap file is empty")
            .trim()
            .parse()
            .expect("Failed to parse vertex count");

        let mut res = Graph::new(n);

        for line in lines {
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
            res.add_reverse_edges(v, u, length)
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
            let lat: f64 = parts[1].parse::<f64>().expect("Failed to parse latitude");
            let lon: f64 = parts[2].parse::<f64>().expect("Failed to parse longitude");
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
