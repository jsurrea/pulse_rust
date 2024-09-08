pub mod algorithm;
pub mod config;
pub mod graph;
use algorithm::initialization::PulseState;
use graph::{Graph, Node};

pub fn run(graph: &Graph, start_node: usize, end_node: usize, time_constraint: u64) {
    let distance: u64 = 0;
    let time: u64 = 0;
    let path: Vec<Node> = vec![];
    let pulse_state = PulseState::new(graph, end_node);
    //algorithm::pulse_recursive(graph, start_node, distance, time);
}
