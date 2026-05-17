use std::collections::{BinaryHeap, HashMap};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

use crate::algo::utils::QueueItem;
use crate::graph::Graph;
use crate::index_vec;
use crate::utility::{EdgeDir, IndexVec};


use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use bincode::{serialize_into, deserialize_from};
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use bitvec::prelude::*;
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
        edges.iter().any(|e| regions[e.1.to] != regions[vertex])
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
                EdgeDir::Forward => self.vertices[vertex].edges.iter().collect(),
                EdgeDir::Reverse => self.vertices[vertex].edges_rev.iter().collect(),
            };
            let flags = match dir {
                EdgeDir::Forward => self.edge_region_flags.as_mut().unwrap(),
                EdgeDir::Reverse => self.edge_region_flags_rev.as_mut().unwrap(),
            };
            flags.push(IndexVec::new());
            for _ in edges.iter() {
                flags[vertex as u32].push(bitvec![AtomicUsize, Lsb0; 0; region_count]);
            }
        }

        let border_vertices: Vec<u32> = (0..self.size as u32)
            .filter(|&v| self.is_vertex_on_region_border(v, dir))
            .collect();

        border_vertices.par_iter().for_each(|&v| self.compute_region_flags(v, dir));
    }
    fn compute_region_flags(&self, source: u32, dir: EdgeDir) {
        let regions = self.regions.as_ref().unwrap();
        let region_source = regions[source] as usize;
        let flags = match dir {
            EdgeDir::Forward => self.edge_region_flags.as_ref().unwrap(),
            EdgeDir::Reverse => self.edge_region_flags_rev.as_ref().unwrap(),
        };
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
                let pred_edge_idx = match dir{
                    EdgeDir::Forward => self.vertices[cur.vertex as usize].edges.iter().collect::<Vec<_>>(),
                    EdgeDir::Reverse => self.vertices[cur.vertex as usize].edges_rev.iter().collect::<Vec<_>>(),
                }.iter().position(|e| e.1.to == pred[cur.vertex]).unwrap();

            let word_idx = region_source / usize::BITS as usize;
            let bit_idx  = region_source % usize::BITS as usize;

            flags[cur.vertex][pred_edge_idx as u32]
                .as_raw_slice()[word_idx]
                .fetch_or(1 << bit_idx, Ordering::Relaxed);
            }
            let neighbors = match dir {
                EdgeDir::Forward => self.vertices[cur.vertex as usize].edges_rev.iter().collect::<Vec<_>>(),
                EdgeDir::Reverse => self.vertices[cur.vertex as usize].edges.iter().collect::<Vec<_>>(),
            };
            for (_, e) in neighbors {
                let alt = QueueItem::new(e.to, e.length + cur.distance);
                if alt.distance < dist[alt.vertex] {
                    que.push(alt);
                    dist[alt.vertex] = alt.distance;
                    pred[alt.vertex] = cur.vertex;
                }
            }
        }
    }
}



#[derive(Serialize, Deserialize)]
struct EdgeRegionCache {
    regions:               IndexVec<u32>,
    edge_region_flags:     IndexVec<IndexVec<BitVec<AtomicUsize, Lsb0>>>,
    edge_region_flags_rev: IndexVec<IndexVec<BitVec<AtomicUsize, Lsb0>>>,
}


impl Graph {
    pub fn save_edge_region_cache(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let regions = self.regions.as_ref().ok_or("regions is None")?;
        let flags = self.edge_region_flags.as_ref().ok_or("edge_region_flags is None")?;
        let flags_rev = self.edge_region_flags_rev.as_ref().ok_or("edge_region_flags_rev is None")?;

        let cache = EdgeRegionCache {
            regions:               regions.clone(),
            edge_region_flags:     flags.clone(),
            edge_region_flags_rev: flags_rev.clone(),
        };

        let file = File::create(path)?;
        serialize_into(BufWriter::new(file), &cache)?;
        Ok(())
    }

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
