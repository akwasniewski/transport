mod vis_app;
use eframe::egui::Color32;

use crate::{algo::algo_result::AlgoResult, graph::Graph, vis::vis_app::run_gui};
use std::{collections::HashSet, sync::Arc, thread};

pub fn visualize_algorithm<F>(graph: Arc<Graph>, from: usize, to: usize, algorithm: F)
where
    F: FnOnce(Arc<Graph>, usize, usize, bool) -> AlgoResult + Send + 'static,
{
    graph.vertices[from].recolor(Color32::YELLOW);
    graph.vertices[to].recolor(Color32::YELLOW);

    let anim_graph = graph.clone();
    thread::spawn(move || {
        algorithm(anim_graph, from, to, true);
    });

    let big_vertices: HashSet<usize> = [from, to].into_iter().collect();
    run_gui(graph, big_vertices);
}
