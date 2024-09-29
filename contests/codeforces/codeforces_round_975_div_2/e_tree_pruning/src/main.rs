use std::io::{Read, Write};

use algo::{
    graph::base::Graph,
    io::{reader::Reader, writer::Writer},
    rfn,
};

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let mut graph = Graph::new(n);
    for _ in 0..n - 1 {
        let u: u32 = reader.read();
        let v: u32 = reader.read();
        graph.add_edge(u - 1, v - 1);
        graph.add_edge(v - 1, u - 1);
    }

    let mut heights = vec![0; n];
    let mut max_children_heights = vec![0; n];
    let mut parents = vec![None; n];
    let mut is_leaf = vec![true; n];
    let mut dfs = rfn!(|f, node: usize, parent: Option<usize>| {
        parents[node] = parent;
        for &child in &graph[node] {
            let child = child as usize;
            if Some(child) == parent {
                continue;
            }
            heights[child] = heights[node] + 1;
            max_children_heights[child] = heights[child];
            is_leaf[node] = false;
            f(child, Some(node));
            max_children_heights[node] =
                max_children_heights[node].max(max_children_heights[child]);
        }
    });
    dfs(0, None);

    let mut leaves: Vec<(u32, usize)> = Vec::new();
    let mut height_cnt = vec![0; n];
    for (index, h) in heights.iter().enumerate() {
        if is_leaf[index] {
            leaves.push((*h, index));
        }
        height_cnt[*h as usize] += 1;
    }
    leaves.sort_unstable();
    leaves.reverse();

    let mut removed = vec![false; n];
    let mut removed_cnt = 0;

    let mut height_cnt_sum = 0;

    let mut ans = usize::MAX;
    for (h, cnt) in height_cnt.iter().enumerate() {
        while let Some((leaf_height, mut node)) = leaves.last().cloned() {
            if leaf_height >= h as u32 {
                break;
            }
            while !removed[node] && max_children_heights[node] <= leaf_height {
                removed[node] = true;
                removed_cnt += 1;
                let parent = parents[node];
                if parent.is_none() {
                    break;
                }
                node = parent.unwrap();
            }
            leaves.pop();
        }
        height_cnt_sum += cnt;
        let tree_size_after = height_cnt_sum - removed_cnt;
        let need_remove = n - tree_size_after;
        ans = ans.min(need_remove);
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
4
7
1 2
1 3
2 4
2 5
4 6
4 7
7
1 2
1 3
1 4
2 5
3 6
5 7
15
12 9
1 6
6 14
9 11
8 7
3 5
13 5
6 10
13 15
13 6
14 12
7 2
8 1
1 4
8
1 2
2 3
2 4
2 5
5 6
6 7
7 8
"#;
        let expected = r#"
2
2
5
2
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
