use super::{Edge, Node};
use crate::config::Config;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Graph {
    pub num_nodes: usize,
    pub num_edges: usize,
    pub node_list: Vec<Node>,
    pub adj_list: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn from_config(config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut graph = Graph {
            num_nodes: config.number_of_nodes,
            num_edges: config.number_of_arcs,
            node_list: Vec::new(),
            adj_list: vec![Vec::new(); config.number_of_nodes + 1],
        };

        graph.init_nodes();
        graph.init_edges(&config.data_file)?;
        Ok(graph)
    }

    fn init_nodes(&mut self) {
        for i in 0..(self.num_nodes + 1) {
            self.node_list.push(Node { id: i });
        }
    }

    fn init_edges(&mut self, data_file: &str) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(data_file)?;
        for (index, line) in content.lines().enumerate() {
            if index == 0 {
                // Skip the first line (header)
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let from: usize = parts[1].parse()?;
                let to: usize = parts[2].parse()?;
                let distance: u64 = parts[3].parse()?;
                let time: u64 = parts[4].parse()?;
                self.adj_list[from].push(Edge {
                    from,
                    to,
                    distance,
                    time,
                });
            }
        }
        Ok(())
    }
}
