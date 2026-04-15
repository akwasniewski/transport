use crate::{
    algo::{algo_result::AlgoResult, utils::QueueItem},
    graph::Graph, index_vec,
};
use eframe::egui::Color32;
use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, thread};
use crate::utility::IndexVec;

pub fn bidirectional_dijkstra(graph: &Graph, from: u32, to: u32, animate: bool) -> AlgoResult {
    let mut dist_f: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); graph.size];
    let mut dist_b: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); graph.size];
    dist_f[from] = OrderedFloat(0.0);
    dist_b[to] = OrderedFloat(0.0);

    let mut que_f: BinaryHeap<QueueItem> = BinaryHeap::new();
    let mut que_b: BinaryHeap<QueueItem> = BinaryHeap::new();

    que_f.push(QueueItem::new(from, OrderedFloat(0.0)));

    que_b.push(QueueItem::new(to, OrderedFloat(0.0)));

    let mut best_dist = OrderedFloat(f32::MAX);

    let mut visited_nodes = 0;

    while !que_f.is_empty() && !que_b.is_empty() {
        if que_f.peek().unwrap().distance + que_b.peek().unwrap().distance >= best_dist {
            return AlgoResult {
                distance: Some(*best_dist),
                visited_nodes,
            };
        }

        // we choose smaller key
        if que_f.peek().unwrap().distance <= que_b.peek().unwrap().distance {
            let cur = que_f.pop().unwrap();

            if cur.distance > dist_f[cur.vertex] {
                continue;
            }

            visited_nodes += 1;

            if animate && cur.vertex != from && cur.vertex != to {
                graph[cur.vertex].recolor(Color32::LIGHT_BLUE);
                thread::sleep(std::time::Duration::from_millis(10));
            }

            for c in &graph[cur.vertex].edges {
                let alt: QueueItem = QueueItem::new(*c.0, c.1 + cur.distance);
                if alt.distance < dist_f[alt.vertex] {
                    que_f.push(alt);
                    dist_f[alt.vertex] = alt.distance;
                }
                if alt.distance + dist_b[*c.0] < best_dist {
                    best_dist = alt.distance + dist_b[*c.0];
                }
            }
        } else {
            let cur = que_b.pop().unwrap();

            if cur.distance > dist_b[cur.vertex] {
                continue;
            }

            visited_nodes += 1;

            if animate && cur.vertex != from && cur.vertex != to {
                graph[cur.vertex].recolor(Color32::LIGHT_BLUE);
            }
            for c in &graph[cur.vertex].edges_rev {
                let alt: QueueItem = QueueItem::new(*c.0, c.1 + cur.distance);
                if alt.distance < dist_b[alt.vertex] {
                    que_b.push(alt);
                    dist_b[alt.vertex] = alt.distance;
                }
                if alt.distance + dist_f[*c.0] < best_dist {
                    best_dist = alt.distance + dist_f[*c.0];
                }
            }
        }
    }

    AlgoResult {
        distance: None,
        visited_nodes,
    }
}
