use std::{
    collections::HashMap,
    io::{Read, Write},
};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let q: usize = reader.read();
    let x: Vec<usize> = reader.read_vec(n);

    let mut map = HashMap::new();
    for (i, w) in x.windows(2).enumerate() {
        let prev = w[0];
        let next = w[1];

        let count_prev = (n - i) * (i + 1) - 1;
        map.entry(count_prev)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);

        let count_mid = (i + 1) * (n - i - 1);
        map.entry(count_mid)
            .and_modify(|counter| *counter += next - prev - 1)
            .or_insert(next - prev - 1);
        // dbg!(next - prev - 1);
    }

    map.entry(n - 1)
        .and_modify(|counter| *counter += 1)
        .or_insert(1);

    let k: Vec<usize> = reader.read_vec(q);
    let ans = k
        .into_iter()
        .map(|x| map.get(&x).unwrap_or(&0))
        .collect::<Vec<_>>();
    writer.write_vec(&ans);
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
2 2
101 200
2 1
6 15
1 2 3 5 6 7
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
5 8
254618033 265675151 461318786 557391198 848083778
6 9 15 10 6 9 4 4294967300

"#;
        let expected = r#"
0 100
0 0 0 0 2 0 0 0 3 0 2 0 0 0 0
291716045 0 0 0 291716045 0 301749698 0

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
