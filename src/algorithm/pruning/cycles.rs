use super::PruningStrategy;
use crate::algorithm::initialization::PulseState;

pub struct CyclesPruningStrategy {}

impl PruningStrategy for CyclesPruningStrategy {
    fn prune(&self, state: &PulseState, current_node: usize) -> bool {
        state.visited[current_node]
    }
}
