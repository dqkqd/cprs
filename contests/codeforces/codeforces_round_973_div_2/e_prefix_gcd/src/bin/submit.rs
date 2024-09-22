use algo::{
    external::num_integer::gcd::Gcd,
    io::{reader::Reader, writer::Writer},
};
use std::io::{Read, Write};
fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let mut v = vec![0; n];
    for x in &mut v {
        *x = reader.read::<i32>();
    }
    let mut sum = 0;
    while !v.is_empty() {
        let min = *v.iter().min().unwrap();
        sum += min as i64;
        let p = v.iter().position(|&x| x == min).unwrap();
        v.remove(p);
        for x in &mut v {
            *x = x.gcd(&min);
        }
        if v.last().is_some_and(|x| v.iter().all(|e| e == x)) {
            break;
        }
    }
    for x in v {
        sum += x as i64;
    }
    writeln!(writer, "{}", sum).unwrap();
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
        pub mod num_integer {
            pub mod gcd {
                pub trait Gcd {
                    fn gcd(&self, other: &Self) -> Self;
                }
                macro_rules! impl_integer_for_isize {
                    ($ T : ty) => {
                        impl Gcd for $T {
                            fn gcd(&self, other: &Self) -> Self {
                                let mut m = *self;
                                let mut n = *other;
                                if m == 0 || n == 0 {
                                    return (m | n).abs();
                                }
                                let shift = (m | n).trailing_zeros();
                                if m == Self::MIN || n == Self::MIN {
                                    return 1 << shift;
                                }
                                m = m.abs();
                                n = n.abs();
                                m >>= m.trailing_zeros();
                                n >>= n.trailing_zeros();
                                while m != n {
                                    if m > n {
                                        m -= n;
                                        m >>= m.trailing_zeros();
                                    } else {
                                        n -= m;
                                        n >>= n.trailing_zeros();
                                    }
                                }
                                m << shift
                            }
                        }
                    };
                }
                impl_integer_for_isize!(i8);
                impl_integer_for_isize!(i16);
                impl_integer_for_isize!(i32);
                impl_integer_for_isize!(i64);
                impl_integer_for_isize!(i128);
                impl_integer_for_isize!(isize);
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
5
3
4 2 2
2
6 3
3
10 15 6
5
6 42 12 52 20
4
42 154 231 66

"#;
        let expected = r#"
6
6
9
14
51

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
