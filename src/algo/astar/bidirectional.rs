use crate::{
    algo::{algo_result::AlgoResult, utils::QueueItem},
    graph::Graph,
};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, thread, time::Duration};

pub fn bidirectional_astar<Ff, Fb>(
    graph: &Graph,
    from: usize,
    to: usize,
    animate: bool,
    potential_f: Ff,
    potential_b: Fb,
) -> AlgoResult
where
    Ff: Fn(&Graph, usize, usize, usize) -> f64 + Send + Sync + 'static,
    Fb: Fn(&Graph, usize, usize, usize) -> f64 + Send + Sync + 'static,
{
    let mut dist_f: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];
    let mut dist_b: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];

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

    let mut best_dist = OrderedFloat(f64::MAX);

    let mut visited_nodes = 0;

    while !que_f.is_empty() && !que_b.is_empty() {
        if que_f.peek().unwrap().priority + que_b.peek().unwrap().priority >= best_dist {
            return AlgoResult {
                distance: Some(*best_dist),
                visited_nodes,
            };
        }

        if !que_f.is_empty() {
            let cur = que_f.pop().unwrap();

            if cur.distance > dist_f[cur.vertex] {
                continue;
            }

            visited_nodes += 1;

            if animate && cur.vertex != from && cur.vertex != to {
                graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
                thread::sleep(std::time::Duration::from_millis(10));
            }

            for c in &graph.vertices[cur.vertex].edges {
                let alt_cost = c.1 + dist_f[cur.vertex].0;

                if alt_cost < dist_f[*c.0] && dist_b[*c.0] == OrderedFloat(f64::MAX) {
                    que_f.push(QueueItem::with_priority(
                        *(c.0),
                        alt_cost + potential_f(graph, *c.0, from, to),
                        alt_cost,
                    ));
                    dist_f[*c.0] = alt_cost;
                }
                if alt_cost + dist_b[*c.0] < best_dist {
                    best_dist = alt_cost + dist_b[*c.0];
                }
            }
        }
        if !que_b.is_empty() {
            let cur = que_b.pop().unwrap();

            if cur.distance > dist_b[cur.vertex] {
                continue;
            }

            visited_nodes += 1;

            if animate && cur.vertex != from && cur.vertex != to {
                graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
                thread::sleep(Duration::from_millis(2));
            }

            for c in &graph.vertices[cur.vertex].edges_rev {
                let alt_cost = c.1 + dist_b[cur.vertex].0;

                if alt_cost < dist_b[*c.0] && dist_f[*c.0] == OrderedFloat(f64::MAX) {
                    que_b.push(QueueItem::with_priority(
                        *(c.0),
                        alt_cost + potential_b(graph, *c.0, from, to),
                        alt_cost,
                    ));
                    dist_b[*c.0] = alt_cost;
                }
                if alt_cost + dist_f[*c.0] < best_dist {
                    best_dist = alt_cost + dist_f[*c.0];
                }
            }
        }
    }

    AlgoResult {
        distance: match best_dist {
            OrderedFloat(f64::MAX) => None,
            e => Some(*e),
        },
        visited_nodes,
    }
}
