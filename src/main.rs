mod algo;
mod graph;
mod vis;

use crate::{
    algo::{
        alt::landmarks::alt_potential,
        astar::{
            astar,
            bidirectional::bidirectional_astar,
            heuristics::{earth_dist, middle_dist, rev},
        },
        dijkstra::{bidirectional::bidirectional_dijkstra, dijkstra},
    },
    graph::Graph,
    vis::visualize_algorithm,
};

fn main() {
    let mut graph = Graph::from_files("graphs/krakow_snap.txt", "graphs/krakow_coords.txt");
    println!("Dijkstra: {}", dijkstra(&graph, 0, 6000, false));
    println!("Astar: {}", astar(&graph, 0, 6000, false, earth_dist));

    graph.get_random_landmarks(16);

    println!("Alt: {}", astar(&graph, 0, 6000, false, alt_potential));
    println!(
        "Dijkstra: {}",
        bidirectional_dijkstra(&graph, 0, 6000, false)
    );
    println!(
        "Bidirectional astar: {}",
        bidirectional_astar(&graph, 0, 6000, false, earth_dist, rev(earth_dist))
    );
    let heura = middle_dist(earth_dist);
    println!(
        "Bidirectional astar middle: {}",
        bidirectional_astar(&graph, 0, 6000, false, heura.0, heura.1)
    );

    graph.divide_into_regions(4);
    visualize_algorithm(graph, 0, 6000);

    // println!{"{:?}",graph.regions}
}
