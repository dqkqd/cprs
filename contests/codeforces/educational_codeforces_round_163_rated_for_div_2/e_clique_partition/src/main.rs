use std::io::{Read, Write};

use algo::io::{reader::Reader, writer::Writer};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let k: usize = reader.read();

    let mut a = vec![0; n];
    let mut num_partitions = 0;
    let mut partitions = vec![0; n];

    let mut current = 0;
    while current < n {
        let stop = (k + current).min(n);
        let mid = (current + stop - 1) / 2;
        for (i, v) in a.iter_mut().enumerate().take(mid + 1).skip(current) {
            *v = mid - i + current + 1;
        }
        for (i, v) in a.iter_mut().enumerate().take(stop).skip(mid + 1) {
            *v = stop + mid - i + 1;
        }
        num_partitions += 1;
        for p in partitions.iter_mut().take(stop).skip(current) {
            *p = num_partitions;
        }
        current = stop;
    }

    writer.write_vec(&a);
    writeln!(writer, "{}", num_partitions).unwrap();
    writer.write_vec(&partitions);
}

fn solve<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let testcases: usize = reader.read();
    for case in 0..testcases {
        eprintln!("Solve case {}", case);
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
2 3
5 4
8 16

"#;
        let expected = r#"
2 1
1
1 1
3 1 5 2 4
2
1 1 2 1 2
1 2 3 4 5 6 7 8
1
1 1 1 1 1 1 1 1

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
