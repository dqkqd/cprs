use std::{
    collections::BTreeSet,
    io::{Read, Write},
};

use algo::{
    io::{reader::Reader, writer::Writer},
    string::z::ZFunction,
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let l: usize = reader.read();
    let r: usize = reader.read();
    let s: String = reader.read();

    let z = s.z_function();
    let mut non_zero_indices = BTreeSet::new();
    let mut indices = vec![vec![]; n + 1];
    for (index, &value) in z.iter().enumerate() {
        indices[value as usize].push(index);
        if value != 0 {
            non_zero_indices.insert(index);
        }
    }

    let mut count = vec![0; n + 1];
    for i in 1..n + 1 {
        count[i] += 1;
        let mut index = i;
        while let Some(next_index) = non_zero_indices.range(index..n).next() {
            index = next_index + i;
            count[i] += 1;
        }

        for value in &indices[i] {
            non_zero_indices.remove(value);
        }
    }

    let mut ans = vec![0; n + 1];
    for (index, value) in count.iter().enumerate().skip(1) {
        ans[*value as usize] = index;
    }
    for i in (0..ans.len() - 1).rev() {
        ans[i] = ans[i].max(ans[i + 1]);
    }
    ans[1] = n;
    writer.write_vec(&ans[l..r + 1]);
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
7
3 1 3
aba
3 2 3
aaa
7 1 5
abacaba
9 1 6
abababcab
10 1 10
aaaaaaawac
9 1 9
abafababa
7 2 7
vvzvvvv

"#;
        let expected = r#"
3 1 0
1 1
7 3 1 1 0
9 2 2 2 0 0
10 3 2 1 1 1 1 1 0 0
9 3 2 1 1 0 0 0 0
2 2 1 1 1 0

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
