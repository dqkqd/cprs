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
        let data = String::from_utf8(raw).unwrap();
        FromStr::from_str(&data).unwrap()
    }
    fn skip_whitespaces(&mut self) -> std::io::Result<usize> {
        skip_while(&mut self.buf_reader, |c| c.is_ascii_whitespace())
    }
    fn read_until_whitespace(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        take_while(&mut self.buf_reader, buf, |c| c.is_ascii_whitespace())
    }
}

fn take_while<R, P>(r: &mut BufReader<R>, buf: &mut Vec<u8>, predicate: P) -> std::io::Result<usize>
where
    R: Read,
    P: Fn(&u8) -> bool,
{
    let mut read = 0;
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(e) => return Err(e),
            };
            match available.iter().position(&predicate) {
                Some(i) if i > 0 => {
                    buf.extend_from_slice(&available[..=i - 1]);
                    (true, i)
                }
                _ => {
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

fn skip_while<R, P>(r: &mut BufReader<R>, predicate: P) -> std::io::Result<usize>
where
    R: Read,
    P: Fn(&u8) -> bool,
{
    let mut read = 0;
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(e) => return Err(e),
            };
            match available.iter().position(|c| !predicate(c)) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_int() {
        let source: &[u8] = b"123";
        let mut input = Reader::new(source);
        assert_eq!(input.read::<i16>(), 123);
    }

    #[test]
    fn test_read_string() {
        let source: &[u8] = b"abc";
        let mut input = Reader::new(source);
        assert_eq!(input.read::<String>(), "abc");
    }

    #[test]
    fn test_read_with_space() {
        let source: &[u8] = b"  123  abc ";
        let mut input = Reader::new(source);
        assert_eq!(input.read::<i16>(), 123);
        assert_eq!(input.read::<String>(), "abc");
    }

    #[test]
    fn test_read_with_tab() {
        let source: &[u8] = b" \t123\tabc ";
        let mut input = Reader::new(source);
        assert_eq!(input.read::<i16>(), 123);
        assert_eq!(input.read::<String>(), "abc");
    }

    #[test]
    fn test_read_with_endline() {
        let source: &[u8] = b" \n123\nabc\rxyz";
        let mut input = Reader::new(source);
        assert_eq!(input.read::<i16>(), 123);
        assert_eq!(input.read::<String>(), "abc");
        assert_eq!(input.read::<String>(), "xyz");
    }
}
