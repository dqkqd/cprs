use algo::{
    graph::dsu::DSU,
    io::{reader::Reader, writer::Writer},
};
use std::io::{Read, Write};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n = reader.read::<usize>();
    let q = reader.read::<usize>();
    let mut dsu = DSU::new(n);
    for _ in 0..q {
        let t = reader.read::<u8>();
        let u = reader.read::<u32>();
        let v = reader.read::<u32>();
        if t == 0 {
            dsu.merge(u, v);
        } else {
            if dsu.is_same(u, v) {
                writeln!(writer, "1")
            } else {
                writeln!(writer, "0")
            }
            .unwrap();
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
        pub mod dsu {
            use std::cell::Cell;
            pub struct DSU {
                parent: Vec<Cell<u32>>,
                size: Vec<u32>,
            }
            impl DSU {
                pub fn new(size: usize) -> DSU {
                    DSU {
                        parent: (0..size as u32).map(Cell::new).collect(),
                        size: vec![1; size],
                    }
                }
                pub fn get(&self, u: u32) -> u32 {
                    let mut u = u;
                    while u != self.parent[u as usize].get() {
                        let p = self.parent[u as usize].get();
                        let g = self.parent[p as usize].get();
                        self.parent[u as usize].set(g);
                        u = g;
                    }
                    u
                }
                pub fn size(&self, u: u32) -> u32 {
                    self.size[self.get(u) as usize]
                }
                pub fn is_same(&self, u: u32, v: u32) -> bool {
                    self.get(u) == self.get(v)
                }
                pub fn merge(&mut self, u: u32, v: u32) -> bool {
                    let mut u = self.get(u);
                    let mut v = self.get(v);
                    if u == v {
                        return false;
                    }
                    if self.size[u as usize] < self.size[v as usize] {
                        (u, v) = (v, u);
                    }
                    self.parent[v as usize].set(u);
                    self.size[u as usize] += self.size[v as usize];
                    true
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
