use crate::{
    algo::{algo_result::AlgoResult, utils::QueueItem},
    graph::Graph, index_vec,
};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, thread, time::{Duration, Instant}};
use crate::utility::IndexVec;
pub fn bidirectional<Ff, Fb>(
    graph: &Graph,
    from: u32,
    to: u32,
    use_arc_flags: bool, 
    potential_f: Ff,
    potential_b: Fb,
) -> AlgoResult
where
    Ff: Fn(&Graph, u32, u32, u32) -> f32 + Send + Sync + 'static,
    Fb: Fn(&Graph, u32, u32, u32) -> f32 + Send + Sync + 'static,
{
    let start = Instant::now();

    let mut dist_f: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); graph.size];
    let mut dist_b: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); graph.size];

    dist_f[from] = OrderedFloat(0.0);
    dist_b[to] = OrderedFloat(0.0);

    let mut que_f: BinaryHeap<QueueItem> = BinaryHeap::new();
    let mut que_b: BinaryHeap<QueueItem> = BinaryHeap::new();

    que_f.push(QueueItem {
        vertex: from,
        priority: OrderedFloat(0.0 + potential_f(graph, from, from, to)),
        distance: OrderedFloat(0.0),
    });
    que_b.push(QueueItem {
        vertex: to,
        priority: OrderedFloat(0.0 + potential_b(graph, to, from, to)),
        distance: OrderedFloat(0.0),
    });

    let mut best_dist = OrderedFloat(f32::MAX);

    let mut visited_nodes = 0;

    while !que_f.is_empty() && !que_b.is_empty() {
        if que_f.peek().unwrap().priority + que_b.peek().unwrap().priority >= best_dist {
            return AlgoResult::ok(*best_dist,visited_nodes, start.elapsed());
        }

        if !que_f.is_empty() {
            let cur = que_f.pop().unwrap();

            if cur.distance > dist_f[cur.vertex] {
                continue;
            }

            visited_nodes += 1;

            graph[cur.vertex].recolor(Color32::LIGHT_BLUE);

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

                let alt_cost = e.length + dist_f[cur.vertex].0;

                if alt_cost < dist_f[e.to] && dist_b[e.to] == OrderedFloat(f32::MAX) {
                    que_f.push(QueueItem::with_priority(
                        e.to,
                        alt_cost + potential_f(graph, e.to, from, to),
                        alt_cost,
                    ));
                    dist_f[e.to] = alt_cost;
                }
                if alt_cost + dist_b[e.to] < best_dist {
                    best_dist = alt_cost + dist_b[e.to];
                }
            } 
        }

        if !que_b.is_empty() {
            let cur = que_b.pop().unwrap();

            if cur.distance > dist_b[cur.vertex] {
                continue;
            }

            visited_nodes += 1;

            graph[cur.vertex].recolor(Color32::LIGHT_BLUE);

            for (edge_idx, e) in graph[cur.vertex].edges_rev.iter() {
                match (use_arc_flags, &graph.edge_region_flags, &graph.regions){
                    (true, Some(edge_region_flags), Some(regions)) =>{
                        if !edge_region_flags[cur.vertex][edge_idx][regions[from] as usize] 
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
            
                
                let alt_cost = e.length + dist_b[cur.vertex].0;

                if alt_cost < dist_b[e.to] && dist_f[e.to] == OrderedFloat(f32::MAX) {
                    que_b.push(QueueItem::with_priority(
                        e.to,
                        alt_cost + potential_b(graph, e.to, from, to),
                        alt_cost,
                    ));
                    dist_b[e.to] = alt_cost;
                }
                if alt_cost + dist_f[e.to] < best_dist {
                    best_dist = alt_cost + dist_f[e.to];
                }
            }
        }
    }

    match best_dist {
        OrderedFloat(f32::MAX) => AlgoResult::err("No path found"),
        e => AlgoResult::ok(*e, visited_nodes, start.elapsed())
    }
}
