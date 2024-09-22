use std::io::{Read, Write};

use algo::{
    graph::dsu::DSU,
    io::{reader::Reader, writer::Writer},
};

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
