use crate::{algo::utils::QueueItem, graph::Graph};
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, sync::Arc};

pub fn distance_to_all(graph: Arc<Graph>, from: usize) -> Vec<OrderedFloat<f64>> {
    let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];

    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem::new(from, OrderedFloat(0.0)));
    while !que.is_empty() {
        let cur = que.pop().unwrap();

        if cur.distance > dist[cur.vertex] {
            continue;
        }

        for c in &graph.vertices[cur.vertex].connections {
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
pub fn distance_to_all_rev(graph: Arc<Graph>, from: usize) -> Vec<OrderedFloat<f64>> {
    let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];

    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem::new(from, OrderedFloat(0.0)));
    while !que.is_empty() {
        let cur = que.pop().unwrap();

        if cur.distance > dist[cur.vertex] {
            continue;
        }

        for c in &graph.vertices[cur.vertex].incoming {
            let alt = QueueItem::new(*c.0, c.1 + cur.distance);
            if alt.distance < dist[alt.vertex] {
                que.push(alt);
                dist[alt.vertex] = alt.distance;
            }
        }
    }

    dist
}
