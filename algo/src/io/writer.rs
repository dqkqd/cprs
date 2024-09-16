use std::{
    fmt::Display,
    io::{BufWriter, Write},
};

pub struct Writer<W: Write> {
    buf_writer: BufWriter<W>,
}

impl<W: Write> Writer<W> {
    pub fn new(inner: W) -> Writer<W> {
        Writer {
            buf_writer: BufWriter::with_capacity(1024, inner),
        }
    }
    pub fn write<T: Display>(&mut self, data: T) {
        let data = data.to_string();
        self.buf_writer.write_all(data.as_bytes()).unwrap();
    }
    pub fn write_line<T: Display>(&mut self, data: T) {
        let data = data.to_string() + "\n";
        self.buf_writer.write_all(data.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_int() {
        let mut writer = Writer::new(Vec::new());
        writer.write(123);
        assert_eq!(writer.buf_writer.into_inner().unwrap(), b"123");
    }

    #[test]
    fn test_write_string() {
        let mut writer = Writer::new(Vec::new());
        writer.write("123");
        assert_eq!(writer.buf_writer.into_inner().unwrap(), b"123");
    }

    #[test]
    fn test_write_line_int() {
        let mut writer = Writer::new(Vec::new());
        writer.write_line(123);
        assert_eq!(writer.buf_writer.into_inner().unwrap(), b"123\n");
    }
}
