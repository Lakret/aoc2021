use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let g = parse(&fs::read_to_string("d12_input").unwrap());

    println!("p1 = {}", p1(&g));
    println!("p1 = {}", p2(&g));
}

type Path = Vec<usize>;

#[derive(Default, Debug)]
pub struct G {
    pub edges: HashMap<usize, Vec<usize>>,
    pub vertices: Vec<String>,
    pub visit_once_vertices: HashSet<usize>,
}

impl G {
    fn get_or_create_vertex_id(&mut self, vertex: &str) -> usize {
        match self.vertices.iter().position(|v| v == vertex) {
            Some(pos) => pos,
            None => {
                self.vertices.push(vertex.to_string());
                let id = self.vertices.len() - 1;

                if is_visit_once_vertex(vertex) {
                    self.visit_once_vertices.insert(id);
                }

                id
            }
        }
    }

    fn add_edge(&mut self, v1: usize, v2: usize) {
        self.edges.entry(v1).or_default().push(v2);
        self.edges.entry(v2).or_default().push(v1);
    }

    fn get_vertex_id(&self, vertex: &str) -> usize {
        self.vertices.iter().position(|v| v == vertex).unwrap()
    }
}

fn p1(g: &G) -> usize {
    dfs(g, p1_extension_validator).len()
}

fn p2(g: &G) -> usize {
    dfs(g, p2_extension_validator).len()
}

fn parse(input: &str) -> G {
    let mut g = G::default();

    for segment in input.trim_end().split_whitespace() {
        let mut parts = segment.split("-");
        let v1 = parts.next().unwrap();
        let v2 = parts.next().unwrap();

        let v1_id = g.get_or_create_vertex_id(v1);
        let v2_id = g.get_or_create_vertex_id(v2);

        g.add_edge(v1_id, v2_id);
    }

    g
}

fn dfs<F>(g: &G, is_extension_valid: F) -> Vec<Path>
where
    F: Fn(&G, &Path, &usize) -> bool,
{
    let start_id = g.get_vertex_id("start");
    let end_id = g.get_vertex_id("end");

    let mut stack = vec![start_id];
    let mut paths = vec![];

    let mut vertex_to_paths = HashMap::new();
    vertex_to_paths.insert(start_id, vec![vec![start_id]]);

    while let Some(v) = stack.pop() {
        if let Some(adjacent) = g.edges.get(&v) {
            if let Some(paths_ending_with_v) = vertex_to_paths.remove(&v) {
                for w in adjacent.iter() {
                    if *w == end_id {
                        for mut path in paths_ending_with_v.iter().cloned() {
                            path.push(*w);
                            paths.push(path);
                        }
                    } else if *w != start_id {
                        let mut found_new_paths = false;
                        let paths_ending_with_w = vertex_to_paths.entry(*w).or_default();

                        for partial_path in paths_ending_with_v.iter() {
                            if is_extension_valid(g, &partial_path, w) {
                                let mut partial_path = partial_path.clone();
                                partial_path.push(*w);

                                paths_ending_with_w.push(partial_path);
                                found_new_paths = true;
                            }
                        }

                        if found_new_paths {
                            stack.push(*w);
                        }
                    }
                }
            }
        }
    }

    paths
}

fn p1_extension_validator(g: &G, path: &Path, vertex_id: &usize) -> bool {
    !(g.visit_once_vertices.contains(vertex_id) && path.contains(vertex_id))
}

fn p2_extension_validator(g: &G, path: &Path, vertex_id: &usize) -> bool {
    if g.visit_once_vertices.contains(vertex_id) {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        *counts.entry(*vertex_id).or_default() += 1;

        for another_vertex_id in path.iter() {
            if g.visit_once_vertices.contains(another_vertex_id) {
                *counts.entry(*another_vertex_id).or_default() += 1;
            }
        }

        let mut double_appearance_found = false;
        for value in counts.values() {
            if *value == 2 {
                if double_appearance_found {
                    return false;
                }

                double_appearance_found = true;
            }

            if *value > 2 {
                return false;
            }
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
    fn p1_and_p2_test() {
        let test_input = fs::read_to_string("../d12_test_input").unwrap();
        let test_g = parse(&test_input);
        assert_eq!(p1(&test_g), 226);

        let input = fs::read_to_string("../d12_input").unwrap();
        let g = parse(&input);
        assert_eq!(p1(&g), 4970);

        assert_eq!(p2(&test_g), 3509);
        // assert_eq!(p2(&g), 137948);
    }
}
