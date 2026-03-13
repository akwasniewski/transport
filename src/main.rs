use std::collections::{HashSet};
use std::fs;

pub struct Vertex{
    pub (crate) label: usize,
    pub (crate) connections: HashSet<usize>,
}

impl Vertex{
    pub fn new(label:usize) -> Self{
        let connections: HashSet<usize>=HashSet::new();
        Self{
            label,
            connections,
        }
    }
}

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
    pub fn add_edge(&mut self, from: usize, to: usize){
        self.vertices[from].connections.insert(to);
    }
    pub fn from_snap(snap: &str)->Self{
        let split = snap.split_whitespace();
        let collected: Vec<usize> = split.map(|x| x.parse().unwrap()).collect();
        let mut res: Graph = Graph::new(9765);
        for i in (0..collected.len()).step_by(2) {
            res.add_edge(collected[i], collected[i + 1]);
        }
        res
    }
}


fn main() {
    let snap_data = fs::read_to_string("graphs/krakow_snap.txt")
        .expect("Failed to read SNAP file");

    let graph = Graph::from_snap(&snap_data);

    println!("Graph loaded!");
}
