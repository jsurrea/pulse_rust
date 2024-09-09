use super::PruningStrategy;
use crate::algorithm::initialization::PulseState;

pub struct FeasibilityPruningStrategy {}

impl PruningStrategy for FeasibilityPruningStrategy {
    fn prune(
        &self,
        state: &PulseState,
        current_node: usize,
        _accumulated_distance: u64,
        accumulated_time: u64,
    ) -> bool {
        let best_possible_time = accumulated_time + state.dual_bounds_time[current_node];
        state.time_constraint < best_possible_time
    }
}
