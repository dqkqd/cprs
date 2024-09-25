use std::io::{Read, Write};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let a: Vec<i32> = reader.read_vec(n);
    let mut reduce = Vec::with_capacity(n);
    reduce.push(a[0]);

    let mut up = true;
    for x in a.iter().skip(1) {
        if up {
            if x >= reduce.last().unwrap() {
                *reduce.last_mut().unwrap() = *x;
            } else {
                reduce.push(*x);
                up = false;
            }
        } else if x < reduce.last().unwrap() {
            *reduce.last_mut().unwrap() = *x;
        } else {
            reduce.push(*x);
            up = true;
        }
    }

    let upper = (0..reduce.len())
        .step_by(2)
        .map(|i| reduce[i])
        .collect::<Vec<_>>();

    let max = *upper.iter().max().unwrap() as usize;

    let lower = (1..reduce.len() - 1)
        .step_by(2)
        .map(|i| reduce[i])
        .collect::<Vec<_>>();

    let count = |v: Vec<i32>| {
        let mut prefix_sum = vec![0; 2 * max + 1];
        for x in &v {
            prefix_sum[*x as usize] += 1;
        }

        for i in 1..prefix_sum.len() {
            prefix_sum[i] += prefix_sum[i - 1];
        }

        let mut cnt = vec![0usize; max + 1];
        for (i, c) in cnt.iter_mut().enumerate().skip(1) {
            for j in (i..=max + i).step_by(i) {
                *c += (j / i) * (prefix_sum[j] - prefix_sum[j - i]);
            }
        }

        cnt
    };

    let count_upper = count(upper);
    let count_lower = count(lower);

    let ans = count_upper
        .iter()
        .zip(count_lower)
        .skip(1)
        .map(|(u, l)| u - l)
        .collect::<Vec<_>>();
    writer.write_vec(&ans);
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
3
5 2 7

"#;
        let expected = r#"
10 6 4 3 2 2 1

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
7 7 7 7

"#;
        let expected = r#"
7 4 3 2 2 2 1

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
    fn case_03() {
        let input = r#"
10
1 9 7 6 2 4 7 8 1 3

"#;
        let expected = r#"
17 9 5 4 3 3 3 2 1

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
