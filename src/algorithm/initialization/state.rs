use super::shortest_path::{shortest_path, ShortestPathCriterion};
use crate::graph::Graph;

#[derive(Debug)]
pub struct PulseState {
    pub dual_bounds_distance: Vec<u64>,
    pub dual_bounds_time: Vec<u64>,
}

impl PulseState {
    pub fn new(graph: &Graph, end_node_id: usize) -> PulseState {
        // Assume that the graph is bidirectional, so no need to invert it
        let dual_bounds_distance =
            shortest_path(graph, end_node_id, ShortestPathCriterion::Distance);
        let dual_bounds_time = shortest_path(graph, end_node_id, ShortestPathCriterion::Time);
        PulseState {
            dual_bounds_distance,
            dual_bounds_time,
        }
    }
}
