use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub data_file: String,
    pub number_of_arcs: usize,
    pub number_of_nodes: usize,
    pub time_constraint: u64,
    pub start_node: usize,
    pub end_node: usize,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        let mut data_file = String::new();
        let mut number_of_arcs = 0;
        let mut number_of_nodes = 0;
        let mut time_constraint = 0;
        let mut start_node = 0;
        let mut end_node = 0;

        for line in content.lines() {
            if let Some((key, value)) = line.split_once(':') {
                match key.trim() {
                    "DataFile" => data_file = value.trim().to_string(),
                    "Number of Arcs" => number_of_arcs = value.trim().parse()?,
                    "Number of Nodes" => number_of_nodes = value.trim().parse()?,
                    "Time Constraint" => time_constraint = value.trim().parse()?,
                    "Start Node" => start_node = value.trim().parse()?,
                    "End Node" => end_node = value.trim().parse()?,
                    _ => {}
                }
            }
        }

        Ok(Config {
            data_file,
            number_of_arcs,
            number_of_nodes,
            time_constraint,
            start_node,
            end_node,
        })
    }
}
