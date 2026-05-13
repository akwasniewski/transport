mod algo;
mod graph;
mod vis;
mod graph_building;
mod utility;
mod benchmark;
use crate::{benchmark::{benchmark, preprocess_dijkstra}, vis::visualize};


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
    let graph_prefix = "poland_s";
    // preprocess_dijkstra(graph_prefix);
    benchmark(graph_prefix, 0, 6000);
    visualize(graph_prefix);
}
