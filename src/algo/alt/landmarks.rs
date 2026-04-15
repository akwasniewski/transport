use std::{collections::HashMap};
use ordered_float::OrderedFloat;
use crate::algo::dijkstra::distance_to_all::EdgeDir; 
use crate::graph::{Graph, LandmarkData};
use crate::utility::IndexVec;

impl Graph {
    pub fn get_random_landmarks(&mut self, count: usize) {
        self.landmarks = HashMap::new();
        let mut i = 0;
        while i < count {
            let cur = rand::random_range(0..(self.size as u32));

            if self.landmarks.contains_key(&cur) {
                continue;
            }

            self.get_landmark_data(cur);
            i += 1;
        }
    }

    pub fn get_farthest_landmarks(&mut self, count: u32){
        self.landmarks = HashMap::new();
        let mut min_distances = vec![OrderedFloat(f32::MAX); self.size];

        let mut cur = rand::random_range(0..(self.size as u32)); 
        for _ in 0..count{
            self.get_landmark_data(cur);
            let cur_distances: &[OrderedFloat<f32>] = self.landmarks.get(&cur).as_ref().unwrap().from.as_ref();
            let mut next: (OrderedFloat<f32>, u32) = (OrderedFloat(0.0), 0);
            for j in 0..self.size{
                min_distances[j]=cur_distances[j].min(min_distances[j]);
                if min_distances[j]>next.0 && min_distances[j]!=OrderedFloat(f32::MAX){
                    next = (min_distances[j], j as u32);
                }
            }
            cur = next.1;
        }
    }

    pub fn get_landmark_data(&mut self, vertex: u32){
        self.landmarks.insert(
                vertex,
                LandmarkData {
                    from: self.distance_to_all(vertex, EdgeDir::Forward),
                    to: self.distance_to_all(vertex, EdgeDir::Reverse),
                },
            );
    }
}

pub fn landmark_potential(
    cur: u32,
    target: u32,
    dist: &IndexVec<OrderedFloat<f32>>,
    rev_dist: &IndexVec<OrderedFloat<f32>>,
) -> OrderedFloat<f32> {
    let dist = dist[target] - dist[cur];
    let rev_dist = rev_dist[cur] - rev_dist[target];
    dist.max(rev_dist)
}

pub fn alt_potential(graph: &Graph, cur: u32, _from: u32, to: u32) -> f32 {
    let mut best = OrderedFloat(f32::MIN);
    for landmark in &graph.landmarks {
        let val = landmark_potential(cur, to, &landmark.1.from, &landmark.1.to);
        if val > best {
            best = val;
        }
    }
    *best
}
