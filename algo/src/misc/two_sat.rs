use crate::graph::base::Graph;

pub struct TwoSat {
    graph: Graph,
}

pub struct TwoSatResult {
    pub assignment: Vec<bool>,
}

impl TwoSat {
    pub fn new(size: usize) -> TwoSat {
        TwoSat {
            graph: Graph::new(size * 2),
        }
    }

    pub fn add_clause(&mut self, a: i32, var_a: bool, b: i32, var_b: bool) {
        let pos = |x, var_x| if var_x { 2 * x } else { 2 * x + 1 };
        let neg = |x, var_x| pos(x, var_x) ^ 1;
        self.graph.add_edge(neg(a, var_a), pos(b, var_b));
        self.graph.add_edge(neg(b, var_b), pos(a, var_a));
    }

    pub fn solve(self) -> Option<TwoSatResult> {
        let components = self.graph.scc().components();
        let assignment = components
            .ids
            .chunks_exact(2)
            .map_while(|w| (w[0] != w[1]).then_some(w[1] < w[0]))
            .collect::<Vec<_>>();
        if assignment.len() * 2 == components.ids.len() {
            Some(TwoSatResult { assignment })
        } else {
            None
        }
    }
}
