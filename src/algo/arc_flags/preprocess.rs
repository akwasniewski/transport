use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

use crate::algo::utils::QueueItem;
use crate::graph::Graph;
use crate::index_vec;
use crate::utility::{EdgeDir, IndexVec};


use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use bincode::{serialize_into, deserialize_from};
use ordered_float::OrderedFloat;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Serialize, Deserialize};
 
impl Graph {
    pub fn count_border_vertices(&self) -> usize {
        (0..self.size as u32)
            .filter(|&v| self.is_vertex_on_region_border(v, EdgeDir::Forward))
            .count()
    }

    fn is_vertex_on_region_border(&self, vertex: u32, dir: EdgeDir) -> bool {
        let regions = self.regions.as_ref().unwrap();
        let edges = match dir {
            EdgeDir::Forward => &self.vertices[vertex as usize].edges,
            EdgeDir::Reverse => &self.vertices[vertex as usize].edges_rev,
        };
        edges.iter().any(|neighbour| regions[*neighbour.0] != regions[vertex])
    }

    pub fn preprocess_region_edges(&mut self, region_count: usize, dir: EdgeDir) {
        match dir {
            EdgeDir::Forward => self.edge_region_flags = Some(IndexVec::new()),
            EdgeDir::Reverse => self.edge_region_flags_rev = Some(IndexVec::new()),
        }

        if dir == EdgeDir::Forward {
            let border_count = self.count_border_vertices();
            println!("Border vertices: {}", border_count);
        }

        for vertex in 0..self.size {
            let edges: Vec<_> = match dir {
                EdgeDir::Forward => self.vertices[vertex].edges.iter().map(|(k, v)| (*k, *v)).collect(),
                EdgeDir::Reverse => self.vertices[vertex].edges_rev.iter().map(|(k, v)| (*k, *v)).collect(),
            };
            let flags = match dir {
                EdgeDir::Forward => self.edge_region_flags.as_mut().unwrap(),
                EdgeDir::Reverse => self.edge_region_flags_rev.as_mut().unwrap(),
            };
            flags.push(HashMap::new());
            for (k, _) in edges {
                flags[vertex as u32].insert(k, index_vec![false; region_count]);
            }
        }

        let border_vertices: Vec<u32> = (0..self.size as u32)
            .filter(|&v| self.is_vertex_on_region_border(v, dir))
            .collect();

        let updates: Vec<Vec<(u32, u32, u32)>> = border_vertices
            .par_iter()
            .map(|&v| self.compute_region_flags(v, dir))
            .collect();

        let flags = match dir {
            EdgeDir::Forward => self.edge_region_flags.as_mut().unwrap(),
            EdgeDir::Reverse => self.edge_region_flags_rev.as_mut().unwrap(),
        };
        for vertex_updates in updates {
            for (vertex, pred, region) in vertex_updates {
                flags[vertex].get_mut(&pred).unwrap()[region] = true;
            }
        }
    }
    fn compute_region_flags(&self, source: u32, dir: EdgeDir) -> Vec<(u32, u32, u32)> {
        let regions = self.regions.as_ref().unwrap();
        let region_source = regions[source];
        let mut dist: IndexVec<OrderedFloat<f32>> = index_vec![OrderedFloat(f32::MAX); self.size];
        let mut pred: IndexVec<u32> = index_vec![0; self.size];
        let mut que: BinaryHeap<QueueItem> = BinaryHeap::new();
        let mut updates = Vec::new();
        dist[source] = OrderedFloat(0.0);
        que.push(QueueItem::new(source, OrderedFloat(0.0)));
        while !que.is_empty() {
            let cur = que.pop().unwrap();
            if cur.distance > dist[cur.vertex] {
                continue;
            }
            if cur.vertex != source {
                updates.push((cur.vertex, pred[cur.vertex], region_source));
            }
            let neighbors = match dir {
                EdgeDir::Forward => self.vertices[cur.vertex as usize].edges_rev.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
                EdgeDir::Reverse => self.vertices[cur.vertex as usize].edges.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
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
        updates
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
struct EdgeRegionCache {
    regions:               IndexVec<u32>,
    edge_region_flags:     IndexVec<HashMap<u32, IndexVec<bool>>>,
    edge_region_flags_rev: IndexVec<HashMap<u32, IndexVec<bool>>>,
}

 
impl Graph {
    /// Serialize `regions`, `edge_region_flags` and `edge_region_flags_rev`
    /// to a binary file at `path`.  All three fields must be populated
    /// beforehand.
    pub fn save_edge_region_cache(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let regions = self.regions.as_ref()
            .ok_or("regions is None")?;
        let flags = self.edge_region_flags.as_ref()
            .ok_or("edge_region_flags is None – run preprocess_region_edges first")?;
        let flags_rev = self.edge_region_flags_rev.as_ref()
            .ok_or("edge_region_flags_rev is None – run preprocess_region_edges_rev first")?;
 
        let cache = EdgeRegionCache {
            regions:               regions.clone(),
            edge_region_flags:     flags.clone(),
            edge_region_flags_rev: flags_rev.clone(),
        };
 
        let file = File::create(path)?;
        serialize_into(BufWriter::new(file), &cache)?;
        Ok(())
    }
 
    /// Deserialize the cache written by `save_edge_region_cache` and populate
    /// `edge_region_flags` / `edge_region_flags_rev`.
    /// Returns `true` when a cache was loaded, `false` when the file does not
    /// exist (so the caller knows it should run preprocessing instead).
    pub fn load_edge_region_cache(&mut self, path: impl AsRef<Path>) -> Result<bool, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(false);
        }
 
        let file = File::open(path)?;
        let cache: EdgeRegionCache = deserialize_from(BufReader::new(file))?;
 
        self.regions               = Some(cache.regions);
        self.edge_region_flags     = Some(cache.edge_region_flags);
        self.edge_region_flags_rev = Some(cache.edge_region_flags_rev);
        Ok(true)
    }
}
