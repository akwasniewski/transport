use std::cmp::Ordering;

use ordered_float::OrderedFloat;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct QueueItem {
    pub vertex: u32,
    pub priority: OrderedFloat<f32>,
    pub distance: OrderedFloat<f32>,
}
impl QueueItem {
    pub fn new(vertex: u32, cost: OrderedFloat<f32>) -> Self {
        Self {
            vertex,
            priority: cost,
            distance: cost,
        }
    }

    pub fn with_priority(
        vertex: u32,
        priority: OrderedFloat<f32>,
        distance: OrderedFloat<f32>,
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
