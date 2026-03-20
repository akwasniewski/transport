use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct AlgoResult {
    pub distance: Option<f64>,
    pub visited_nodes: usize,
}

impl fmt::Display for AlgoResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.distance {
            Some(d) => write!(
                f,
                "distance: {:.2}, visited nodes: {}",
                d, self.visited_nodes
            ),
            None => write!(f, "No path found, visited nodes: {}", self.visited_nodes),
        }
    }
}
