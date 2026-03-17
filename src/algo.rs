use crate::graph::{self, Graph, Vertex};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn dijsktra(graph: Arc<Graph>, from: usize, to: usize, animate: bool) -> Option<(f64, usize)> {
    let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];

    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem {
        vertex: from,
        cost: OrderedFloat(0.0),
    });
    let mut visited_nodes = 0;
    while !que.is_empty() {
        let cur = que.pop().unwrap();

        if animate {
            graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
            thread::sleep(Duration::from_millis(2));
        }

        if cur.cost > dist[cur.vertex] {
            continue;
        }

        visited_nodes += 1;

        if cur.vertex == to {
            return Some((dist[to].0, visited_nodes));
        }
        for c in &graph.vertices[cur.vertex].connections {
            let alt: QueueItem = QueueItem {
                vertex: *(c.0),
                cost: c.1 + cur.cost,
            };
            if alt.cost < dist[alt.vertex] {
                que.push(alt);
                dist[alt.vertex] = alt.cost;
            }
        }
    }
    None
}

pub fn a_star_2d(graph: Arc<Graph>, from: usize, to: usize, animate: bool) -> Option<(f64, usize)> {
    fn simple_dist(coords1: (f64, f64), coords2: (f64, f64)) -> f64 {
        let r = 6371009.0; // earth's radius

        let coords1 = (coords1.0.to_radians(), coords1.1.to_radians());
        let coords2 = (coords2.0.to_radians(), coords2.1.to_radians());

        let delta_lat = coords1.0 - coords2.0;
        let mid_lat = (coords1.0 + coords2.0) / 2.0;
        let delta_long = coords1.1 - coords2.1;

        let x = delta_lat;
        let y = mid_lat.cos() * delta_long;
        let tunnel_dist = (x.powi(2) + y.powi(2)).sqrt();
        2.0 * r * (tunnel_dist / 2.0).asin()
    }

    let target_coords = graph.vertices[to].coords;

    let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];

    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem {
        vertex: from,
        cost: OrderedFloat(0.0 + simple_dist(graph.vertices[from].coords, target_coords)),
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
                    cost: alt_cost + simple_dist(graph.vertices[*c.0].coords, target_coords),
                });
                dist[*c.0] = alt_cost;
            }
        }
    }
    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct QueueItem {
    vertex: usize,
    cost: OrderedFloat<f64>,
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}
