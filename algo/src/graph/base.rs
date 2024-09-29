use std::ops::Index;

#[derive(Debug)]
pub enum GraphKind {
    Directed,
    Undirected,
}

pub trait GraphBase {
    fn add_edge(&mut self, from: u32, to: u32);
    fn node_counts(&self) -> usize;
    fn neighbors(&self, u: u32) -> std::iter::Cloned<std::slice::Iter<'_, u32>>;
}

#[derive(Debug)]
pub struct Graph {
    kind: GraphKind,
    pub(super) adjacents: Vec<Vec<u32>>,
}

impl Index<usize> for Graph {
    type Output = Vec<u32>;
    fn index(&self, index: usize) -> &Vec<u32> {
        &self.adjacents[index]
    }
}

impl Graph {
    pub fn new_directed(node_counts: usize) -> Graph {
        Graph {
            kind: GraphKind::Directed,
            adjacents: vec![vec![]; node_counts],
        }
    }
    pub fn new_undirected(node_counts: usize) -> Graph {
        Graph {
            kind: GraphKind::Undirected,
            adjacents: vec![vec![]; node_counts],
        }
    }
}

impl GraphBase for Graph {
    fn add_edge(&mut self, from: u32, to: u32) {
        match self.kind {
            GraphKind::Directed => {
                self.adjacents[from as usize].push(to);
            }
            GraphKind::Undirected => {
                self.adjacents[from as usize].push(to);
                self.adjacents[to as usize].push(from);
            }
        }
    }

    fn node_counts(&self) -> usize {
        self.adjacents.len()
    }

    fn neighbors(&self, u: u32) -> std::iter::Cloned<std::slice::Iter<'_, u32>> {
        self.adjacents[u as usize].iter().cloned()
    }
}
