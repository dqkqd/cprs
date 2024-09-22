use super::base::Graph;

pub trait Bridge {
    fn bridge(&self) -> DfsTree;
}

impl Bridge for Graph {
    fn bridge(&self) -> DfsTree {
        DfsTree::new(self, 0)
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
        let n = graph.size;
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
        let mut visited = vec![false; self.graph.size];
        let mut backedges_count = vec![-1; self.graph.size];
        self.dfs(self.root, &mut visited, &mut backedges_count);
        self.bridges = backedges_count
            .iter()
            .enumerate()
            .filter(|(_, &count)| count == 0)
            .map(|(node, _)| (node as u32, self.parent[node].unwrap()))
            .collect();
        self
    }

    fn dfs(&mut self, node: u32, visited: &mut [bool], backedges_count: &mut [i32]) {
        let node = node as usize;
        self.weight[node] += 1;
        visited[node] = true;

        for index in 0..self.graph[node].len() {
            let child = self.graph[node][index];
            if !visited[child as usize] {
                self.parent[child as usize] = Some(node as u32);
                self.height[child as usize] = self.height[node] + 1;
                self.dfs(child, visited, backedges_count);
                self.weight[node] += self.weight[child as usize];
                backedges_count[node] += backedges_count[child as usize];
            } else if self.height[node] > self.height[child as usize] {
                backedges_count[node] += 1;
            } else {
                backedges_count[node] -= 1;
            }
        }
    }
}
