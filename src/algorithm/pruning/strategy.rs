use crate::algorithm::initialization::PulseState;
use std::fmt::{Debug, Formatter, Result};

pub trait PruningStrategy {
    fn prune(
        &self,
        state: &PulseState,
        current_node: usize,
        accumulated_distance: u64,
        accumulated_time: u64,
    ) -> bool;
}

impl Debug for dyn PruningStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PruningStrategy")
    }
}
