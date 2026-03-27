use crate::{algo::utils::QueueItem, graph::Graph};
use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;

impl Graph {
    pub fn distance_to_all(&self, from: usize) -> Vec<OrderedFloat<f64>> {
        let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); self.size];

        let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
        dist[from] = OrderedFloat(0.0);
        que.push(QueueItem::new(from, OrderedFloat(0.0)));
        while !que.is_empty() {
            let cur = que.pop().unwrap();

            if cur.distance > dist[cur.vertex] {
                continue;
            }

            for c in &self.vertices[cur.vertex].edges {
                let alt = QueueItem::new(*c.0, c.1 + cur.distance);
                if alt.distance < dist[alt.vertex] {
                    que.push(alt);
                    dist[alt.vertex] = alt.distance;
                }
            }
        }

        dist
    }

    //TODO : unify it to one function
    pub fn distance_to_all_rev(&self, from: usize) -> Vec<OrderedFloat<f64>> {
        let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); self.size];

        let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
        dist[from] = OrderedFloat(0.0);
        que.push(QueueItem::new(from, OrderedFloat(0.0)));
        while !que.is_empty() {
            let cur = que.pop().unwrap();

            if cur.distance > dist[cur.vertex] {
                continue;
            }

            for c in &self.vertices[cur.vertex].edges_rev {
                let alt = QueueItem::new(*c.0, c.1 + cur.distance);
                if alt.distance < dist[alt.vertex] {
                    que.push(alt);
                    dist[alt.vertex] = alt.distance;
                }
            }
        }

        dist
    }
}
