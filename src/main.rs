mod algo;
mod graph;
mod vis;
mod graph_building;
mod utility;
use crate::{
    algo::{
        alt::landmarks::alt_potential, arc_flags::{arc_flags_astar::arc_flags_astar, bidirecional::bidirectional_arcflags}, astar::{
            astar,
            bidirectional::bidirectional_astar,
            heuristics::{earth_dist, middle_dist, rev},
        }, dijkstra::{bidirectional::bidirectional_dijkstra, dijkstra}
    },
    graph::Graph, vis::visualize_algorithm,
};

use graph_building::{parse_osm, ParseConfig};

// fn main() {
//     let result = parse_osm(
//         "poland.osm.pbf",   // input PBF file
//         "graphs/",                  // output directory (created if missing)
//         "poland_s",                   // name prefix for output files
//         ParseConfig::default(),     // largest_component_only: true
//     ).expect("parsing failed");
//
//     println!("nodes: {}, edges: {}", result.node_count, result.edge_count);
// }
fn main() {
    let mut graph = Graph::from_files("graphs/poland_s_snap.txt", "graphs/poland_s_coords.txt");
    println!("Dijkstra: {}", dijkstra(&graph, 0, 6000, false));
    println!("Astar: {}", astar(&graph, 0, 6000, false, earth_dist));

    // graph.get_random_landmarks(16);
    // println!("Alt random: {}", astar(&graph, 0, 6000, false, alt_potential));

    graph.get_farthest_landmarks(32);
    println!("Alt farthest: {}", astar(&graph, 0, 6000, false, alt_potential));

    println!(
        "Bidirectional Dijkstra: {}",
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

    graph.divide_into_regions_dijkstra(64);
    graph.preprocess_region_edges(64, utility::EdgeDir::Forward);
    graph.preprocess_region_edges(64, utility::EdgeDir::Reverse);
    
    // let _ = graph.load_edge_region_cache("graphs/edge_region_cache.bin");
    // let _ = graph.save_edge_region_cache("graphs/edge_region_cache_farthest.bin");
    println!("Arc flags: {}", arc_flags_astar(&graph, 0, 6000, false, earth_dist));
    println!("Arc flags alt: {}", arc_flags_astar(&graph, 0, 6000, false, alt_potential));
    println!("Bidirectional Arc flags: {}", bidirectional_arcflags(&graph, 0, 6000, false, earth_dist, rev(earth_dist)));
    visualize_algorithm(graph, 0, 6000);
}
