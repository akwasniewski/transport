mod vis_app;

use crate::{graph::Graph, vis::vis_app::VisApp};
use std::{collections::HashSet, sync::Arc};

pub fn visualize(graph_prefix: &str){
    let snap_path = format!("graphs/{}_snap.txt", graph_prefix);
    let coords_path = format!("graphs/{}_coords.txt", graph_prefix);
    let flags_path = format!("graphs/{}_flags.bin", graph_prefix);
    let farthest_landmarks_path = format!("graphs/{}_landmarks_farthest.bin", graph_prefix);
    let mut graph = Graph::from_files(&snap_path, &coords_path);
    graph.load_landmarks(&farthest_landmarks_path);
    let _ = graph.load_edge_region_cache(flags_path);
    visualize_graph(graph, 0, 1);
}
fn visualize_graph(graph: Graph, source: u32, sink: u32) {
    let options = eframe::NativeOptions::default();
    let graph = Arc::new(graph);
    eframe::run_native(
        "Graph Visualization",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(VisApp::new(
                graph.clone(),
                HashSet::new(),
                source,
                sink,
            )))
        }),
    )
    .expect("Failed to start GUI");
}
