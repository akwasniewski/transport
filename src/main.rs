mod algo;
mod graph;
mod graph_app;

use crate::{
    algo::astar::{astar, heuristics::earth_dist},
    graph::Graph,
    graph_app::GraphApp,
};
use std::{fs, sync::Arc, thread};

fn main() {
    let snap_data = fs::read_to_string("graphs/krakow_snap.txt").expect("Failed to read SNAP file");
    let coords_data =
        fs::read_to_string("graphs/krakow_coords.txt").expect("Failed to read coords file");

    let mut graph = Graph::from_snap(&snap_data);
    graph.add_coords(&coords_data);

    let graph_arc = Arc::new(graph);

    let anim_graph = graph_arc.clone();
    thread::spawn(move || {
        let res = astar(anim_graph, 0, 6000, true, earth_dist);
        println!("res: {:?}", res)
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Graph Visualization",
        options,
        Box::new({
            let graph = graph_arc.clone();
            move |_cc| Ok(Box::new(GraphApp::new(graph)))
        }),
    )
    .expect("Failed to start GUI");
}
