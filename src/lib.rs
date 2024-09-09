pub mod algorithm;
pub mod config;
pub mod graph;
use algorithm::initialization::PulseState;
use algorithm::recursion::pulse_recursion;
use graph::Graph;

pub fn run(graph: &Graph, start_node: usize, end_node: usize, time_constraint: u64) -> PulseState {
    let mut pulse_state = PulseState::new(graph, start_node, end_node, time_constraint);
    pulse_recursion(&mut pulse_state, start_node, 0, 0);
    pulse_state
}
