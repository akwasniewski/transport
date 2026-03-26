use std::{collections::HashMap, sync::Arc};

use ordered_float::OrderedFloat;

use crate::{
    algo::dijkstra::distance_to_all::{distance_to_all, distance_to_all_rev},
    graph::Graph,
};

pub fn get_random_landmarks(
    graph: Arc<Graph>,
    count: usize,
) -> HashMap<usize, (Vec<OrderedFloat<f64>>, Vec<OrderedFloat<f64>>)> {
    let mut res: HashMap<usize, (Vec<OrderedFloat<f64>>, Vec<OrderedFloat<f64>>)> = HashMap::new();

    let mut i = 0;
    while i < count {
        let cur = rand::random_range(0..=graph.size);

        if res.contains_key(&cur) {
            continue;
        }

        res.insert(
            cur,
            (
                distance_to_all(graph.clone(), cur),
                distance_to_all_rev(graph.clone(), cur),
            ),
        );
        i += 1;
    }
    res
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

pub fn potential(
    cur: usize,
    target: usize,
    landmarks: &HashMap<usize, (Vec<OrderedFloat<f64>>, Vec<OrderedFloat<f64>>)>,
) -> OrderedFloat<f64> {
    let mut best = OrderedFloat(f64::MIN);
    for landmark in landmarks {
        let val = landmark_potential(cur, target, &landmark.1.0, &landmark.1.1);
        if val > best {
            best = val;
        }
    }
    best
}
