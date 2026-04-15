use std::collections::BTreeSet;
use ordered_float::OrderedFloat;
use crate::{graph::Graph, index_vec};
use crate::utility::IndexVec;

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

            for (&neighbour, &edge_dist) in edges {
                let new_dist = edge_dist + cur_dist;
                if new_dist < dist[neighbour] {
                    que.remove(&(dist[neighbour], neighbour));
                    dist[neighbour] = new_dist;
                    que.insert((new_dist, neighbour));
                }
            }
        }

        dist
    }
}

pub enum EdgeDir {
    Forward,
    Reverse,
}
