use super::shortest_path::{
    calculate_resource_consumption, reconstruct_path, shortest_path, ShortestPathCriterion,
};
use crate::{
    algorithm::pruning::{
        BoundsPruningStrategy, CyclesPruningStrategy, DominancePruningStrategy,
        FeasibilityPruningStrategy, PruningStrategy,
    },
    graph::Graph,
};

#[derive(Copy, Debug, Clone)]
pub struct Label {
    pub distance: u64,
    pub time: u64,
}

#[derive(Debug)]
pub struct PulseState<'a> {
    pub graph: &'a Graph,
    pub start_node: usize,
    pub end_node: usize,
    pub time_constraint: u64,
    pub visited: Vec<bool>,
    pub labels: Vec<[Label; 3]>, // MemorySize = 3
    pub current_path: Vec<usize>,
    pub dual_bounds_distance: Vec<u64>,
    pub dual_bounds_time: Vec<u64>,
    pub primal_bound_distance: u64,
    pub primal_bound_time: u64,
    pub primal_bound_path: Vec<usize>,
    pub pruning_strategies: [Box<dyn PruningStrategy>; 4],
}

impl PulseState<'_> {
    pub fn new(
        graph: &Graph,
        start_node: usize,
        end_node: usize,
        time_constraint: u64,
    ) -> PulseState {
        // Assume that the graph is bidirectional, so no need to invert it
        let visited = vec![false; graph.num_nodes + 1];
        let labels = vec![
            [Label {
                distance: u64::MAX,
                time: u64::MAX
            }; 3];
            graph.num_nodes + 1
        ];
        let (dual_bounds_distance, _) =
            shortest_path(graph, end_node, ShortestPathCriterion::Distance);
        let (dual_bounds_time, backtracking_time) =
            shortest_path(graph, end_node, ShortestPathCriterion::Time);
        let primal_bound_path = reconstruct_path(end_node, start_node, &backtracking_time);
        let (primal_bound_distance, primal_bound_time) =
            calculate_resource_consumption(graph, &primal_bound_path);
        let pruning_strategies: [Box<dyn PruningStrategy>; 4] = [
            Box::new(CyclesPruningStrategy {}),
            Box::new(FeasibilityPruningStrategy {}),
            Box::new(BoundsPruningStrategy {}),
            Box::new(DominancePruningStrategy {}),
        ];

        PulseState {
            graph,
            start_node,
            end_node,
            time_constraint,
            visited,
            labels,
            current_path: vec![],
            dual_bounds_distance,
            dual_bounds_time,
            primal_bound_distance,
            primal_bound_time,
            primal_bound_path,
            pruning_strategies,
        }
    }
}
