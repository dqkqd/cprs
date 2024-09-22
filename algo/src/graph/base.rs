use std::ops::Index;

#[derive(Debug)]
pub struct Graph {
    pub(super) adj: Vec<Vec<i32>>,
    pub(super) size: usize,
}

impl Index<usize> for Graph {
    type Output = Vec<i32>;
    fn index(&self, index: usize) -> &Vec<i32> {
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

    pub fn add_edge(&mut self, u: i32, v: i32) {
        self.adj[u as usize].push(v);
    }
}
