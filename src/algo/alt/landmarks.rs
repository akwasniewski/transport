use ordered_float::OrderedFloat;

use crate::graph::{Graph, LandmarkData};
impl Graph {
    pub fn get_random_landmarks(&mut self, count: usize) {
        let mut i = 0;
        while i < count {
            let cur = rand::random_range(0..=self.size);

            if self.landmarks.contains_key(&cur) {
                continue;
            }

            self.landmarks.insert(
                cur,
                LandmarkData {
                    to: self.distance_to_all(cur),
                    from: self.distance_to_all_rev(cur),
                },
            );
            i += 1;
        }
    }
}

pub fn landmark_potential(
    cur: usize,
    target: usize,
    dist: &[OrderedFloat<f64>],
    rev_dist: &[OrderedFloat<f64>],
) -> OrderedFloat<f64> {
    let dist = dist[target] - dist[cur];
    let rev_dist = rev_dist[cur] - rev_dist[target];
    dist.max(rev_dist)
}

pub fn alt_potential(graph: &Graph, cur: usize, _from: usize, to: usize) -> f64 {
    let mut best = OrderedFloat(f64::MIN);
    for landmark in &graph.landmarks {
        let val = landmark_potential(cur, to, &landmark.1.from, &landmark.1.to);
        if val > best {
            best = val;
        }
    }
    *best
}
