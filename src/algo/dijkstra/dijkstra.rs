use crate::{
    algo::{algo_result::AlgoResult, utils::QueueItem},
    graph::Graph, index_vec,
};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, thread};
use crate::utility::IndexVec;

pub fn dijkstra(graph: &Graph, from: u32, to: u32, animate: bool) -> AlgoResult {
    let mut dist: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); graph.size];

    let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
    dist[from] = OrderedFloat(0.0);
    que.push(QueueItem::new(from, OrderedFloat(0.0)));
    let mut visited_nodes = 0;
    while !que.is_empty() {
        let cur = que.pop().unwrap();

        if animate && cur.vertex != from && cur.vertex != to {
            graph[cur.vertex].recolor(Color32::LIGHT_BLUE);
            thread::sleep(std::time::Duration::from_millis(10));
        }

        if cur.distance > dist[cur.vertex] {
            continue;
        }

        visited_nodes += 1;

        if cur.vertex == to {
            return AlgoResult {
                distance: Some(dist[to].0),
                visited_nodes,
            };
        }
        for c in &graph[cur.vertex].edges {
            let alt = QueueItem::new(*c.0, c.1 + cur.distance);
            if alt.distance < dist[alt.vertex] {
                que.push(alt);
                dist[alt.vertex] = alt.distance;
            }
        }
    }

    AlgoResult {
        distance: None,
        visited_nodes,
    }
}
