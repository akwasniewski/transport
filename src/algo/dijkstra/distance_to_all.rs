use std::collections::BTreeSet;
use ordered_float::OrderedFloat;
use crate::{graph::Graph, index_vec};
use crate::utility::{EdgeDir, IndexVec};

impl Graph {
    pub fn distance_to_all(&self, from: u32, dir: EdgeDir) -> IndexVec<OrderedFloat<f32>> {
        let mut dist = index_vec![OrderedFloat(f32::MAX); self.size];
        let mut que: BTreeSet<(OrderedFloat<f32>, u32)> = BTreeSet::new();

        dist[from] = OrderedFloat(0.0);
        que.insert((OrderedFloat(0.0), from));

        while let Some(&(cur_dist, cur)) = que.iter().next() {
            que.remove(&(cur_dist, cur));

            let edges = match dir {
                EdgeDir::Forward  => &self[cur].edges,
                EdgeDir::Reverse  => &self[cur].edges_rev,
            };

            for e in edges {
                let new_dist = e.length + cur_dist;
                if new_dist < dist[e.to] {
                    que.remove(&(dist[e.to], e.to));
                    dist[e.to] = new_dist;
                    que.insert((new_dist, e.to));
                }
            }
        }

        dist
    }
}
