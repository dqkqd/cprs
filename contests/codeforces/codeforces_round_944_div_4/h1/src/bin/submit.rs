use algo::{
    io::{reader::Reader, writer::Writer},
    misc::two_sat::TwoSat,
};
use std::io::{Read, Write};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let mut g: [Vec<i32>; 3] = core::array::from_fn(|_| vec![0; n]);
    for row in &mut g {
        for item in row {
            *item = reader.read();
        }
    }
    let mut graph = TwoSat::new(n);
    for (g1, g2, g3) in izip!(&g[0], &g[1], &g[2]) {
        graph.add_clause(g1.abs() - 1, *g1 > 0, g2.abs() - 1, *g2 > 0);
        graph.add_clause(g2.abs() - 1, *g2 > 0, g3.abs() - 1, *g3 > 0);
        graph.add_clause(g3.abs() - 1, *g3 > 0, g1.abs() - 1, *g1 > 0);
    }
    if graph.solve().is_some() {
        writeln!(writer, "YES").unwrap();
    } else {
        writeln!(writer, "NO").unwrap();
    }
}
fn solve<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let testcases: usize = reader.read();
    for _ in 0..testcases {
        solve_case(reader, writer);
    }
}
fn main() {
    let mut reader = Reader::new(std::io::stdin());
    let mut writer = Writer::new(std::io::stdout().lock());
    solve(&mut reader, &mut writer);
}
pub mod algo {
    pub mod external {
        pub mod itertools {
            #[macro_export]
            macro_rules ! izip { (@ closure $ p : pat => $ tup : expr) => { |$ p | $ tup } ; (@ closure $ p : pat => ($ ($ tup : tt) *) , $ _iter : expr $ (, $ tail : expr) *) => { $ crate :: izip ! (@ closure ($ p , b) => ($ ($ tup) *, b) $ (, $ tail) *) } ; ($ first : expr $ (,) *) => { std :: iter :: IntoIterator :: into_iter ($ first) } ; ($ first : expr , $ second : expr $ (,) *) => { $ crate :: izip ! ($ first) . zip ($ second) } ; ($ first : expr $ (, $ rest : expr) * $ (,) *) => { $ crate :: izip ! ($ first) $ (. zip ($ rest)) * . map ($ crate :: izip ! (@ closure a => (a) $ (, $ rest) *)) } ; }
        }
    }
    pub mod graph {
        pub mod base {
            use super::scc::SCC;
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
                pub fn add_edges(&mut self, edges: &[(i32, i32)]) {
                    for (u, v) in edges {
                        self.add_edge(*u, *v);
                    }
                }
                pub fn scc(self) -> SCC {
                    SCC::new(self)
                }
            }
        }
        pub mod scc {
            use super::base::Graph;
            use std::{cmp::min, ops::Index};
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
        }
    }
    pub mod io {
        pub mod reader {
            use std::{
                io::{BufRead, BufReader, Read},
                str::FromStr,
            };
            const DEFAULT_BUF_SIZE: usize = 1024;
            pub struct Reader<R: Read> {
                buf_reader: BufReader<R>,
            }
            impl<R: Read> Reader<R> {
                pub fn new(inner: R) -> Reader<R> {
                    Reader {
                        buf_reader: BufReader::with_capacity(DEFAULT_BUF_SIZE, inner),
                    }
                }
                pub fn read<T>(&mut self) -> T
                where
                    T: FromStr,
                    <T as FromStr>::Err: ::std::fmt::Debug,
                {
                    self.skip_whitespaces().unwrap();
                    let mut raw = Vec::new();
                    self.read_until_whitespace(&mut raw).unwrap();
                    if raw.last().is_some_and(|c| c.is_ascii_whitespace()) {
                        raw.pop();
                    }
                    let data = String::from_utf8(raw).unwrap();
                    FromStr::from_str(&data).unwrap()
                }
                fn skip_whitespaces(&mut self) -> std::io::Result<usize> {
                    skip_whitespaces(&mut self.buf_reader)
                }
                fn read_until_whitespace(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
                    read_until_whitespace(&mut self.buf_reader, buf)
                }
            }
            fn skip_whitespaces<R: BufRead + ?Sized>(r: &mut R) -> std::io::Result<usize> {
                let mut read = 0;
                loop {
                    let (done, used) = {
                        let available = match r.fill_buf() {
                            Ok(n) => n,
                            Err(e) => return Err(e),
                        };
                        match available.iter().position(|c| !c.is_ascii_whitespace()) {
                            Some(i) => (true, i),
                            None => (false, available.len()),
                        }
                    };
                    r.consume(used);
                    read += used;
                    if done || used == 0 {
                        return Ok(read);
                    }
                }
            }
            fn read_until_whitespace<R: BufRead + ?Sized>(
                r: &mut R,
                buf: &mut Vec<u8>,
            ) -> std::io::Result<usize> {
                let mut read = 0;
                loop {
                    let (done, used) = {
                        let available = match r.fill_buf() {
                            Ok(n) => n,
                            Err(e) => return Err(e),
                        };
                        match available.iter().position(|c| c.is_ascii_whitespace()) {
                            Some(i) => {
                                buf.extend_from_slice(&available[..=i]);
                                (true, i + 1)
                            }
                            None => {
                                buf.extend_from_slice(available);
                                (false, available.len())
                            }
                        }
                    };
                    r.consume(used);
                    read += used;
                    if done || used == 0 {
                        return Ok(read);
                    }
                }
            }
        }
        pub mod writer {
            use std::io::{BufWriter, Write};
            pub struct Writer<W: Write> {
                buf_writer: BufWriter<W>,
            }
            impl<W: Write> Writer<W> {
                pub fn new(inner: W) -> Writer<W> {
                    Writer {
                        buf_writer: BufWriter::with_capacity(1024, inner),
                    }
                }
            }
            impl<W: Write> Write for Writer<W> {
                fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                    self.buf_writer.write(buf)
                }
                fn flush(&mut self) -> std::io::Result<()> {
                    self.buf_writer.flush()
                }
            }
        }
    }
    pub mod misc {
        pub mod two_sat {
            use crate::algo::graph::base::Graph;
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
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_01() {
        let input = r#"
4
4
1 -2 -3 -2
-4 4 -1 -3
1 2 -2 4
2
1 2
-1 -2
2 -2
5
1 2 3 4 5
-2 3 -4 -5 -1
3 -5 1 2 2
6
1 3 -6 2 5 2
1 3 -2 -3 -6 -5
-2 -1 -3 2 3 1

"#;
        let expected = r#"
YES
NO
YES
NO

"#;
        let mut output = Vec::new();
        {
            let mut reader = Reader::new(input.as_bytes());
            let mut writer = Writer::new(&mut output);
            solve(&mut reader, &mut writer);
        }
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output.trim(), expected.trim());
    }
}
