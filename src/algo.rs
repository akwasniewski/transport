use crate::graph::{self, Graph, Vertex};
use std::{collections::BinaryHeap};
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

pub fn dijsktra(graph: &Graph, from: usize, to: usize)->Option<f64>{
    let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];

    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem{vertex: from, cost:OrderedFloat(0.0)});

    while !que.is_empty(){
        let cur = que.pop().unwrap();
        if cur.vertex == to {  return Some(dist[to].0); }
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

pub fn a_star_2d(graph: &Graph, from: usize, to: usize)->Option<f64>{

    fn simple_dist(coords1: (f64, f64), coords2: (f64, f64))->f64{
        f64::sqrt(f64::powf(coords2.0-coords1.0,2.0) + f64::powf(coords2.1 - coords1.1,2.0))
    }

    let target_coords = graph.vertices[to].coords;

    let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); graph.size];

    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem{vertex: from, cost:OrderedFloat(0.0+simple_dist(graph.vertices[from].coords, target_coords))});

    while !que.is_empty(){
        let cur = que.pop().unwrap();
        if cur.vertex == to { return Some(dist[to].0); }

        for c in &graph.vertices[cur.vertex].connections{
            let alt_cost = c.1+dist[cur.vertex].0;
            if alt_cost<dist[*c.0]{
                que.push(QueueItem{vertex: *(c.0), cost: alt_cost + simple_dist(graph.vertices[*c.0].coords, target_coords)});
                dist[*c.0]=alt_cost;
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
        other.cost.cmp(&self.cost)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}