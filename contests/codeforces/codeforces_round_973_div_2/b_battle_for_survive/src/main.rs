use std::io::{Read, Write};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let mut v = vec![0; n];
    for x in &mut v {
        *x = reader.read::<i64>()
    }
    let (last, elements) = v.split_last().unwrap();
    let (second_last, elements) = elements.split_last().unwrap();
    let ans = last - (second_last - elements.iter().sum::<i64>());
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
5
2
2 1
3
2 2 8
4
1 2 4 3
5
1 2 3 4 5
5
3 2 4 5 4

"#;
        let expected = r#"
-1
8
2
7
8

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
