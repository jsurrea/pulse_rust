use super::PruningStrategy;
use crate::algorithm::initialization::PulseState;

pub struct BoundsPruningStrategy {}

impl PruningStrategy for BoundsPruningStrategy {
    fn prune(
        &self,
        state: &PulseState,
        current_node: usize,
        accumulated_distance: u64,
        _accumulated_time: u64,
    ) -> bool {
        let best_possible_distance =
            accumulated_distance + state.dual_bounds_distance[current_node];
        state.primal_bound_distance < best_possible_distance
    }
}
