use std::io::{Read, Write};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let a: Vec<i32> = reader.read_vec(n);
    let a = a
        .into_iter()
        .enumerate()
        .map(|(index, x)| x - (index as i32) - 1)
        .collect::<Vec<_>>();

    let mut suffix_min = a
        .iter()
        .rev()
        .scan(i32::MAX, |state, &x| {
            *state = (*state).min(x);
            Some(*state)
        })
        .collect::<Vec<_>>();
    suffix_min.reverse();

    let mut cnt = 0;
    let mut min_left = i32::MAX;

    for i in 0..a.len() {
        let step = i as i32;
        if i > 0 {
            let value = a[i - 1] + (step - 1) - 1;
            if value < 0 {
                break;
            }

            let l = i + value as usize + 1;
            if l < n && suffix_min[l] + step < 1 {
                break;
            }
        }

        let right_ok = suffix_min[i] + step >= 0;
        let left_ok = min_left >= 0;

        if right_ok && left_ok {
            cnt += 1;
        }

        min_left = min_left.min(a[i] + step);
        min_left -= 1;
    }

    writeln!(writer, "{}", cnt).unwrap();
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
3
6
6 3 3 3 5 5
6
5 6 4 1 4 5
9
8 6 4 2 1 3 5 7 9

"#;
        let expected = r#"
3
0
1

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
