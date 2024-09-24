use algo::{
    io::{reader::Reader, writer::Writer},
    string::z::ZFunction,
};
use std::{
    collections::BTreeSet,
    io::{Read, Write},
};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let l: usize = reader.read();
    let r: usize = reader.read();
    let s: String = reader.read();
    let z = s.z_function();
    let mut non_zero_indices = BTreeSet::new();
    let mut indices = vec![vec![]; n + 1];
    for (index, &value) in z.iter().enumerate() {
        indices[value as usize].push(index);
        if value != 0 {
            non_zero_indices.insert(index);
        }
    }
    let mut count = vec![0; n + 1];
    for i in 1..n + 1 {
        count[i] += 1;
        let mut index = i;
        while let Some(next_index) = non_zero_indices.range(index..n).next() {
            index = next_index + i;
            count[i] += 1;
        }
        for value in &indices[i] {
            non_zero_indices.remove(value);
        }
    }
    let mut ans = vec![0; n + 1];
    for (index, value) in count.iter().enumerate().skip(1) {
        ans[*value as usize] = index;
    }
    for i in (0..ans.len() - 1).rev() {
        ans[i] = ans[i].max(ans[i + 1]);
    }
    ans[1] = n;
    writer.write_vec(&ans[l..r + 1]);
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
    pub mod string {
        pub mod z {
            pub trait ZFunction {
                fn z_function(&self) -> Vec<u32>;
            }
            impl ZFunction for String {
                fn z_function(&self) -> Vec<u32> {
                    let mut z = vec![0; self.len()];
                    let mut l = 0;
                    let mut r = 0;
                    let bytes = self.as_bytes();
                    for i in 1..bytes.len() {
                        if i < r {
                            z[i] = (r - i).min(z[i - l]);
                        }
                        while i + z[i] < self.len() && bytes[z[i]] == bytes[i + z[i]] {
                            z[i] += 1;
                        }
                        if i + z[i] > r {
                            l = i;
                            r = i + z[i];
                        }
                    }
                    z.into_iter().map(|v| v as u32).collect()
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
7
3 1 3
aba
3 2 3
aaa
7 1 5
abacaba
9 1 6
abababcab
10 1 10
aaaaaaawac
9 1 9
abafababa
7 2 7
vvzvvvv

"#;
        let expected = r#"
3 1 0
1 1
7 3 1 1 0
9 2 2 2 0 0
10 3 2 1 1 1 1 1 0 0
9 3 2 1 1 0 0 0 0
2 2 1 1 1 0

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
