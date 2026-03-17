use crate::{algo::utils::QueueItem, graph::Graph};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, sync::Arc, thread, time::Duration};

pub fn astar(
    graph: Arc<Graph>,
    from: usize,
    to: usize,
    animate: bool,
    potential: fn((f64, f64), (f64, f64)) -> f64,
) -> Option<(f64, usize)> {
    let target_coords = graph.vertices[to].coords;

    let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];

    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem {
        vertex: from,
        cost: OrderedFloat(0.0 + potential(graph.vertices[from].coords, target_coords)),
    });

    let mut visited_nodes = 0;

    while !que.is_empty() {
        let cur = que.pop().unwrap();

        if animate {
            graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
            thread::sleep(Duration::from_millis(2));
        }

        visited_nodes += 1;

        if cur.vertex == to {
            return Some((dist[to].0, visited_nodes));
        }

        for c in &graph.vertices[cur.vertex].connections {
            let alt_cost = c.1 + dist[cur.vertex].0;
            if alt_cost < dist[*c.0] {
                que.push(QueueItem {
                    vertex: *(c.0),
                    cost: alt_cost + potential(graph.vertices[*c.0].coords, target_coords),
                });
                dist[*c.0] = alt_cost;
            }
        }
    }
    None
}
