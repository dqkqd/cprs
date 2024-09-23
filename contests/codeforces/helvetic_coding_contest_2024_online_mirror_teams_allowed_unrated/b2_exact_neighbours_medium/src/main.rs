use std::io::{Read, Write};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let v: Vec<u32> = reader.read_vec(n);
    assert!(v[0] == 0);
    let mut position = vec![(0, 0); n];
    let mut relative = vec![0; n];

    let mut v = v.into_iter().enumerate().collect::<Vec<_>>();
    v.sort_by_key(|x| x.1);

    let mut rel = 0;
    let mut rel_col = 0;

    for (col, &(pos, val)) in v.iter().enumerate() {
        if val == 0 {
            position[pos] = (col, 0);
            relative[pos] = pos;
            rel = pos;
            rel_col = col;
        } else {
            let diff = col - rel_col;
            let row = val - diff as u32;
            position[pos] = (col, row);
            relative[pos] = rel;
            if row == 0 {
                rel = pos;
                rel_col = col;
            }
        }
    }

    writeln!(writer, "YES").unwrap();
    for (x, y) in position {
        writeln!(writer, "{} {}", x + 1, y + 1).unwrap();
    }

    let relative = &relative.iter().map(|v| v + 1).collect::<Vec<_>>();
    writer.write_vec(relative);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_01() {
        let input = r#"
4
0 4 2 4

"#;
        let expected = r#"
YES
4 4
1 3
2 4
3 1
1 1 1 3

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

    #[test]
    fn case_02() {
        let input = r#"
4
0 1 3 1

"#;
        let expected = r#"
YES
1 1
2 1
4 1
3 1
1 1 1 3

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
