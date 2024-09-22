use algo::{
    graph::{base::Graph, bridge::Bridge, scc::Scc},
    io::{reader::Reader, writer::Writer},
};
use std::io::{Read, Write};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let m: usize = reader.read();
    let c: u64 = reader.read();
    let mut graph = Graph::new(n);
    for _ in 0..m {
        let u: u32 = reader.read();
        let v: u32 = reader.read();
        graph.add_edge(u - 1, v - 1);
        graph.add_edge(v - 1, u - 1);
    }
    let calc_x_y = |x: u64| -> u64 {
        if x as usize > n {
            0
        } else {
            let y = n as u64 - x;
            x * x + y * y
        }
    };
    let components = graph.scc();
    if components.size == 1 {
        let tree = graph.bridge(0);
        if tree.bridges.is_empty() {
            writeln!(writer, "{}", -1).unwrap();
            return;
        }
    }
    let c_contrib = c * (components.size as u64 - 1);
    let mut weights = vec![false; n + 1];
    weights[0] = true;
    for index in 0..components.inner.len() {
        let size = components[index].len();
        for i in (0..n - size).rev() {
            if weights[i] {
                weights[i + size] = true;
            }
        }
    }
    let mut x_y_contrib = u64::MAX;
    for (index, w) in weights.iter().enumerate() {
        if *w {
            x_y_contrib = x_y_contrib.min(calc_x_y(index as u64));
        }
    }
    for (id, component) in components.inner.iter().enumerate() {
        let mut weights = vec![false; n + 1];
        weights[0] = true;
        for index in 0..components.inner.len() {
            if index == id {
                continue;
            }
            let size = components[index].len();
            for i in (0..n - size).rev() {
                if weights[i] {
                    weights[i + size] = true;
                }
            }
        }
        let tree = graph.bridge(component[0]);
        for (u, _) in tree.bridges {
            let size_u = tree.weight[u as usize] as u64;
            for (index, w) in weights.iter().enumerate() {
                if *w {
                    x_y_contrib = x_y_contrib.min(calc_x_y(index as u64 + size_u));
                }
            }
        }
    }
    let ans = x_y_contrib + c_contrib;
    writeln!(writer, "{}", ans).unwrap();
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
    pub mod graph {
        pub mod base {
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
        }
        pub mod bridge {
            use super::base::Graph;
            pub trait Bridge {
                fn bridge(&self, root: u32) -> DfsTree;
            }
            impl Bridge for Graph {
                fn bridge(&self, root: u32) -> DfsTree {
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
        }
        pub mod scc {
            use super::base::Graph;
            use std::{cmp::min, ops::Index};
            pub trait Scc {
                fn scc(&self) -> Components;
            }
            impl Scc for Graph {
                fn scc(&self) -> Components {
                    SccGraph::new(self).components()
                }
            }
            #[derive(Debug)]
            pub struct Components {
                pub ids: Vec<u32>,
                pub size: usize,
                pub inner: Vec<Vec<u32>>,
            }
            impl Index<usize> for Components {
                type Output = Vec<u32>;
                fn index(&self, index: usize) -> &Vec<u32> {
                    &self.inner[index]
                }
            }
            impl Components {
                fn new(component_ids: Vec<u32>, size: usize) -> Components {
                    let mut inner = vec![vec![]; size];
                    for (node, id) in component_ids.iter().enumerate() {
                        inner[*id as usize].push(node as u32);
                    }
                    Components {
                        ids: component_ids,
                        size,
                        inner,
                    }
                }
                pub fn is_same(&self, u: u32, v: u32) -> bool {
                    self.ids[u as usize] == self.ids[v as usize]
                }
            }
            #[derive(Debug)]
            pub(super) struct SccGraph<'g> {
                graph: &'g Graph,
                visited: Vec<u32>,
                visiting: Vec<bool>,
                low: Vec<u32>,
                order: Vec<Option<u32>>,
                current_order: u32,
                component_ids: Vec<u32>,
                num_components: u32,
            }
            impl<'g> SccGraph<'g> {
                pub(super) fn new(graph: &'g Graph) -> SccGraph {
                    let size = graph.size;
                    SccGraph {
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
                pub(super) fn components(mut self) -> Components {
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
                    self.visited.push(node as u32);
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
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_01() {
        let input = r#"
2
4 6 5
4 3
2 3
2 4
1 2
4 1
3 1
6 6 2
1 4
2 5
3 6
1 5
3 5
6 5
"#;
        let expected = r#"
-1
20
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
    #[test]
    fn case_02() {
        let input = r#"
1
6 5 7
1 4
2 5
3 6
3 5
6 5
"#;
        let expected = r#"
25
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
    #[test]
    fn case_03() {
        let input = r#"
1
7 5 4
1 4
3 6
3 5
6 5
2 7
"#;
        let expected = r#"
33
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
    #[test]
    fn case_04() {
        let input = r#"
1
6 6 1
1 2
2 3
3 1
4 5
5 6
6 4
"#;
        let expected = r#"
19
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
