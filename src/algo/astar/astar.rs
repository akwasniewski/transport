use crate::{
    algo::{algo_result::AlgoResult, utils::QueueItem},
    graph::Graph,
};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, thread};

pub fn astar(
    graph: &Graph,
    from: usize,
    to: usize,
    animate: bool,
    potential: fn(&Graph, usize, usize, usize) -> f64,
) -> AlgoResult {
    let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];

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

        if animate && cur.vertex != from && cur.vertex != to {
            graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
            thread::sleep(std::time::Duration::from_millis(10));
        }

        visited_nodes += 1;

        if cur.vertex == to {
            return AlgoResult {
                distance: Some(dist[to].0),
                visited_nodes,
            };
        }

        for c in &graph.vertices[cur.vertex].edges {
            let alt_cost = c.1 + dist[cur.vertex].0;
            if alt_cost < dist[*c.0] {
                que.push(QueueItem {
                    vertex: *(c.0),
                    priority: alt_cost + potential(graph, *c.0, from, to),
                    distance: alt_cost,
                });
                dist[*c.0] = alt_cost;
            }
        }
    }
    AlgoResult {
        distance: None,
        visited_nodes,
    }
}
