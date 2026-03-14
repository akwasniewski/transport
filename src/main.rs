mod algo;
mod graph;
mod graph_app;
use eframe::egui;

use crate::{
    algo::{a_star_2d, dijsktra},
    graph::Graph,
    graph_app::GraphApp,
};
use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
};
fn main() {
    let snap_data = fs::read_to_string("graphs/krakow_snap.txt").expect("Failed to read SNAP file");
    let coords_data =
        fs::read_to_string("graphs/krakow_coords.txt").expect("Failed to read coords file");

    let mut graph = Graph::from_snap(&snap_data);
    graph.add_coords(&coords_data);

    let res = dijsktra(&graph, 9, 100).unwrap();

    println!("some Dijkstra {res}");

    let res = a_star_2d(&graph, 9, 100).unwrap();

    println!("some a* {res}");

    let graph_arc = Arc::new(Mutex::new(graph));

    let anim_graph = graph_arc.clone();
    thread::spawn(move || {
        let mut idx = 0;
        loop {
            {
                let mut g = anim_graph.lock().unwrap();
                let n = g.vertices.len();
                g.vertices[idx % n].color = egui::Color32::LIGHT_BLUE;
            }

            idx += 1;
            thread::sleep(std::time::Duration::from_millis(10));
        }
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
