use std::{
    collections::{BTreeMap, VecDeque},
    io::{Read, Write},
};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let m: u64 = reader.read();
    let k: u64 = reader.read();
    let mut diaries = vec![(0, 0); n];
    for (d, a) in &mut diaries {
        *d = reader.read();
        *a = reader.read();
    }

    let until = diaries
        .iter()
        .map(|&(d, a)| (d, a))
        .collect::<BTreeMap<_, _>>();

    let mut entries = diaries
        .iter()
        .flat_map(|v| vec![v.0, v.0 + k - 1])
        .collect::<Vec<_>>();
    entries.sort_unstable();
    entries.dedup();
    entries.push(1e15 as u64);

    let mut ans = 0;
    let mut milk = VecDeque::new();
    let mut run = |l: u64, r: u64| {
        if r < l {
            return;
        }

        if let Some(&milk_until) = until.get(&l) {
            milk.push_back((l + k - 1, milk_until));
        }

        let range = r - l + 1;

        let r = r + k - 1;
        let maximum = range * m;
        let mut total = 0;
        // search from [l, r] in milk
        let mut last_deadline = None;
        while let Some(&(deadline, amount)) = milk.back() {
            assert!(deadline <= r);
            if total >= maximum {
                break;
            }

            if deadline < l {
                break;
            }

            total += amount;
            last_deadline = Some(deadline);
            milk.pop_back();
        }

        let satisfactory = (total / m).min(range);
        ans += satisfactory;

        if total > maximum {
            // push back some
            assert!(last_deadline.is_some());
            milk.push_back((last_deadline.unwrap(), total - maximum));
        }
    };

    for w in entries.windows(2) {
        run(w[0], w[0]);
        run(w[0] + 1, w[1] - 1);
    }

    writeln!(writer, "{}", ans).unwrap();
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
6
1 1 3
1 5
2 3 3
1 5
2 7
4 5 2
1 9
2 6
4 9
5 6
5 2 4
4 7
5 3
7 1
11 2
12 1
4 1 3
5 10
9 4
14 8
15 3
5 5 5
8 9
10 7
16 10
21 5
28 9

"#;
        let expected = r#"
3
3
4
5
10
6

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
1
5 2 4
4 7
5 3
7 1
11 2
12 1
"#;
        let expected = r#"
5
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
