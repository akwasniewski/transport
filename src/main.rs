mod graph;
mod algo;
use std::fs;
use crate::{algo::dijsktra, graph::Graph};
fn main(){
    let snap_data = fs::read_to_string("graphs/krakow_snap.txt")
        .expect("Failed to read SNAP file");

    let graph = Graph::from_snap(&snap_data);

    println!("Graph loaded! {:?}", graph.vertices);

    let res =dijsktra(&graph, 9, 9756).unwrap();

    println!("some Dijkstra {res}")
}
