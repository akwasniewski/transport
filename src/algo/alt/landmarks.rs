use std::{collections::HashMap, hash::Hash};

use eframe::egui::Order;
use ordered_float::OrderedFloat;

use crate::graph::{Graph, LandmarkData};
impl Graph {
    pub fn get_random_landmarks(&mut self, count: usize) {
        self.landmarks = HashMap::new();
        let mut i = 0;
        while i < count {
            let cur = rand::random_range(0..self.size);

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

    pub fn get_farthest_landmarks(&mut self, count: usize){
        self.landmarks = HashMap::new();
        let mut min_distances = vec![OrderedFloat(f64::MAX); self.size];

        let mut cur = rand::random_range(0..self.size); 
        for _ in 0..count{
            self.get_landmark_data(cur);
            let cur_distances: &Vec<OrderedFloat<f64>> = self.landmarks.get(&cur).as_ref().unwrap().from.as_ref();
            let mut next: (OrderedFloat<f64>, usize) = (OrderedFloat(0.0), 0);
            for j in 0..self.size{
                min_distances[j]=cur_distances[j].min(min_distances[j]);
                if min_distances[j]>next.0 && min_distances[j]!=OrderedFloat(f64::MAX){
                    next = (min_distances[j], j);
                }
            }
            cur = next.1;
        }
    }

    pub fn get_landmark_data(&mut self, vertex: usize){
        self.landmarks.insert(
                vertex,
                LandmarkData {
                    from: self.distance_to_all(vertex),
                    to: self.distance_to_all_rev(vertex),
                },
            );
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
