mod algo;
mod graph;
mod vis;
mod graph_building;
mod utility;
mod benchmark;
mod preprocess;
use crate::{benchmark::benchmark, graph_building::{ParseConfig, parse_osm}, preprocess::{preprocess_contraction, preprocess_flags, preprocess_landmarks}, vis::visualize};


// fn main() {
//     let result = parse_osm(
//         "europe.osm.pbf",   // input PBF file
//         "graphs/",                  // output directory (created if missing)
//         "europe",                   // name prefix for output files
//         ParseConfig::default(),     // largest_component_only: true
//     ).expect("parsing failed");
//
//     println!("nodes: {}, edges: {}", result.node_count, result.edge_count);
// }
fn main() {
    let graph_prefix = "europe";
    // preprocess_landmarks(graph_prefix);
    // preprocess_flags(graph_prefix);
    // preprocess_contraction(graph_prefix);
    benchmark(graph_prefix, 0, 6020);
    // visualize(graph_prefix);
}
