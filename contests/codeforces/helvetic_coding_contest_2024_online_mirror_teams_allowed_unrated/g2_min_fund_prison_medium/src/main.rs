use std::io::{Read, Write};

use algo::{
    graph::{base::Graph, bridge::Bridge, scc::Scc},
    io::{reader::Reader, writer::Writer},
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let m: usize = reader.read();
    let c: u64 = reader.read();

    let mut graph = Graph::new(n);
    for _ in 0..m {
        let u: u32 = reader.read();
        let v: u32 = reader.read();
        graph.add_edge(u - 1, v - 1);
        graph.add_edge(v - 1, u - 1);
    }

    let calc_x_y = |x: u64| -> u64 {
        if x as usize > n {
            0
        } else {
            let y = n as u64 - x;
            x * x + y * y
        }
    };

    let components = graph.scc();
    if components.size == 1 {
        let tree = graph.bridge(0);
        if tree.bridges.is_empty() {
            writeln!(writer, "{}", -1).unwrap();
            return;
        }
    }
    let c_contrib = c * (components.size as u64 - 1);

    let mut weights = vec![false; n + 1];
    weights[0] = true;
    for index in 0..components.inner.len() {
        let size = components[index].len();
        for i in (0..n - size).rev() {
            if weights[i] {
                weights[i + size] = true;
            }
        }
    }

    let mut x_y_contrib = u64::MAX;
    for (index, w) in weights.iter().enumerate() {
        if *w {
            x_y_contrib = x_y_contrib.min(calc_x_y(index as u64));
        }
    }

    for (id, component) in components.inner.iter().enumerate() {
        let mut weights = vec![false; n + 1];
        weights[0] = true;
        for index in 0..components.inner.len() {
            if index == id {
                continue;
            }
            let size = components[index].len();
            for i in (0..n - size).rev() {
                if weights[i] {
                    weights[i + size] = true;
                }
            }
        }

        let tree = graph.bridge(component[0]);
        for (u, _) in tree.bridges {
            let size_u = tree.weight[u as usize] as u64;
            for (index, w) in weights.iter().enumerate() {
                if *w {
                    x_y_contrib = x_y_contrib.min(calc_x_y(index as u64 + size_u));
                }
            }
        }
    }

    let ans = x_y_contrib + c_contrib;
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
2
4 6 5
4 3
2 3
2 4
1 2
4 1
3 1
6 6 2
1 4
2 5
3 6
1 5
3 5
6 5
"#;
        let expected = r#"
-1
20
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
6 5 7
1 4
2 5
3 6
3 5
6 5
"#;
        let expected = r#"
25
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
1
7 5 4
1 4
3 6
3 5
6 5
2 7
"#;
        let expected = r#"
33
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
    fn case_04() {
        let input = r#"
1
6 6 1
1 2
2 3
3 1
4 5
5 6
6 4
"#;
        let expected = r#"
19
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
