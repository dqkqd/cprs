use crate::rfn;

use super::base::{Graph, GraphBase};

pub trait Bridge {
    fn bridges(&self, root: u32) -> DfsTree;
}

impl Bridge for Graph {
    fn bridges(&self, root: u32) -> DfsTree {
        DfsTree::new(self, root)
    }
}

#[derive(Debug)]
pub struct DfsTree<'g> {
    graph: &'g Graph,
    pub root: u32,
    pub parent: Vec<Option<u32>>,
    pub weight: Vec<u32>,
    pub height: Vec<u32>,
    pub bridges: Vec<(u32, u32)>,
}

impl<'g> DfsTree<'g> {
    pub(super) fn new(graph: &'g Graph, root: u32) -> DfsTree {
        let n = graph.node_counts();
        DfsTree {
            graph,
            root,
            parent: vec![None; n],
            height: vec![0; n],
            weight: vec![0; n],
            bridges: Vec::new(),
        }
        .bridges()
    }

    fn bridges(mut self) -> DfsTree<'g> {
        let size = self.graph.node_counts();
        let mut visited = vec![false; size];
        let mut backedges_count = vec![-1; size];
        let mut dfs = rfn!(|dfs, node: u32| {
            let node = node as usize;
            self.weight[node] += 1;
            visited[node] = true;

            for child in self.graph.neighbors(node as u32) {
                if !visited[child as usize] {
                    self.parent[child as usize] = Some(node as u32);
                    self.height[child as usize] = self.height[node] + 1;
                    dfs(child);
                    self.weight[node] += self.weight[child as usize];
                    backedges_count[node] += backedges_count[child as usize];
                } else if self.height[node] > self.height[child as usize] {
                    backedges_count[node] += 1;
                } else {
                    backedges_count[node] -= 1;
                }
            }
        });
        dfs(self.root);

        self.bridges = backedges_count
            .iter()
            .enumerate()
            .filter(|(_, &count)| count == 0)
            .map(|(node, _)| (node as u32, self.parent[node].unwrap()))
            .collect();
        self
    }
}
