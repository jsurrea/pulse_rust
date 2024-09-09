use super::super::initialization::PulseState;

pub fn pulse_recursion(
    pulse_state: &mut PulseState,
    current_node: usize,
    accumulated_distance: u64,
    accumulated_time: u64,
) {
    if pulse_state
        .pruning_strategies
        .iter()
        .any(|strategy| strategy.prune(pulse_state, current_node))
    {
        return;
    }

    pulse_state.visited[current_node] = true;
    pulse_state.current_path.push(current_node);

    if current_node == pulse_state.end_node {
        update_primal_bound(pulse_state, accumulated_distance, accumulated_time);
    } else {
        for edge in &pulse_state.graph.adj_list[current_node] {
            let accumulated_distance = accumulated_distance + edge.distance;
            let accumulated_time = accumulated_time + edge.time;
            let next_node = edge.to;
            pulse_recursion(
                pulse_state,
                next_node,
                accumulated_distance,
                accumulated_time,
            )
        }
    }

    pulse_state.current_path.pop();
    pulse_state.visited[current_node] = false;
}

fn update_primal_bound(
    pulse_state: &mut PulseState,
    accumulated_distance: u64,
    accumulated_time: u64,
) {
    if accumulated_distance <= pulse_state.primal_bound_distance
        && accumulated_time <= pulse_state.time_constraint
    {
        println!(
            "Found a new primal bound: distance = {}, time = {}",
            accumulated_distance, accumulated_time
        );
        pulse_state.primal_bound_distance = accumulated_distance;
        pulse_state.primal_bound_time = accumulated_time;
        pulse_state.primal_bound_path = pulse_state.current_path.clone();
    }
}
