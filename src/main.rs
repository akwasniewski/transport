mod algo;
mod graph;
mod vis;

use crate::{
    algo::{
        astar::{astar, heuristics::earth_dist},
        bidirectional_dijkstra::bidirectional_dijkstra,
        dijkstra::dijkstra,
    },
    graph::Graph,
    vis::visualize_algorithm,
};

fn main() {
    let graph = Graph::from_files("graphs/krakow_snap.txt", "graphs/krakow_coords.txt");

    // visualize_algorithm(graph, 0, 6000, |g, f, t, a| astar(g, f, t, a, earth_dist));
    visualize_algorithm(graph, 0, 6000, bidirectional_dijkstra)
}
