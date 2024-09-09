use pulse::config::Config;
use pulse::graph::Graph;
use std::time;

fn main() {
    let config = Config::from_file("config.txt").expect("Failed to read config file");
    let graph = Graph::from_config(&config).expect("Failed to create graph");
    let start = time::Instant::now();
    let pulse_state = pulse::run(
        &graph,
        config.start_node,
        config.end_node,
        config.time_constraint,
    );
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Optimal Solution:");
    println!("Distance: {}", pulse_state.primal_bound_distance);
    println!("Time: {}", pulse_state.primal_bound_time);
    println!("Path: {:?}", pulse_state.primal_bound_path);
}
