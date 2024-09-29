use super::base::{Graph, GraphBase};
use crate::rfn;
use std::ops::Index;

pub trait Scc {
    fn scc(&self) -> Components;
}

impl Scc for Graph {
    fn scc(&self) -> Components {
        SccGraph { graph: self }.components()
    }
}

#[derive(Debug)]
pub struct Components {
    pub ids: Vec<usize>,
    pub size: usize,
    pub inner: Vec<Vec<usize>>,
}

impl Index<usize> for Components {
    type Output = Vec<usize>;
    fn index(&self, index: usize) -> &Vec<usize> {
        &self.inner[index]
    }
}

impl Components {
    fn new(component_ids: Vec<usize>, size: usize) -> Components {
        let mut inner = vec![vec![]; size];
        for (node, id) in component_ids.iter().enumerate() {
            inner[*id].push(node);
        }
        Components {
            ids: component_ids,
            size,
            inner,
        }
    }

    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.ids[u] == self.ids[v]
    }
}

#[derive(Debug)]
pub(super) struct SccGraph<'g> {
    graph: &'g Graph,
}

impl<'g> SccGraph<'g> {
    pub(super) fn components(mut self) -> Components {
        let (num_components, component_ids) = self.tarjan();

        // reorder
        let component_ids = component_ids
            .into_iter()
            .map(|id| num_components - id - 1)
            .collect();

        Components::new(component_ids, num_components)
    }

    fn tarjan(&mut self) -> (usize, Vec<usize>) {
        let size = self.graph.node_counts();
        let mut visited = Vec::new();
        let mut visiting = vec![false; size];
        let mut low = vec![0; size];
        let mut order = vec![None; size];
        let mut current_order = 0;

        let mut component_ids = vec![0; size];
        let mut num_components = 0;

        let mut dfs = rfn!(|dfs, node: usize, entry: bool| {
            if entry && order[node].is_some() {
                return;
            }

            current_order += 1;
            visited.push(node as u32);
            visiting[node] = true;
            order[node] = Some(current_order);
            low[node] = current_order;

            for neighbor in self.graph.neighbors(node as u32) {
                let neighbor = neighbor as usize;
                match order[neighbor] {
                    None => {
                        dfs(neighbor, false);
                        low[node] = low[node].min(low[neighbor]);
                    }
                    Some(order) => {
                        if visiting[neighbor] {
                            low[node] = low[node].min(order);
                        }
                    }
                }
            }

            if Some(low[node]) == order[node] {
                while let Some(n) = visited.last() {
                    let n = *n as usize;
                    visited.pop();
                    visiting[n] = false;
                    component_ids[n] = num_components;
                    if n == node {
                        break;
                    }
                }
                num_components += 1;
            }
        });

        for node in 0..self.graph.node_counts() {
            dfs(node, true);
        }

        (num_components, component_ids)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn simple() {
        let mut g = Graph::new_directed(10);
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
        let components = g.scc();

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
