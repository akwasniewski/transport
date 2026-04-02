use std::collections::HashMap;

use crate::graph::Graph;

impl Graph{
    fn is_vertex_on_region_border(&self, vertex: usize) -> bool{
        let regions = self.regions.as_ref().unwrap();
        for neighbour in &self.vertices[vertex].edges{
           if regions[*neighbour.0] != regions[vertex]{
                return true;
            }
        }
        
        false
    }

    pub fn preprocess_region_edges(&mut self, region_count: usize) {
        self.edge_region_flags=Some(Vec::new());
        let edge_region_flags = self.edge_region_flags.as_mut().unwrap();
        let regions = self.regions.as_ref().unwrap();
        //initialize array
        for vertex in 0..self.size{
            edge_region_flags.push(HashMap::new());
            for edge in &self.vertices[vertex].edges{
                edge_region_flags[vertex].insert(*edge.0,vec![false;region_count]); 
            }

        }

        for vertex in 0..self.size{
            if self.is_vertex_on_region_border(vertex){
                self.tree_edge_region_flags(vertex);
            }
        }
    }
}
