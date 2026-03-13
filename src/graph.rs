use std::collections::{HashMap};

use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct Vertex{
    pub (crate) label: usize,
    pub (crate) connections: HashMap<usize, OrderedFloat<f32>>,
}

impl Vertex{
    pub fn new(label:usize) -> Self{
        let connections: HashMap<usize, OrderedFloat<f32>>=HashMap::new();
        Self{
            label,
            connections,
        }
    }
}

#[derive(Debug)]
pub struct Graph{
    pub size: usize,
    pub vertices: Vec<Vertex>,
}

impl Graph{
        pub fn new(size: usize) -> Self{
        let mut vertices=Vec::new();
        for i in 0..size{
            vertices.push(Vertex::new(i));
        }
        Self{
            size,
            vertices
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, travel_time: OrderedFloat<f32>){
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
            let travel_time: OrderedFloat<f32> = parts[2]
                .parse::<f32>()
                .map(OrderedFloat)
                .expect("Failed to parse travel_time");

            res.add_edge(u, v, travel_time);
        }

        res
    }
}