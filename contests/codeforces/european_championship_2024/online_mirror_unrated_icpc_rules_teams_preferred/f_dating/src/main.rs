use std::{
    collections::BTreeSet,
    io::{Read, Write},
};

use algo::io::{reader::Reader, writer::Writer};

fn dfs(
    node: usize,
    tree: &Vec<Vec<usize>>,
    parents: &mut Vec<Option<usize>>,
    activities_count: &Vec<usize>,
) -> Option<(usize, usize)> {
    for &child in &tree[node] {
        if let Some(parent) = parents[child] {
            if activities_count[parent] < activities_count[node] {
                return Some((child, parent));
            } else {
                return Some((child, node));
            }
        }

        parents[child] = Some(node);
        if let Some(res) = dfs(child, tree, parents, activities_count) {
            return Some(res);
        }
    }
    None
}

fn dfs2(
    node: usize,
    tree: &Vec<Vec<usize>>,
    activities: &Vec<BTreeSet<usize>>,
    visited: &mut Vec<bool>,
) -> Option<(usize, usize)> {
    if visited[node] {
        return None;
    }

    visited[node] = true;
    let node_activities = &activities[node];
    for &child in &tree[node] {
        if let Some(res) = dfs2(child, tree, activities, visited) {
            return Some(res);
        }
        if !node_activities.is_superset(&activities[child]) {
            return Some((node, child));
        }
    }
    None
}

fn solve_case<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let n: usize = reader.read();
    let m: usize = reader.read();

    let mut activities_by_user = vec![vec![]; n];
    let mut activities_count = vec![0; n];
    let mut users_by_activity = vec![vec![]; m + 1];
    for (user, activities) in activities_by_user.iter_mut().enumerate() {
        let k: usize = reader.read();
        *activities = reader.read_vec::<usize>(k);
        for activity in activities {
            users_by_activity[*activity].push(user);
        }
        activities_count[user] = k;
    }

    let mut inserted = BTreeSet::new();

    let mut tree = vec![vec![]; n];
    for users in &mut users_by_activity {
        users.sort_by_key(|user| activities_count[*user]);
        for w in users.windows(2) {
            let child = w[0];
            let parent = w[1];
            if inserted.contains(&(child, parent)) {
                continue;
            }
            tree[parent].push(child);
            inserted.insert((child, parent));
        }
    }

    let mut parents: Vec<Option<usize>> = vec![None; n];
    let mut ans: Option<(usize, usize)> = None;

    let mut sorted_activities_count = activities_count.iter().enumerate().collect::<Vec<_>>();
    sorted_activities_count.sort_by_cached_key(|v| v.1);
    for &(user, _) in sorted_activities_count.iter().rev() {
        if ans.is_some() {
            break;
        }
        if parents[user].is_some() {
            continue;
        }
        ans = dfs(user, &tree, &mut parents, &activities_count);
    }

    if ans.is_none() {
        let activities = activities_by_user
            .into_iter()
            .map(|activities| activities.into_iter().collect::<BTreeSet<_>>())
            .collect::<Vec<_>>();

        let mut visited = vec![false; n];
        for &(user, _) in sorted_activities_count.iter().rev() {
            if ans.is_some() {
                break;
            }
            ans = dfs2(user, &tree, &activities, &mut visited);
        }
    }

    if let Some((u, v)) = ans {
        writeln!(writer, "YES").unwrap();
        writeln!(writer, "{} {}", u + 1, v + 1).unwrap();
    } else {
        writeln!(writer, "NO").unwrap();
    }
}

fn solve<R: Read, W: Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) {
    let testcases: usize = 1;
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
3 5
3 1 2 4
5 1 2 3 4 5
2 1 5

"#;
        let expected = r#"
YES
3 1

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
3 3
1 1
1 2
3 2 3 1

"#;
        let expected = r#"
NO

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
3 5
2 1 2
4 1 2 5 3
5 1 2 3 5 4
"#;
        let expected = r#"
NO
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
4 5
2 1 2
3 1 2 5
4 1 2 3 5
2 5 3
"#;
        let expected = r#"
YES
4 2
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
    fn case_05() {
        let input = r#"
4 5
2 1 2
2 2 5
4 1 2 3 5
2 5 3
"#;
        let expected = r#"
YES
1 2
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
    fn case_06() {
        let input = r#"
4 4
1 1
3 1 2 3
3 1 2 4
1 2
"#;
        let expected = r#"
YES
3 2
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
