mod algo;
mod graph;
mod vis;

use std::sync::Arc;

use crate::{
    algo::{
        astar::{
            astar,
            bidirectional::bidirectional_astar,
            heuristics::{earth_dist, rev},
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

    // visualize_algorithm(graph_arc, 0, 6000, dijkstra);
    // visualize_algorithm(graph_arc, 0, 6000, |g, f, t, a| {
    //     astar(g, f, t, a, earth_dist)
    // });
    // visualize_algorithm(graph_arc, 0, 6000, bidirectional_dijkstra)
    visualize_algorithm(graph_arc, 0, 6000, |g, f, t, a| {
        bidirectional_astar(g, f, t, a, earth_dist, rev(earth_dist))
    });
}
