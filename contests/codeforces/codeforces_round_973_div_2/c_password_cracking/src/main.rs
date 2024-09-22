use std::io::{Read, Write};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: i32 = reader.read();

    let mut s = String::new();
    let mut ask = |t: String| {
        writeln!(writer, "? {}", t).unwrap();
        writer.flush().unwrap();
        reader.read::<u8>()
    };

    // first char
    if ask("0".to_string()) == 0 {
        s += "1";
    } else {
        s += "0";
    }
    let n = n - 1;

    let mut end_right = false;
    for _ in 0..n {
        if end_right {
            let zero = ["0", &s].join("");
            let one = ["1", &s].join("");
            s = if ask(zero.clone()) == 1 { zero } else { one };
        } else {
            let zero = [&s, "0"].join("");
            let one = [&s, "1"].join("");
            if ask(zero.clone()) == 1 {
                s = zero;
            } else if ask(one.clone()) == 1 {
                s = one;
            } else {
                end_right = true;
                let zero = ["0", &s].join("");
                let one = ["1", &s].join("");
                s = if ask(zero.clone()) == 1 { zero } else { one };
            }
        }
    }

    writeln!(writer, "! {}", s).unwrap();
    writer.flush().unwrap();
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
4
3

0

0

1

4

4

2

"#;
        let expected = r#"


? 00

? 000

? 010

! 010

! 1100

! 0110

! 10

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
