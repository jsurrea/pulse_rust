use pulse::config::Config;

fn main() {
    let config = Config::from_file("config.txt").expect("Failed to read config file");
    println!("{:?}", config);
}
