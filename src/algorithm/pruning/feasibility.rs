use super::PruningStrategy;
use crate::algorithm::initialization::PulseState;

pub struct FeasibilityPruningStrategy {}

impl PruningStrategy for FeasibilityPruningStrategy {
    fn prune(&self, state: &PulseState) -> bool {
        false
    }
}
