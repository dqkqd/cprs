use algo::io::{reader::Reader, writer::Writer};
use std::io::{Read, Write};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let k: usize = reader.read();
    let a: Vec<usize> = reader.read_vec(n);
    let max = a.iter().max().unwrap();
    let sum: usize = a.iter().sum();
    dbg!(max, sum);
    for size in (1..=n).rev() {
        let v = (sum + k) / size;
        if v * size >= sum && v >= *max {
            writeln!(writer, "{}", size).unwrap();
            break;
        }
    }
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
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_01() {
        let input = r#"
9
3 1
3 2 2
5 4
2 6 1 2 4
2 100
1410065408 10000000000
10 8
7 4 6 6 9 3 10 2 8 7
2 12
2 2
2 70
0 1
1 0
1
3 0
2 1 2
3 1
0 3 3

"#;
        let expected = r#"
2
3
1
7
2
2
1
1
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
