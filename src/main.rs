mod graph;
mod algo;
use std::fs;
use crate::{algo::{a_star_2d, dijsktra}, graph::Graph};
fn main(){
    let snap_data = fs::read_to_string("graphs/krakow_snap.txt")
        .expect("Failed to read SNAP file");
    let coords_data = fs::read_to_string("graphs/krakow_snap.txt")
        .expect("Failed to read coords file");
    
    let mut graph = Graph::from_snap(&snap_data);
    graph.add_coords(&coords_data);

    let res = dijsktra(&graph, 9, 9756).unwrap();

    println!("some Dijkstra {res}");


    let res = a_star_2d(&graph, 9, 9756).unwrap();

    println!("some a* {res}")
}
