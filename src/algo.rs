use crate::graph::{Graph, Vertex};
use std::collections::BinaryHeap;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

pub fn dijsktra(graph: &Graph, from: usize, to: usize)->Option<f32>{
    let mut dist: Vec<OrderedFloat<f32>> = vec![OrderedFloat(f32::MAX); graph.size];

    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem{vertex: from, cost:OrderedFloat(0.0)});

    while !que.is_empty(){
        let cur = que.pop().unwrap();
        if cur.vertex == to { return Some(cur.cost.0); }
        if cur.cost>dist[cur.vertex]{continue;}
        for c in &graph.vertices[cur.vertex].connections{
            let alt: QueueItem = QueueItem{vertex: *(c.0), cost: c.1+cur.cost};
            if alt.cost<dist[alt.vertex]{
                que.push(alt);
                dist[alt.vertex]=alt.cost;
            }
        }
    }
    None
}


#[derive(Copy, Clone, Eq, PartialEq)]
struct QueueItem {
    vertex: usize,
    cost: OrderedFloat<f32>,
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}