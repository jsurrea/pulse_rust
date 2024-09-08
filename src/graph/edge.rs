#[derive(Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub distance: u64,
    pub time: u64,
}

impl Clone for Edge {
    fn clone(&self) -> Self {
        Edge {
            from: self.from,
            to: self.to,
            distance: self.distance,
            time: self.time,
        }
    }
}
