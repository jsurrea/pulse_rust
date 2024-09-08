use pulse::config::Config;
use pulse::graph::Graph;

fn main() {
    let config = Config::from_file("config.txt").expect("Failed to read config file");
    let graph = Graph::from_config(&config).expect("Failed to create graph");
    println!("{:?}", config);
    println!("{:?}", graph);
}
