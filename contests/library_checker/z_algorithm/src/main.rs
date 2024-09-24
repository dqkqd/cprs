use std::io::{Read, Write};

use algo::{
    io::{reader::Reader, writer::Writer},
    string::z::ZFunction,
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let s: String = reader.read();
    let mut z = s.z_function();
    z[0] = s.len() as u32;
    writer.write_vec(&z);
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
