use crate::graph::Graph;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{backtrace, path};

#[derive(Debug)]
pub enum ShortestPathCriterion {
    Distance,
    Time,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapElement {
    cost: u64,
    node_id: usize,
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        // Invert the ordering to get a min-heap
        other.cost.cmp(&self.cost)
    }
}

pub fn shortest_path(
    graph: &Graph,
    start_node_id: usize,
    criterion: ShortestPathCriterion,
) -> (Vec<u64>, Vec<usize>) {
    let mut resource: Vec<u64> = vec![u64::MAX; graph.num_nodes + 1];
    let mut backtrack: Vec<usize> = vec![0; graph.num_nodes + 1];
    let mut heap = BinaryHeap::new();

    resource[start_node_id] = 0;
    heap.push(HeapElement {
        cost: 0,
        node_id: start_node_id,
    });

    while let Some(HeapElement { cost, node_id }) = heap.pop() {
        if resource[node_id] < cost {
            continue;
        }

        for edge in &graph.adj_list[node_id] {
            let edge_cost = match criterion {
                ShortestPathCriterion::Distance => edge.distance,
                ShortestPathCriterion::Time => edge.time,
            };

            let next_element = HeapElement {
                cost: cost + edge_cost,
                node_id: edge.to,
            };

            if next_element.cost < resource[edge.to] {
                resource[edge.to] = next_element.cost;
                backtrack[edge.to] = node_id;
                heap.push(next_element);
            }
        }
    }

    (resource, backtrack)
}

pub fn reconstruct_path(
    start_node_id: usize,
    end_node_id: usize,
    backtrack: &Vec<usize>,
) -> Vec<usize> {
    let mut current_node = end_node_id;
    let mut path = vec![];

    while current_node != start_node_id {
        path.push(current_node);
        current_node = backtrack[current_node];
    }

    path.push(start_node_id);
    path.reverse();

    path
}

pub fn calculate_resource_consumption(graph: &Graph, path: &Vec<usize>) -> (u64, u64) {
    let mut distance = 0;
    let mut time = 0;

    for i in 0..path.len() - 1 {
        let current_node = path[i];
        let next_node = path[i + 1];

        for edge in &graph.adj_list[current_node] {
            if edge.to == next_node {
                distance += edge.distance;
                time += edge.time;
                break;
            }
        }
    }

    (distance, time)
}
