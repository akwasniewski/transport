use crate::{
    algo::{algo_result::AlgoResult, utils::QueueItem},
    graph::Graph,
};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, sync::Arc, thread, time::Duration};

pub fn bidirectional_astar<F_f, F_b>(
    graph: Arc<Graph>,
    from: usize,
    to: usize,
    animate: bool,
    potential_f: F_f,
    potential_b: F_b,
) -> AlgoResult
where
    F_f: Fn((f64, f64), (f64, f64)) -> f64 + Send + Sync + 'static,
    F_b: Fn((f64, f64), (f64, f64)) -> f64 + Send + Sync + 'static,
{
    let mut dist_f: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];
    let mut dist_b: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];
    dist_f[from] = OrderedFloat(0.0);
    dist_b[to] = OrderedFloat(0.0);

    let mut que_f: BinaryHeap<QueueItem> = BinaryHeap::new();
    let mut que_b: BinaryHeap<QueueItem> = BinaryHeap::new();

    let target_coords = graph.vertices[to].coords;
    que_f.push(QueueItem {
        vertex: from,
        cost: OrderedFloat(0.0 + potential_f(graph.vertices[from].coords, target_coords)),
    });
    que_b.push(QueueItem {
        vertex: to,
        cost: OrderedFloat(0.0 + potential_b(graph.vertices[to].coords, target_coords)),
    });

    let mut best_dist = OrderedFloat(f64::MAX);

    let mut visited_nodes = 0;

    while !que_f.is_empty() && !que_b.is_empty() {
        if que_f.peek().unwrap().cost + que_b.peek().unwrap().cost >= best_dist {
            return AlgoResult {
                distance: Some(*best_dist),
                visited_nodes,
            };
        }

        if !que_f.is_empty() {
            let cur = que_f.pop().unwrap();

            visited_nodes += 1;

            if animate && cur.vertex != from && cur.vertex != to {
                graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
                thread::sleep(Duration::from_millis(2));
            }
            for c in &graph.vertices[cur.vertex].connections {
                let alt_cost = c.1 + dist_f[cur.vertex].0;

                if alt_cost < dist_f[*c.0] {
                    que_f.push(QueueItem {
                        vertex: *(c.0),
                        cost: alt_cost + potential_f(graph.vertices[*c.0].coords, target_coords),
                    });
                    dist_f[*c.0] = alt_cost;
                }
                if alt_cost + dist_b[*c.0] < best_dist {
                    best_dist = alt_cost + dist_b[*c.0];
                }
            }
        }
        if !que_b.is_empty() {
            let cur = que_b.pop().unwrap();

            visited_nodes += 1;

            if animate && cur.vertex != from && cur.vertex != to {
                graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
                thread::sleep(Duration::from_millis(2));
            }

            for c in &graph.vertices[cur.vertex].incoming {
                let alt_cost = c.1 + dist_b[cur.vertex].0;

                if alt_cost < dist_b[*c.0] {
                    que_b.push(QueueItem {
                        vertex: *(c.0),
                        cost: alt_cost + potential_b(graph.vertices[*c.0].coords, target_coords),
                    });
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
