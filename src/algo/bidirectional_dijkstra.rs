use crate::{algo::utils::QueueItem, graph::Graph};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, sync::Arc, thread, time::Duration};

pub fn bidirectional_dijkstra(
    graph: Arc<Graph>,
    from: usize,
    to: usize,
    animate: bool,
) -> Option<(f64, usize)> {
    let mut dist_f: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];
    let mut dist_b: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];
    dist_f[from] = OrderedFloat(0.0);
    dist_b[to] = OrderedFloat(0.0);

    let mut que_f: BinaryHeap<QueueItem> = BinaryHeap::new();
    let mut que_b: BinaryHeap<QueueItem> = BinaryHeap::new();

    que_f.push(QueueItem {
        vertex: from,
        cost: OrderedFloat(0.0),
    });
    que_b.push(QueueItem {
        vertex: to,
        cost: OrderedFloat(0.0),
    });

    let mut best_dist = OrderedFloat(f64::MAX);

    let mut visited_nodes = 0;

    while !que_f.is_empty() && !que_b.is_empty() {
        // we choose smaller key
        if que_f.peek().unwrap().cost + que_b.peek().unwrap().cost >= best_dist + 10.0 {
            return Some((*best_dist, visited_nodes));
        }

        if que_f.peek().unwrap().cost <= que_b.peek().unwrap().cost {
            let cur = que_f.pop().unwrap();

            if cur.cost > dist_f[cur.vertex] {
                continue;
            }

            visited_nodes += 1;

            if animate && cur.vertex != from && cur.vertex != to {
                graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
                thread::sleep(Duration::from_millis(2));
            }
            for c in &graph.vertices[cur.vertex].connections {
                let alt: QueueItem = QueueItem {
                    vertex: *c.0,
                    cost: c.1 + cur.cost,
                };
                if alt.cost < dist_f[alt.vertex] {
                    que_f.push(alt);
                    dist_f[alt.vertex] = alt.cost;
                }
                if alt.cost + dist_b[*c.0] < best_dist {
                    best_dist = alt.cost + dist_b[*c.0];
                }
            }
        } else {
            let cur = que_b.pop().unwrap();

            if cur.cost > dist_b[cur.vertex] {
                continue;
            }

            visited_nodes += 1;

            if animate && cur.vertex != from && cur.vertex != to {
                graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
                thread::sleep(Duration::from_millis(2));
            }
            for c in &graph.vertices[cur.vertex].incoming {
                let alt: QueueItem = QueueItem {
                    vertex: *c.0,
                    cost: c.1 + cur.cost,
                };
                if alt.cost < dist_b[alt.vertex] {
                    que_b.push(alt);
                    dist_b[alt.vertex] = alt.cost;
                }
                if alt.cost + dist_f[*c.0] < best_dist {
                    best_dist = alt.cost + dist_f[*c.0];
                }
            }
        }
    }
    None
}
