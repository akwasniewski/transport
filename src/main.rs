mod algo;
mod graph;
mod vis;

use std::sync::Arc;

use crate::{
    algo::{
        astar::{
            astar,
            bidirectional::bidirectional_astar,
            heuristics::{earth_dist, middle_dist, rev},
        },
        bidirectional_dijkstra::bidirectional_dijkstra,
        dijkstra::dijkstra,
    },
    graph::Graph,
    vis::visualize_algorithm,
};

fn main() {
    let graph = Graph::from_files("graphs/krakow_snap.txt", "graphs/krakow_coords.txt");
    let graph_arc = Arc::new(graph);
    println!("Dijkstra: {}", dijkstra(graph_arc.clone(), 0, 6000, false));
    println!(
        "Astar: {}",
        astar(graph_arc.clone(), 0, 6000, false, earth_dist)
    );
    println!(
        "Dijkstra: {}",
        bidirectional_dijkstra(graph_arc.clone(), 0, 6000, false)
    );
    println!(
        "Bidirectional astar: {}",
        bidirectional_astar(
            graph_arc.clone(),
            0,
            6000,
            false,
            earth_dist,
            rev(earth_dist)
        )
    );
    let heura = middle_dist(earth_dist);
    println!(
        "Bidirectional astar middle: {}",
        bidirectional_astar(graph_arc.clone(), 0, 6000, false, heura.0, heura.1)
    );

    visualize_algorithm(graph_arc, 0, 6000);
}
