use super::PruningStrategy;
use crate::algorithm::initialization::PulseState;

pub struct DominancePruningStrategy {}

impl PruningStrategy for DominancePruningStrategy {
    fn prune(
        &self,
        state: &PulseState,
        current_node: usize,
        accumulated_distance: u64,
        accumulated_time: u64,
    ) -> bool {
        state.labels[current_node]
            .iter()
            .any(|label| label.distance <= accumulated_distance && label.time <= accumulated_time)
    }
}
