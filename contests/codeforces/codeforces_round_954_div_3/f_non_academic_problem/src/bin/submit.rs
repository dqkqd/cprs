use algo::{
    graph::{
        base::{Graph, GraphBase},
        bridge::Bridge,
    },
    io::{reader::Reader, writer::Writer},
};
use std::io::{Read, Write};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n = reader.read::<usize>();
    let m = reader.read::<usize>();
    let mut graph = Graph::new_undirected(n);
    for _ in 0..m {
        let u = reader.read::<u32>();
        let v = reader.read::<u32>();
        graph.add_edge(u - 1, v - 1);
    }
    let tree = graph.bridges(0);
    let mut ans = n * (n - 1) / 2;
    for (u, _) in tree.bridges {
        let u = u as usize;
        let size = tree.weight[u] as usize;
        let left = size * (size - 1) / 2;
        let right = (n - size) * (n - size - 1) / 2;
        ans = ans.min(left + right);
    }
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
        }
        pub mod bridge {
            use super::base::{Graph, GraphBase};
            use crate::rfn;
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
        }
    }
    pub mod io {
        pub mod reader {
            use std::{
                io::{BufRead, BufReader, Read},
                str::FromStr,
            };
            pub struct Reader<R: Read> {
                buf_reader: BufReader<R>,
            }
            impl<R: Read> Reader<R> {
                pub fn new(inner: R) -> Reader<R> {
                    Reader {
                        buf_reader: BufReader::new(inner),
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
                pub fn read_vec<T>(&mut self, n: usize) -> Vec<T>
                where
                    T: FromStr,
                    <T as FromStr>::Err: ::std::fmt::Debug,
                {
                    let mut v = Vec::with_capacity(n);
                    for _ in 0..n {
                        let x = self.read::<T>();
                        v.push(x);
                    }
                    v
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
                        buf_writer: BufWriter::new(inner),
                    }
                }
                pub fn write_vec<T>(&mut self, v: &[T])
                where
                    T: std::fmt::Display,
                {
                    if !v.is_empty() {
                        let (last, rest) = v.split_last().unwrap();
                        for e in rest {
                            write!(self, "{} ", e).unwrap();
                        }
                        writeln!(self, "{}", last).unwrap();
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
        pub mod recursive_closure {
            #[macro_export]
            macro_rules ! rfn { (|$ self_arg : ident $ (, $ arg_name : ident : $ arg_type : ty) * $ (,) ? | -> $ ret_type : ty $ body : block) => { { trait HideFn { fn call (& mut self , $ ($ arg_name : $ arg_type ,) *) -> $ ret_type ; } struct HideFnImpl < F : FnMut (& mut dyn HideFn , $ ($ arg_type ,) *) -> $ ret_type > (std :: cell :: UnsafeCell < F >) ; impl < F : FnMut (& mut dyn HideFn , $ ($ arg_type ,) *) -> $ ret_type > HideFn for HideFnImpl < F > { # [inline] fn call (& mut self , $ ($ arg_name : $ arg_type ,) *) -> $ ret_type { unsafe { (* self . 0 . get ()) (self , $ ($ arg_name ,) *) } } } let mut inner = HideFnImpl (std :: cell :: UnsafeCell :: new (# [inline] |$ self_arg : & mut dyn HideFn , $ ($ arg_name : $ arg_type ,) *| -> $ ret_type { let mut $ self_arg = |$ ($ arg_name : $ arg_type) ,*| $ self_arg . call ($ ($ arg_name ,) *) ; { $ body } })) ; # [inline] move |$ ($ arg_name : $ arg_type) ,*| -> $ ret_type { inner . call ($ ($ arg_name) ,*) } } } ; (|$ self_arg : ident $ (, $ arg_name : ident : $ arg_type : ty) * $ (,) ? | $ body : block) => { { trait HideFn { fn call (& mut self , $ ($ arg_name : $ arg_type ,) *) ; } struct HideFnImpl < F : FnMut (& mut dyn HideFn , $ ($ arg_type ,) *) > (std :: cell :: UnsafeCell < F >) ; impl < F : FnMut (& mut dyn HideFn , $ ($ arg_type ,) *) > HideFn for HideFnImpl < F > { # [inline] fn call (& mut self , $ ($ arg_name : $ arg_type ,) *) -> () { unsafe { (* self . 0 . get ()) (self , $ ($ arg_name ,) *) } } } let mut inner = HideFnImpl (std :: cell :: UnsafeCell :: new (# [inline] |$ self_arg : & mut dyn HideFn , $ ($ arg_name : $ arg_type ,) *| -> () { let mut $ self_arg = |$ ($ arg_name : $ arg_type) ,*| $ self_arg . call ($ ($ arg_name ,) *) ; { $ body } })) ; # [inline] move |$ ($ arg_name : $ arg_type) ,*| { inner . call ($ ($ arg_name) ,*) } } } }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_01() {
        let input = r#"
6
2 1
1 2
3 3
1 2
2 3
1 3
5 5
1 2
1 3
3 4
4 5
5 3
6 7
1 2
1 3
2 3
3 4
4 5
4 6
5 6
5 5
1 2
1 3
2 3
2 4
3 5
10 12
1 2
1 3
2 3
2 4
4 5
5 6
6 7
7 4
3 8
8 9
9 10
10 8

"#;
        let expected = r#"
0
3
4
6
6
21

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
