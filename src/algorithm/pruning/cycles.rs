use super::PruningStrategy;
use crate::algorithm::initialization::PulseState;

pub struct CyclesPruningStrategy {}

impl PruningStrategy for CyclesPruningStrategy {
    fn prune(
        &self,
        state: &PulseState,
        current_node: usize,
        _accumulated_distance: u64,
        _accumulated_time: u64,
    ) -> bool {
        state.visited[current_node]
    }
}
