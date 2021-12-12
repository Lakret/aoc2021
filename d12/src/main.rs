use std::collections::HashMap;
use std::fs;

type G = HashMap<String, Vec<String>>;
type Path = Vec<String>;

fn main() {
    println!("Hello, world!");
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
    }
}
