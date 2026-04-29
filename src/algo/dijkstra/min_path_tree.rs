use std::collections::BinaryHeap;
use ordered_float::OrderedFloat;
use crate::index_vec;
use crate::{algo::utils::QueueItem, graph::Graph};
use crate::utility::{EdgeDir, IndexVec};
    
impl Graph{
    pub fn tree_edge_region_flags(&mut self, source: u32, dir: EdgeDir) {
        let regions = self.regions.as_ref().unwrap();
        let region_source = regions[source];
        let mut dist: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); self.size];
        let mut pred: IndexVec<u32> = index_vec![0; self.size];
        let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
        dist[source] = OrderedFloat(0.0);
        que.push(QueueItem::new(source, OrderedFloat(0.0)));
        while !que.is_empty() {
            let cur = que.pop().unwrap();
            if cur.distance > dist[cur.vertex] {
                continue;
            }
            if cur.vertex != source {
                let flags = match dir {
                    EdgeDir::Forward => self.edge_region_flags.as_mut().unwrap(),
                    EdgeDir::Reverse => self.edge_region_flags_rev.as_mut().unwrap(),
                };
                flags[cur.vertex].get_mut(&pred[cur.vertex]).unwrap()[region_source] = true;
            }
            let neighbors: Vec<_> = match dir {
                EdgeDir::Forward => self.vertices[cur.vertex as usize].edges_rev.iter().map(|(k, v)| (*k, *v)).collect(),
                EdgeDir::Reverse => self.vertices[cur.vertex as usize].edges.iter().map(|(k, v)| (*k, *v)).collect(),
            };
            for (nb, weight) in neighbors {
                let alt = QueueItem::new(nb, weight + cur.distance);
                if alt.distance < dist[alt.vertex] {
                    que.push(alt);
                    dist[alt.vertex] = alt.distance;
                    pred[alt.vertex] = cur.vertex;
                }
            }
        }
    }
}
