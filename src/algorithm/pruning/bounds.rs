use super::PruningStrategy;
use crate::algorithm::initialization::PulseState;

pub struct BoundsPruningStrategy {}

impl PruningStrategy for BoundsPruningStrategy {
    fn prune(&self, state: &PulseState, current_node: usize) -> bool {
        false
    }
}
