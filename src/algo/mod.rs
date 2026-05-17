use crate::{algo::{algo_result::AlgoResult, alt::landmarks::alt_potential, astar::{bidirectional::bidirectional, heuristics::{dijkstra_potential, earth_dist, middle_dist, rev}, unidirectional::unidirectional}}, graph::Graph};


pub mod algo_result;
pub mod alt;
pub mod astar;
pub mod dijkstra;
mod utils;
pub mod arc_flags;
pub mod contraction;
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
    bidirectional(graph, from, to, dijkstra_potential, dijkstra_potential, false, false)
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
    bidirectional(graph, from, to, earth_dist, rev(earth_dist), false, false)
}

pub fn astar_bidirectional_arc_flags(
    graph: &Graph,
    from: u32,
    to: u32,
)->AlgoResult{
    bidirectional(graph, from, to, earth_dist, rev(earth_dist), true, false)
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

pub fn contraction_hierarchies(
    graph: &Graph,
    from: u32,
    to: u32,
) -> AlgoResult{
    bidirectional(graph, from, to, dijkstra_potential, dijkstra_potential, false, true)
}

pub fn astar_contraction_hierarchies(
    graph: &Graph,
    from: u32,
    to: u32,
) -> AlgoResult{
    bidirectional(graph, from, to, earth_dist, rev(earth_dist), false, true)
}
