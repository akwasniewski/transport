use std::collections::BinaryHeap;

use eframe::egui::debug_text::print;
use ordered_float::OrderedFloat;

use crate::{algo::utils::QueueItem, graph::Graph};

    
impl Graph{
    pub fn tree_edge_region_flags(&mut self, from: usize) {

        let edge_region_flags = self.edge_region_flags.as_mut().unwrap();
        let regions = self.regions.as_ref().unwrap();

        let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); self.size];
        let mut pred: Vec<usize> = vec![0; self.size];

        let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
        dist[from] = OrderedFloat(0.0);
        que.push(QueueItem::new(from, OrderedFloat(0.0)));
        while !que.is_empty() {
            let cur = que.pop().unwrap();

            if cur.distance > dist[cur.vertex] {
                continue;
            }

            //assign region flag 
            if cur.vertex != from{
                edge_region_flags[cur.vertex].get_mut(&pred[cur.vertex]).unwrap()[regions[from]] = true;
            }

            for c in &self.vertices[cur.vertex].edges_rev {
                let alt = QueueItem::new(*c.0, c.1 + cur.distance);
                if alt.distance < dist[alt.vertex] {
                    que.push(alt);
                    dist[alt.vertex] = alt.distance;
                    pred[alt.vertex] = cur.vertex;
                }
            }
        }
    }

    pub fn tree_edge_region_flags_rev(&mut self, to: usize) {
        let edge_region_flags_rev = self.edge_region_flags_rev.as_mut().unwrap();
        let regions = self.regions.as_ref().unwrap();

        let mut dist: Vec<OrderedFloat<f64>> = vec![OrderedFloat(f64::MAX); self.size];
        let mut pred: Vec<usize> = vec![0; self.size];

        let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
        dist[to] = OrderedFloat(0.0);
        que.push(QueueItem::new(to, OrderedFloat(0.0)));
        while !que.is_empty() {
            let cur = que.pop().unwrap();

            if cur.distance > dist[cur.vertex] {
                continue;
            }

            //assign region flag 
            if cur.vertex != to{
                edge_region_flags_rev[cur.vertex].get_mut(&pred[cur.vertex]).unwrap()[regions[to]] = true;
            }

            for c in &self.vertices[cur.vertex].edges {
                let alt = QueueItem::new(*c.0, c.1 + cur.distance);
                if alt.distance < dist[alt.vertex] {
                    que.push(alt);
                    dist[alt.vertex] = alt.distance;
                    pred[alt.vertex] = cur.vertex;
                }
            }
        }

    }
}
