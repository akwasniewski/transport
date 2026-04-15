use std::collections::BinaryHeap;
use ordered_float::OrderedFloat;
use crate::index_vec;
use crate::{algo::utils::QueueItem, graph::Graph};
use crate::utility::IndexVec;
    
impl Graph{
    pub fn tree_edge_region_flags(&mut self, from: u32) {

        let edge_region_flags = self.edge_region_flags.as_mut().unwrap();
        let regions = self.regions.as_ref().unwrap();

        let mut dist: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); self.size];
        let mut pred: IndexVec<u32> = index_vec![0; self.size];

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

            for c in &self.vertices[cur.vertex as usize].edges_rev {
                let alt = QueueItem::new(*c.0, c.1 + cur.distance);
                if alt.distance < dist[alt.vertex] {
                    que.push(alt);
                    dist[alt.vertex] = alt.distance;
                    pred[alt.vertex] = cur.vertex;
                }
            }
        }
    }

    pub fn tree_edge_region_flags_rev(&mut self, to: u32) {
        let edge_region_flags_rev = self.edge_region_flags_rev.as_mut().unwrap();
        let regions = self.regions.as_ref().unwrap();

        let mut dist: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); self.size];
        let mut pred: IndexVec<u32> = index_vec![0; self.size];

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

            for c in &self.vertices[cur.vertex as usize].edges {
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
