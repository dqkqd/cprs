use std::io::{Read, Write};

use algo::{
    graph::{
        base::{Graph, GraphBase},
        bridge::Bridge,
    },
    io::{reader::Reader, writer::Writer},
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n = reader.read::<usize>();
    let m = reader.read::<usize>();

    let mut graph = Graph::new_undirected(n);
    for _ in 0..m {
        let u = reader.read::<u32>();
        let v = reader.read::<u32>();
        graph.add_edge(u - 1, v - 1);
    }

    let tree = graph.bridges(0);
    let mut ans = n * (n - 1) / 2;
    for (u, _) in tree.bridges {
        let u = u as usize;
        let size = tree.weight[u] as usize;
        let left = size * (size - 1) / 2;
        let right = (n - size) * (n - size - 1) / 2;
        ans = ans.min(left + right);
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
2 1
1 2
3 3
1 2
2 3
1 3
5 5
1 2
1 3
3 4
4 5
5 3
6 7
1 2
1 3
2 3
3 4
4 5
4 6
5 6
5 5
1 2
1 3
2 3
2 4
3 5
10 12
1 2
1 3
2 3
2 4
4 5
5 6
6 7
7 4
3 8
8 9
9 10
10 8

"#;
        let expected = r#"
0
3
4
6
6
21

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
