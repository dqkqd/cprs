use std::io::{Read, Write};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: f64 = reader.read();
    let x: f64 = reader.read();
    let y: f64 = reader.read();
    let c = x.min(y);
    let ans = (n / c).ceil();
    writeln!(writer, "{}", ans).unwrap();
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
7
0
10 10
5
3 4
3
1 2
6
4 3
100
4 3
9
3 3
1000000000
1 1

"#;
        let expected = r#"
0
2
3
2
34
3
1000000000
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
