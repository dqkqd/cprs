use std::io::{Read, Write};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let mut v = vec![0; n];
    for x in &mut v {
        *x = reader.read::<i64>();
    }

    let vmin = *v.iter().min().unwrap();
    let vmax = *v.iter().max().unwrap();

    let mut low = vmin;
    let mut high = vmax;
    while low < high {
        let mid = (low + high + 1) / 2;
        let mut sum = 0;
        let mut ok = true;
        for x in &v {
            sum += x - mid;
            if sum < 0 {
                ok = false;
                break;
            }
        }
        if ok {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    let min = low;

    let v = v.into_iter().rev().collect::<Vec<_>>();
    let mut low = vmin;
    let mut high = vmax;
    while low < high {
        let mid = (low + high) / 2;
        let mut sum = 0;
        let mut ok = true;
        for x in &v {
            sum += mid - x;
            if sum < 0 {
                ok = false;
                break;
            }
        }
        if ok {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    let max = low;

    writeln!(writer, "{}", max - min).unwrap();
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
1
1
3
1 2 3
4
4 1 2 3
4
4 2 3 1
5
5 14 4 10 2

"#;
        let expected = r#"
0
2
1
1
3

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
