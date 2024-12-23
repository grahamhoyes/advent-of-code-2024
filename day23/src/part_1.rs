use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub type Graph = HashMap<String, HashSet<String>>;

pub fn build_graph(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();

    for line in input.trim().lines() {
        let (left, right) = line.split_once("-").unwrap();
        let left = left.to_string();
        let right = right.to_string();

        // Build an undirected graph
        graph.entry(left.clone()).or_default().insert(right.clone());
        graph.entry(right).or_default().insert(left);
    }

    graph
}

/// Find all cliques of size 3
pub fn find_3_cliques(graph: Graph) -> Vec<Vec<String>> {
    // Cliques of size 3
    let mut cliques: Vec<Vec<String>> = Vec::new();

    let nodes = graph.keys().cloned().collect_vec();

    for (node1, node2, node3) in nodes.into_iter().tuple_combinations() {
        let neighbours1 = &graph[&node1];
        let neighbours2 = &graph[&node2];

        // Check for a clique
        if neighbours1.contains(&node2)
            && neighbours1.contains(&node3)
            && neighbours2.contains(&node3)
        {
            cliques.push(vec![node1, node2, node3]);
        }
    }

    cliques
}

pub fn solution(input: &str) -> usize {
    let graph = build_graph(input);
    let cliques = find_3_cliques(graph);

    // Filter for just cliques that have a node starting with "t"
    cliques
        .into_iter()
        .filter(|nodes| nodes.iter().any(|node| node.starts_with("t")))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 7);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1173);
    }
}
