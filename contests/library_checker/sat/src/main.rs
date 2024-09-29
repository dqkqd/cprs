use std::io::{Read, Write};

use algo::{
    io::{reader::Reader, writer::Writer},
    misc::two_sat::TwoSat,
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let _: String = reader.read();
    let _: String = reader.read();
    let n: usize = reader.read();
    let m: usize = reader.read();

    let mut two_sat = TwoSat::new(n);
    for _ in 0..m {
        let a: i32 = reader.read();
        let b: i32 = reader.read();
        let _: char = reader.read();
        two_sat.add_clause((a.abs() - 1) as u32, a > 0, (b.abs() - 1) as u32, b > 0);
    }

    match two_sat.solve() {
        Some(result) => {
            writeln!(writer, "s SATISFIABLE").unwrap();
            write!(writer, "v ").unwrap();
            for v in result
                .assignment
                .into_iter()
                .map(|v| if v { 1 } else { -1 })
                .enumerate()
                .map(|(index, v)| (index + 1) as i32 * v)
            {
                write!(writer, "{} ", v).unwrap();
            }
            writeln!(writer, "0").unwrap();
        }
        None => {
            writeln!(writer, "s UNSATISFIABLE").unwrap();
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
