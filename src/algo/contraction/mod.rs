use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap, HashMap, HashSet}, fs::File, io::{BufReader, BufWriter}};


use hashbrown::hash_map;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

use crate::{algo::utils::QueueItem, graph::{Edge, Graph}, index_vec, utility::IndexVec};

impl Graph{
    fn contract_vertex(&mut self, v: u32, contracted: &HashSet<u32>) -> u32{
        let added_shortcuts = self.simulate_contraction(v, contracted);

        let added_edges = added_shortcuts.len() as u32;
        for s in added_shortcuts{
            self[s.0].edges.push(Edge{to:s.1, length:s.2});
            self[s.1].edges_rev.push(Edge{to: s.0, length: s.2});
        }

        added_edges
    }
    
    fn simulate_contraction(&self, v: u32, contracted: &HashSet<u32>) -> Vec<(u32, u32, OrderedFloat<f32>)>{
        let mut longest_edge = OrderedFloat(0.0);
        for e in &self[v].edges{
            if e.length>longest_edge{
                longest_edge = e.length;
            }
        }
        let mut added_shortcuts: Vec<(u32, u32, OrderedFloat<f32>)> = Vec::new();
        for e_in in &self[v].edges_rev{
            if contracted.contains(&e_in.to) || self[v].rank > self[e_in.to].rank{
                continue;
            }

            let b_max = e_in.length + longest_edge;
            
            let mut dist: HashMap<u32, OrderedFloat<f32>> = HashMap::new(); 
            let mut que: PriorityQueue<u32, Reverse<OrderedFloat<f32>>>= PriorityQueue::new();
            dist.insert(e_in.to, OrderedFloat(0.0));
            que.push(e_in.to, Reverse(OrderedFloat(0.0)));

            while !que.is_empty(){
                let cur = que.pop().unwrap();
                let cur_vertex = cur.0;
                let cur_priority = cur.1;
                if dist.get(&cur_vertex).is_some_and(|d| d<&cur_priority.0){
                    continue;
                }
                if cur_priority.0 > b_max{
                    break;
                }
                
                for e_out in &self[cur_vertex].edges{
                    if e_out.to == v || contracted.contains(&e_out.to){
                        continue;
                    }

                    let alt_cost = e_out.length + cur_priority.0;
                    if dist.get(&e_out.to).is_some_and(|d| *d <= alt_cost){
                        continue;
                    }

                    que.push_increase(e_out.to, Reverse(alt_cost));
                    dist.insert(e_out.to, alt_cost);
                }
            }
            for e_out in &self[v].edges{
                if contracted.contains(&e_out.to) || self[v].rank > self[e_out.to].rank{
                    continue;
                }
                let b = e_in.length + e_out.length;
                if dist.get(&e_out.to).is_some_and(|x| *x<b){
                    continue;
                } 

                added_shortcuts.push((e_in.to, e_out.to, b));
            }


        }
        added_shortcuts
    }


    pub fn contract(&mut self) -> u32{
        let mut contracted: HashSet<u32> = HashSet::new();
        let mut que: PriorityQueue<u32, Reverse<OrderedFloat<f32>>>= PriorityQueue::new();
        for (v_index, v) in self.vertices.iter().enumerate(){
            let cur_priority = self.get_priority(v_index as u32, &contracted);
            que.push(v_index as u32, Reverse(cur_priority));
        }
        let mut contract_order: IndexVec<u32> = IndexVec::new(); 
        let mut cur_rank = 0;

        let mut added_edges = 0;
        while !que.is_empty() {
            let cur = que.pop().unwrap();
            let cur_vertex = cur.0;
            let cur_priority = self.get_priority(cur_vertex, &contracted);
            if !que.is_empty() && &Reverse(cur_priority) < que.peek().unwrap().1{
                que.push(cur_vertex, Reverse(cur_priority));
                continue;
            }

            added_edges += self.contract_vertex(cur_vertex, &contracted);
            contracted.insert(cur_vertex);
            contract_order.push(cur_vertex);
            self[cur_vertex].rank = cur_rank;
            cur_rank+=1;

            for e in &self[cur_vertex].edges{
                if contracted.contains(&e.to){
                    continue;
                }
                let e_priority = self.get_priority(e.to, &contracted);
                que.push(e.to, Reverse(e_priority));
            }

            for e in &self[cur_vertex].edges_rev {
                if contracted.contains(&e.to) { 
                    continue; 
                }
                let e_priority = self.get_priority(e.to, &contracted);
                que.push_decrease(e.to, Reverse(e_priority));
            }
        }
        added_edges
    }
    fn get_priority(&self, v: u32, contracted: &HashSet<u32>) -> OrderedFloat<f32> {
        OrderedFloat(2.0 * self.simulate_contraction(v, contracted).len() as f32
                - (self[v].edges.len()) as f32 + self[v].edges_rev.len() as f32)  // ← only counts outgoing edges!
    }

    pub fn save_contraction(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let data: Vec<(u32, &IndexVec<Edge>, &IndexVec<Edge>)> = self.vertices.iter()
            .map(|v| (v.rank, &v.edges, &v.edges_rev))
            .collect();
        bincode::serialize_into(writer, &data)?;
        Ok(())
    }

    pub fn load_contraction(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: Vec<(u32, IndexVec<Edge>, IndexVec<Edge>)> = bincode::deserialize_from(reader)?;
        for (i, (rank, edges, edges_rev)) in data.into_iter().enumerate() {
            self.vertices[i].rank = rank;
            self.vertices[i].edges = edges;
            self.vertices[i].edges_rev = edges_rev;
        }
        Ok(())
    }
}
