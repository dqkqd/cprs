use std::io::{Read, Write};

use algo::{
    io::{reader::Reader, writer::Writer},
    izip,
    misc::two_sat::TwoSat,
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let mut g: [Vec<i32>; 3] = core::array::from_fn(|_| vec![0; n]);

    for row in &mut g {
        for item in row {
            *item = reader.read();
        }
    }

    let mut graph = TwoSat::new(n);
    for (g1, g2, g3) in izip!(&g[0], &g[1], &g[2]) {
        graph.add_clause(g1.abs() - 1, *g1 > 0, g2.abs() - 1, *g2 > 0);
        graph.add_clause(g2.abs() - 1, *g2 > 0, g3.abs() - 1, *g3 > 0);
        graph.add_clause(g3.abs() - 1, *g3 > 0, g1.abs() - 1, *g1 > 0);
    }

    if graph.solve().is_some() {
        writeln!(writer, "YES").unwrap();
    } else {
        writeln!(writer, "NO").unwrap();
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_01() {
        let input = r#"
4
4
1 -2 -3 -2
-4 4 -1 -3
1 2 -2 4
2
1 2
-1 -2
2 -2
5
1 2 3 4 5
-2 3 -4 -5 -1
3 -5 1 2 2
6
1 3 -6 2 5 2
1 3 -2 -3 -6 -5
-2 -1 -3 2 3 1

"#;
        let expected = r#"
YES
NO
YES
NO

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
