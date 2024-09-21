use std::{cmp::min, ops::Index};

use super::base::Graph;

#[derive(Debug)]
pub struct Components {
    pub ids: Vec<i32>,
    pub size: usize,
    pub inner: Vec<Vec<i32>>,
}

impl Index<usize> for Components {
    type Output = Vec<i32>;
    fn index(&self, index: usize) -> &Vec<i32> {
        &self.inner[index]
    }
}

impl Components {
    fn new(component_ids: Vec<i32>, size: usize) -> Components {
        let mut inner = vec![vec![]; size];
        for (node, id) in component_ids.iter().enumerate() {
            inner[*id as usize].push(node as i32);
        }
        Components {
            ids: component_ids,
            size,
            inner,
        }
    }

    pub fn is_same(&self, u: i32, v: i32) -> bool {
        self.ids[u as usize] == self.ids[v as usize]
    }
}

#[derive(Debug)]
pub struct SCC {
    graph: Graph,

    visited: Vec<i32>,
    visiting: Vec<bool>,

    low: Vec<i32>,

    order: Vec<Option<i32>>,
    current_order: i32,

    component_ids: Vec<i32>,
    num_components: i32,
}

impl SCC {
    pub fn new(graph: Graph) -> SCC {
        let size = graph.size;
        SCC {
            graph,

            visited: Vec::new(),
            visiting: vec![false; size],

            low: vec![0; size],

            order: vec![None; size],
            current_order: 0,

            component_ids: vec![0; size],
            num_components: 0,
        }
    }

    pub fn components(mut self) -> Components {
        self.tarjan();
        Components::new(self.component_ids, self.num_components as usize)
    }

    fn tarjan(&mut self) {
        for node in 0..self.graph.size {
            if self.order[node].is_none() {
                self.dfs(node);
            }
        }
        for component_id in self.component_ids.iter_mut() {
            *component_id = self.num_components - *component_id - 1;
        }
    }

    fn dfs(&mut self, node: usize) {
        self.current_order += 1;
        self.visited.push(node as i32);
        self.visiting[node] = true;
        self.order[node] = Some(self.current_order);
        self.low[node] = self.current_order;

        for i in 0..self.graph[node].len() {
            let neighbor = self.graph[node][i] as usize;
            match self.order[neighbor] {
                None => {
                    self.dfs(neighbor);
                    self.low[node] = min(self.low[node], self.low[neighbor]);
                }
                Some(order) => {
                    if self.visiting[neighbor] {
                        self.low[node] = min(self.low[node], order);
                    }
                }
            }
        }

        if Some(self.low[node]) == self.order[node] {
            while let Some(n) = self.visited.last() {
                let n = *n as usize;
                self.visited.pop();
                self.visiting[n] = false;
                self.component_ids[n] = self.num_components;
                if n == node {
                    break;
                }
            }
            self.num_components += 1;
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn simple() {
        let mut g = Graph::new(10);
        g.add_edge(0, 1);
        g.add_edge(0, 7);
        g.add_edge(1, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 1);
        g.add_edge(2, 5);
        g.add_edge(3, 2);
        g.add_edge(3, 4);
        g.add_edge(4, 9);
        g.add_edge(5, 3);
        g.add_edge(5, 6);
        g.add_edge(5, 9);
        g.add_edge(6, 2);
        g.add_edge(7, 0);
        g.add_edge(7, 6);
        g.add_edge(7, 8);
        g.add_edge(8, 6);
        g.add_edge(8, 9);
        g.add_edge(9, 4);
        let components = g.scc().components();

        assert_eq!(components.size, 4);

        assert!(components.is_same(0, 7));
        assert_eq!(components.ids[0], 0);

        assert_eq!(components.ids[8], 1);

        assert!(components.is_same(1, 2));
        assert!(components.is_same(1, 3));
        assert!(components.is_same(1, 5));
        assert!(components.is_same(1, 6));
        assert_eq!(components.ids[1], 2);

        assert!(components.is_same(4, 9));
        assert_eq!(components.ids[4], 3);
    }
}
