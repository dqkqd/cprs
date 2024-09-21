use std::io::{Read, Write};

use algo::{
    graph::base::Graph,
    io::{reader::Reader, writer::Writer},
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let m: usize = reader.read();

    let mut g = Graph::new(n);
    for _ in 0..m {
        let u: i32 = reader.read();
        let v: i32 = reader.read();
        g.add_edge(u, v);
    }

    let components = g.scc().components();
    writeln!(writer, "{}", components.size).unwrap();
    for id in 0..components.size {
        write!(writer, "{} ", components[id].len()).unwrap();
        for id in &components[id] {
            write!(writer, "{} ", id).unwrap();
        }
        writeln!(writer).unwrap();
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
