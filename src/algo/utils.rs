use std::cmp::Ordering;

use ordered_float::OrderedFloat;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct QueueItem {
    pub vertex: usize,
    pub cost: OrderedFloat<f64>,
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}
