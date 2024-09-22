use std::io::{Read, Write};

use algo::{
    external::num_integer::gcd::Gcd,
    io::{reader::Reader, writer::Writer},
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let mut v = vec![0; n];
    for x in &mut v {
        *x = reader.read::<i32>();
    }

    let mut sum = 0;
    while !v.is_empty() {
        let min = *v.iter().min().unwrap();
        sum += min as i64;
        let p = v.iter().position(|&x| x == min).unwrap();
        v.remove(p);

        for x in &mut v {
            *x = x.gcd(&min);
        }

        if v.last().is_some_and(|x| v.iter().all(|e| e == x)) {
            break;
        }
    }

    for x in v {
        sum += x as i64;
    }

    writeln!(writer, "{}", sum).unwrap();
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
3
4 2 2
2
6 3
3
10 15 6
5
6 42 12 52 20
4
42 154 231 66

"#;
        let expected = r#"
6
6
9
14
51

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
