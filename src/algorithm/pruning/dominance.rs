use super::PruningStrategy;
use crate::algorithm::initialization::PulseState;

pub struct DominancePruningStrategy {}

impl PruningStrategy for DominancePruningStrategy {
    fn prune(&self, state: &PulseState) -> bool {
        false
    }
}
