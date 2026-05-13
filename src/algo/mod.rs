use crate::{algo::{algo_result::AlgoResult, alt::landmarks::alt_potential, astar::{bidirectional::bidirectional, heuristics::{dijkstra_potential, earth_dist, middle_dist}, unidirectional::unidirectional}}, graph::Graph};


pub mod algo_result;
pub mod alt;
pub mod astar;
pub mod dijkstra;
mod utils;
pub mod arc_flags;

pub fn dijkstra(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
    unidirectional(graph, from, to, dijkstra_potential, false)
}

pub fn dijkstra_arc_flags(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
    unidirectional(graph, from, to, dijkstra_potential, true)
}

pub fn dijkstra_bidirectional(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
    bidirectional(graph, from, to, false, dijkstra_potential, dijkstra_potential)
}
pub fn astar(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
     unidirectional(graph, from, to, earth_dist, false)
}

pub fn astar_arc_flags(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
     unidirectional(graph, from, to, earth_dist, true)
}

pub fn astar_bidirectional(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
    let heura = middle_dist(earth_dist);
    bidirectional(graph, from, to, false, heura.0, heura.1)
}

pub fn astar_bidirectional_arc_flags(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
    let heura = middle_dist(earth_dist);
    bidirectional(graph, from, to, true, heura.0, heura.1)
}

pub fn alt(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
    unidirectional(graph, from, to, alt_potential, false)
}


pub fn alt_arc_flags(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
    unidirectional(graph, from, to, alt_potential, true)
}

