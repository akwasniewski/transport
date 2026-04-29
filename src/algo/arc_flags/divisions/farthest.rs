use std::collections::BinaryHeap;

use crate::algo::arc_flags::divisions::utils::QueueItemSource;
use crate::graph::Graph;
use crate::index_vec;
use crate::utility::{EdgeDir, IndexVec};
use ordered_float::OrderedFloat;

pub fn multi_dijkstra_regions(graph: &mut Graph, sources: Vec<u32>) {
    let mut dist: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); graph.size];
    let mut que: BinaryHeap<QueueItemSource> = BinaryHeap::new();

    for i in 0..sources.len(){
        dist[sources[i]] = OrderedFloat(0.0);
        que.push(QueueItemSource::new(sources[i], i as u32,OrderedFloat(0.0)));
    }

    while !que.is_empty() {
        let cur = que.pop().unwrap();
        if cur.distance > dist[cur.vertex] {
            continue;
        }
        

        graph.regions.as_mut().unwrap()[cur.vertex]= cur.source;

        for c in &graph[cur.vertex].edges {
            let alt = QueueItemSource::new(*c.0, cur.source, c.1 + cur.distance);
            if alt.distance < dist[alt.vertex] {
                que.push(alt);
                dist[alt.vertex] = alt.distance;
            }
        }
    }
}

impl Graph{
    pub fn divide_into_regions_dijkstra(&mut self, region_count: u32){
        self.regions = Some(index_vec![0;self.vertices.len()]);
        let sources = self.get_farthest_points(region_count);
        multi_dijkstra_regions(self, sources);
    }

    pub fn get_farthest_points(&self, count: u32) -> Vec<u32>{
        let mut min_distances = vec![OrderedFloat(f32::MAX); self.size];
        let mut cur = rand::random_range(0..(self.size as u32)); 
        let mut res = vec!();
        for _ in 0..count{
            let cur_distances: IndexVec<OrderedFloat<f32>> = self.distance_to_all(cur, EdgeDir::Forward);
            let mut next: (OrderedFloat<f32>, u32) = (OrderedFloat(0.0), 0);
            for j in 0..self.size{
                min_distances[j]=cur_distances[j as u32].min(min_distances[j]);
                if min_distances[j]>next.0 && min_distances[j]!=OrderedFloat(f32::MAX){
                    next = (min_distances[j], j as u32);
                }
            }
            cur = next.1;
            res.push(cur);
        }
        res
    }
}
