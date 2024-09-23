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
