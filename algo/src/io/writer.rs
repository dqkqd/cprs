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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_string() {
        let mut writer = Writer::new(Vec::new());
        write!(writer, "123").unwrap();
        assert_eq!(writer.buf_writer.into_inner().unwrap(), b"123");
    }

    #[test]
    fn test_write_line_int() {
        let mut writer = Writer::new(Vec::new());
        writeln!(writer, "123").unwrap();
        assert_eq!(writer.buf_writer.into_inner().unwrap(), b"123\n");
    }
}
