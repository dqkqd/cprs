use std::ops::Index;

#[derive(Debug)]
pub struct Graph {
    pub(super) adj: Vec<Vec<u32>>,
    pub(super) size: usize,
}

impl Index<usize> for Graph {
    type Output = Vec<u32>;
    fn index(&self, index: usize) -> &Vec<u32> {
        &self.adj[index]
    }
}

impl Graph {
    pub fn new(size: usize) -> Graph {
        Graph {
            adj: vec![vec![]; size],
            size,
        }
    }

    pub fn add_edge(&mut self, u: u32, v: u32) {
        self.adj[u as usize].push(v);
    }
}
