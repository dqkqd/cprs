use std::io::{Read, Write};

use algo::{
    external::num_integer::gcd::Gcd,
    io::{reader::Reader, writer::Writer},
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: u32 = reader.read();
    let m: u32 = reader.read();

    let mut ans = 0;
    for x in 1..f32::sqrt(n as f32).ceil() as u32 {
        for y in 1..f32::sqrt(m as f32).ceil() as u32 {
            if (x as i32).gcd(&(y as i32)) != 1 {
                continue;
            }
            let cx = n / ((x + y) * x);
            let cy = m / ((x + y) * y);
            ans += cx.min(cy);
        }
    }

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
6
1 1
2 3
3 5
10 8
100 1233
1000000 1145141

"#;
        let expected = r#"
0
1
1
6
423
5933961

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
