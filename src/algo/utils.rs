use std::{cmp::Ordering, fmt};

use ordered_float::OrderedFloat;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct QueueItem {
    pub vertex: usize,
    pub priority: OrderedFloat<f64>,
    pub distance: OrderedFloat<f64>,
}
impl QueueItem {
    pub fn new(vertex: usize, cost: OrderedFloat<f64>) -> Self {
        Self {
            vertex,
            priority: cost,
            distance: cost,
        }
    }

    pub fn with_priority(
        vertex: usize,
        priority: OrderedFloat<f64>,
        distance: OrderedFloat<f64>,
    ) -> Self {
        Self {
            vertex,
            priority,
            distance,
        }
    }
}
impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}

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
                "Distance: {:.2}, Visited edges: {}",
                d, self.visited_nodes
            ),
            None => write!(f, "No path found, Visited nodes: {}", self.visited_nodes),
        }
    }
}
