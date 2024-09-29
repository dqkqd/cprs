use algo::io::{reader::Reader, writer::Writer};
use std::{
    collections::{BTreeMap, VecDeque},
    io::{Read, Write},
};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let m: u64 = reader.read();
    let k: u64 = reader.read();
    let mut diaries = vec![(0, 0); n];
    for (d, a) in &mut diaries {
        *d = reader.read();
        *a = reader.read();
    }
    let until = diaries
        .iter()
        .map(|&(d, a)| (d, a))
        .collect::<BTreeMap<_, _>>();
    let mut entries = diaries
        .iter()
        .flat_map(|v| vec![v.0, v.0 + k - 1])
        .collect::<Vec<_>>();
    entries.sort_unstable();
    entries.dedup();
    entries.push(1e15 as u64);
    let mut ans = 0;
    let mut milk = VecDeque::new();
    let mut run = |l: u64, r: u64| {
        if r < l {
            return;
        }
        if let Some(&milk_until) = until.get(&l) {
            milk.push_back((l + k - 1, milk_until));
        }
        let range = r - l + 1;
        let r = r + k - 1;
        let maximum = range * m;
        let mut total = 0;
        let mut last_deadline = None;
        while let Some(&(deadline, amount)) = milk.back() {
            assert!(deadline <= r);
            if total >= maximum {
                break;
            }
            if deadline < l {
                break;
            }
            total += amount;
            last_deadline = Some(deadline);
            milk.pop_back();
        }
        let satisfactory = (total / m).min(range);
        ans += satisfactory;
        if total > maximum {
            assert!(last_deadline.is_some());
            milk.push_back((last_deadline.unwrap(), total - maximum));
        }
    };
    for w in entries.windows(2) {
        run(w[0], w[0]);
        run(w[0] + 1, w[1] - 1);
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
6
1 1 3
1 5
2 3 3
1 5
2 7
4 5 2
1 9
2 6
4 9
5 6
5 2 4
4 7
5 3
7 1
11 2
12 1
4 1 3
5 10
9 4
14 8
15 3
5 5 5
8 9
10 7
16 10
21 5
28 9

"#;
        let expected = r#"
3
3
4
5
10
6

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
5 2 4
4 7
5 3
7 1
11 2
12 1
"#;
        let expected = r#"
5
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
