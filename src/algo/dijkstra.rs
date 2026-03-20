use crate::{
    algo::{algo_result::AlgoResult, utils::QueueItem},
    graph::Graph,
};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, sync::Arc, thread, time::Duration};

pub fn dijkstra(graph: Arc<Graph>, from: usize, to: usize, animate: bool) -> AlgoResult {
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

        if animate && cur.vertex != from && cur.vertex != to {
            graph.vertices[cur.vertex].recolor(Color32::LIGHT_BLUE);
            thread::sleep(Duration::from_millis(2));
        }

        if cur.cost > dist[cur.vertex] {
            continue;
        }

        visited_nodes += 1;

        if cur.vertex == to {
            return AlgoResult {
                distance: Some(dist[to].0),
                visited_nodes,
            };
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

    AlgoResult {
        distance: None,
        visited_nodes,
    }
}
