use std::collections::HashMap;
use std::fs;

type G = HashMap<String, Vec<String>>;
type Path = Vec<String>;

fn main() {
    let g = parse(&fs::read_to_string("d12_input").unwrap());
    let p1_ans = dfs(&g).len();
    println!("p1 = {}", p1_ans);

    let p2_ans = dfs2(&g).len();
    println!("p1 = {}", p2_ans);
}

fn parse(input: &str) -> G {
    let mut g: G = HashMap::new();

    for segment in input.trim_end().split_whitespace() {
        let mut parts = segment.split("-");
        let src = parts.next().unwrap();
        let dst = parts.next().unwrap();

        let dsts = g.entry(src.to_string()).or_default();
        dsts.push(dst.to_string());

        let srcs = g.entry(dst.to_string()).or_default();
        srcs.push(src.to_string());
    }

    g
}

fn dfs(g: &G) -> Vec<Path> {
    let mut stack = vec!["start".to_string()];
    let mut paths = vec![];

    let mut vertex_to_paths = HashMap::new();
    vertex_to_paths.insert("start".to_string(), vec![vec!["start".to_string()]]);

    while let Some(v) = stack.pop() {
        if let Some(adjacent) = g.get(&v) {
            if let Some(paths_ending_with_v) = vertex_to_paths.remove(&v) {
                for w in adjacent.iter() {
                    if w == "end" {
                        for mut path in paths_ending_with_v.iter().cloned() {
                            path.push(w.to_string());
                            paths.push(path);
                        }
                    } else {
                        let mut found_new_paths = false;
                        let paths_ending_with_w = vertex_to_paths.entry(w.to_string()).or_default();

                        for mut partial_path in paths_ending_with_v.iter().cloned() {
                            if !(is_visit_once_vertex(&w) && partial_path.contains(&w)) {
                                partial_path.push(w.clone());

                                paths_ending_with_w.push(partial_path);
                                found_new_paths = true;
                            }
                        }

                        if found_new_paths {
                            stack.push(w.clone());
                        }
                    }
                }
            }
        }
    }

    paths
}

fn dfs2(g: &G) -> Vec<Path> {
    let mut stack = vec!["start".to_string()];
    let mut paths = vec![];

    let mut vertex_to_paths = HashMap::new();
    vertex_to_paths.insert("start".to_string(), vec![vec!["start".to_string()]]);

    while let Some(v) = stack.pop() {
        if let Some(adjacent) = g.get(&v) {
            if let Some(paths_ending_with_v) = vertex_to_paths.remove(&v) {
                for w in adjacent.iter() {
                    if w == "end" {
                        for mut path in paths_ending_with_v.iter().cloned() {
                            path.push(w.to_string());
                            paths.push(path);
                        }
                    } else if w != "start" {
                        let mut found_new_paths = false;
                        let paths_ending_with_w = vertex_to_paths.entry(w.to_string()).or_default();

                        for mut partial_path in paths_ending_with_v.iter().cloned() {
                            if can_be_extended_with(&partial_path, &w) {
                                partial_path.push(w.clone());

                                paths_ending_with_w.push(partial_path);
                                found_new_paths = true;
                            }
                        }

                        if found_new_paths {
                            stack.push(w.clone());
                        }
                    }
                }
            }
        }
    }

    paths
}

fn can_be_extended_with(path: &Path, vertex: &str) -> bool {
    if is_visit_once_vertex(vertex) {
        let mut counts: HashMap<String, usize> = HashMap::new();
        *counts.entry(vertex.to_string()).or_default() += 1;

        for another_vertex in path.iter() {
            if is_visit_once_vertex(another_vertex) {
                *counts.entry(another_vertex.clone()).or_default() += 1;
            }
        }

        let mut double_appearances = 0;
        for value in counts.values() {
            if *value == 2 {
                double_appearances += 1;
            }

            if *value > 2 {
                return false;
            }
        }

        if double_appearances > 1 {
            return false;
        }
    }

    return true;
}

fn is_visit_once_vertex(v: &str) -> bool {
    v.chars().next().unwrap().is_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let test_input = fs::read_to_string("../d12_test_input").unwrap();
        let test_g = parse(&test_input);
        assert_eq!(dfs(&test_g).len(), 226);

        let input = fs::read_to_string("../d12_input").unwrap();
        let g = parse(&input);
        assert_eq!(dfs(&g).len(), 4970);

        assert_eq!(dfs2(&test_g).len(), 3509);
        assert_eq!(dfs2(&g).len(), 4970);
    }
}
