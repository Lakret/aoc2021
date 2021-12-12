use std::collections::HashMap;
use std::fs;

type G = HashMap<String, Vec<String>>;

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

fn dfs(g: &G) -> Vec<Vec<String>> {
    let mut finished_paths: Vec<Vec<String>> = vec![];

    let mut stack = vec!["start".to_string()];

    let mut end_vertex_to_partial_paths: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    end_vertex_to_partial_paths.insert("start".to_string(), vec![vec!["start".to_string()]]);

    while let Some(v) = stack.pop() {
        if let Some(adjacent) = g.get(&v) {
            if let Some(partial_paths_ending_with_v) = end_vertex_to_partial_paths.remove(&v) {
                for w in adjacent.iter() {
                    if w == "end" {
                        for mut partial_path in partial_paths_ending_with_v.iter().cloned() {
                            partial_path.push(w.to_string());
                            finished_paths.push(partial_path);
                        }
                    } else {
                        let partial_paths_ending_with_w = end_vertex_to_partial_paths
                            .entry(w.to_string())
                            .or_default();

                        let mut new_paths_found = false;
                        for partial_path in partial_paths_ending_with_v.iter() {
                            if !(is_visit_once_vertex(&w) && partial_path.contains(&w)) {
                                let mut partial_path = partial_path.clone();
                                partial_path.push(w.clone());

                                partial_paths_ending_with_w.push(partial_path.clone());
                                new_paths_found = true;
                            }
                        }

                        if new_paths_found {
                            stack.push(w.clone());
                        }
                    }
                }
            }
        }
    }

    finished_paths
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
