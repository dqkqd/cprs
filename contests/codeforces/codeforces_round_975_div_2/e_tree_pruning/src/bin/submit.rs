use algo::{
    graph::base::Graph,
    io::{reader::Reader, writer::Writer},
};
use std::io::{Read, Write};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let mut graph = Graph::new(n);
    for _ in 0..n - 1 {
        let u: u32 = reader.read();
        let v: u32 = reader.read();
        graph.add_edge(u - 1, v - 1);
        graph.add_edge(v - 1, u - 1);
    }
    let mut heights = vec![0; n];
    let mut max_children_heights = vec![0; n];
    let mut parents = vec![None; n];
    let mut is_leaf = vec![true; n];
    let mut dfs = rfn!(|f, node: usize, parent: Option<usize>| {
        parents[node] = parent;
        for &child in &graph[node] {
            let child = child as usize;
            if Some(child) == parent {
                continue;
            }
            heights[child] = heights[node] + 1;
            max_children_heights[child] = heights[child];
            is_leaf[node] = false;
            f(child, Some(node));
            max_children_heights[node] =
                max_children_heights[node].max(max_children_heights[child]);
        }
    });
    dfs(0, None);
    let mut leaves: Vec<(u32, usize)> = Vec::new();
    let mut height_cnt = vec![0; n];
    for (index, h) in heights.iter().enumerate() {
        if is_leaf[index] {
            leaves.push((*h, index));
        }
        height_cnt[*h as usize] += 1;
    }
    leaves.sort_unstable();
    leaves.reverse();
    let mut removed = vec![false; n];
    let mut removed_cnt = 0;
    let mut height_cnt_sum = 0;
    let mut ans = usize::MAX;
    for (h, cnt) in height_cnt.iter().enumerate() {
        while let Some((leaf_height, mut node)) = leaves.last().cloned() {
            if leaf_height >= h as u32 {
                break;
            }
            while !removed[node] && max_children_heights[node] <= leaf_height {
                removed[node] = true;
                removed_cnt += 1;
                let parent = parents[node];
                if parent.is_none() {
                    break;
                }
                node = parent.unwrap();
            }
            leaves.pop();
        }
        height_cnt_sum += cnt;
        let tree_size_after = height_cnt_sum - removed_cnt;
        let need_remove = n - tree_size_after;
        ans = ans.min(need_remove);
    }
    writeln!(writer, "{}", ans).unwrap();
}
fn solve<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let testcases: usize = reader.read();
    for case in 0..testcases {
        eprintln!("Solve case {}", case + 1);
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
4
7
1 2
1 3
2 4
2 5
4 6
4 7
7
1 2
1 3
1 4
2 5
3 6
5 7
15
12 9
1 6
6 14
9 11
8 7
3 5
13 5
6 10
13 15
13 6
14 12
7 2
8 1
1 4
8
1 2
2 3
2 4
2 5
5 6
6 7
7 8
"#;
        let expected = r#"
2
2
5
2
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
