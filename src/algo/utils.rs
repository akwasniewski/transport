use std::cmp::Ordering;

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
