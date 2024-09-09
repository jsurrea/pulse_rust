use super::PruningStrategy;
use crate::algorithm::initialization::PulseState;

pub struct DominancePruningStrategy {}

impl PruningStrategy for DominancePruningStrategy {
    fn prune(&self, state: &PulseState, current_node: usize) -> bool {
        false
    }
}
