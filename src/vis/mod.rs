mod vis_app;
use eframe::egui::Color32;

use crate::{graph::Graph, vis::vis_app::run_gui};
use std::{collections::HashSet, sync::Arc, thread};

pub fn visualize_algorithm<F, R>(graph: Graph, from: usize, to: usize, algorithm: F)
where
    F: FnOnce(Arc<Graph>, usize, usize, bool) -> R + Send + 'static,
    R: std::fmt::Debug + Send + 'static,
{
    let graph_arc = Arc::new(graph);
    let anim_graph = graph_arc.clone();
    anim_graph.vertices[from].recolor(Color32::YELLOW);
    anim_graph.vertices[to].recolor(Color32::YELLOW);
    thread::spawn(move || {
        let res = algorithm(anim_graph, from, to, true);
        println!("res: {:?}", res);
    });

    let big_vertices: HashSet<usize> = [from, to].into_iter().collect();
    run_gui(graph_arc, big_vertices);
}
