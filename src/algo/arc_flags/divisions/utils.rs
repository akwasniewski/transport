use std::cmp::Ordering;

use ordered_float::OrderedFloat;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct QueueItemSource {
    pub vertex: u32,
    pub source: u32,
    pub distance: OrderedFloat<f32>,
}
impl QueueItemSource {
    pub fn new(vertex: u32, source: u32, cost: OrderedFloat<f32>) -> Self {
        Self {
            vertex,
            source,
            distance: cost,
        }
    }

}

impl PartialOrd for QueueItemSource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItemSource {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance) 
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}
