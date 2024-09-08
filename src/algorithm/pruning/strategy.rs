use crate::algorithm::initialization::PulseState;

pub trait PruningStrategy {
    fn prune(&self, state: &PulseState) -> bool;
}
