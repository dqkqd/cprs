use std::io::{Read, Write};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let k: usize = reader.read();
    let a: Vec<usize> = reader.read_vec(n);
    let max = a.iter().max().unwrap();
    let sum: usize = a.iter().sum();

    dbg!(max, sum);
    for size in (1..=n).rev() {
        let v = (sum + k) / size;
        if v * size >= sum && v >= *max {
            writeln!(writer, "{}", size).unwrap();
            break;
        }
    }
}

fn solve<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let testcases: usize = reader.read();
    for case in 0..testcases {
        eprintln!("Solve case {}", case + 1);
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
9
3 1
3 2 2
5 4
2 6 1 2 4
2 100
1410065408 10000000000
10 8
7 4 6 6 9 3 10 2 8 7
2 12
2 2
2 70
0 1
1 0
1
3 0
2 1 2
3 1
0 3 3

"#;
        let expected = r#"
2
3
1
7
2
2
1
1
2

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
