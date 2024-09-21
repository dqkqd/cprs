use algo::{
    io::{reader::Reader, writer::Writer},
    misc::two_sat::TwoSat,
};
use std::io::{Read, Write};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let _: String = reader.read();
    let _: String = reader.read();
    let n: usize = reader.read();
    let m: usize = reader.read();
    let mut two_sat = TwoSat::new(n);
    for _ in 0..m {
        let a: i32 = reader.read();
        let b: i32 = reader.read();
        let _: char = reader.read();
        two_sat.add_clause(a.abs() - 1, a > 0, b.abs() - 1, b > 0);
    }
    match two_sat.solve() {
        Some(result) => {
            writeln!(writer, "s SATISFIABLE").unwrap();
            write!(writer, "v ").unwrap();
            for v in result
                .assignment
                .into_iter()
                .map(|v| if v { 1 } else { -1 })
                .enumerate()
                .map(|(index, v)| (index + 1) as i32 * v)
            {
                write!(writer, "{} ", v).unwrap();
            }
            writeln!(writer, "0").unwrap();
        }
        None => {
            writeln!(writer, "s UNSATISFIABLE").unwrap();
        }
    }
}
fn solve<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let testcases: usize = 1;
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
