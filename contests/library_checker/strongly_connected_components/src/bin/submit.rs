use algo::{
    graph::{
        base::{Graph, GraphBase},
        scc::Scc,
    },
    io::{reader::Reader, writer::Writer},
};
use std::io::{Read, Write};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let m: usize = reader.read();
    let mut g = Graph::new_directed(n);
    for _ in 0..m {
        let u: u32 = reader.read();
        let v: u32 = reader.read();
        g.add_edge(u, v);
    }
    let components = g.scc();
    writeln!(writer, "{}", components.size).unwrap();
    for id in 0..components.size {
        write!(writer, "{} ", components[id].len()).unwrap();
        for id in &components[id] {
            write!(writer, "{} ", id).unwrap();
        }
        writeln!(writer).unwrap();
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
        pub mod scc {
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
