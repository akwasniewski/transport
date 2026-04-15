mod vis_app;

use crate::{graph::Graph, vis::vis_app::VisApp};
use std::{collections::HashSet, sync::Arc};

pub fn visualize_algorithm(graph: Graph, source: u32, sink: u32) {
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
