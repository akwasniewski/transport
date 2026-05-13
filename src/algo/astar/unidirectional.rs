use crate::{
    algo::{algo_result::AlgoResult, utils::QueueItem},
    graph::Graph, index_vec,
};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, time::Instant};
use crate::utility::IndexVec;

pub fn unidirectional(
    graph: &Graph,
    from: u32,
    to: u32,
    potential: fn(&Graph, u32, u32, u32) -> f32,
    use_arc_flags: bool
) -> AlgoResult {
    let mut dist: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); graph.size];
    let start = Instant::now();
    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem::with_priority(
        from,
        OrderedFloat(0.0 + potential(graph, from, from, to)),
        OrderedFloat(0.0),
    ));

    let mut visited_nodes = 0;

    while !que.is_empty() {
        let cur = que.pop().unwrap();

        if cur.distance > dist[cur.vertex] {
            continue;
        }

        graph[cur.vertex].recolor(Color32::LIGHT_BLUE);

        visited_nodes += 1;

        if cur.vertex == to {
            return AlgoResult::ok(dist[to].0,visited_nodes, start.elapsed());
        }

        for (edge_idx, e) in graph[cur.vertex].edges.iter() {
            match (use_arc_flags, &graph.edge_region_flags, &graph.regions){
                (true, Some(edge_region_flags), Some(regions)) =>{
                    if !edge_region_flags[cur.vertex][edge_idx][regions[to] as usize] 
                        && regions[cur.vertex] != regions[from] 
                        && regions[cur.vertex] != regions[to]{
                        continue;
                    }
                },
                (true, _ , _) => {
                    return AlgoResult::err("Ran algorithm with use arc flags when arc flags not set")
                }
                _ => {}
            }

            let alt_cost = e.length + dist[cur.vertex].0;
            if alt_cost < dist[e.to] {
                que.push(QueueItem {
                    vertex: e.to,
                    priority: alt_cost + potential(graph, e.to, from, to),
                    distance: alt_cost,
                });
                dist[e.to] = alt_cost;
            }
        }
    }
    AlgoResult::err("Path not found")
}
