use graph::{make_complementary_graph, make_graph, Graph};

pub fn dfs(graph: &Graph, u: &usize, visited: &mut Vec<bool>) {
    visited[*u] = true;

    for v in graph.nodes.get(u).unwrap() {
        dfs(graph, v, visited);
    }
}

pub fn algorithm(lines: Vec<String>) -> bool {
    let (n, _, pairs) = parse_input(lines);

    let mut visited: Vec<bool> = vec![false; n + 1];

    // 1. Calculate road graph
    let road_graph: Graph = make_graph(&pairs);
    // 2. Calculate snow graph (complementary graph)
    let snow_graph: Graph = make_complementary_graph(&road_graph);

    // 3. Set start graph bases on adjacent houses of house 1
    let current_graph = if road_graph.nodes.get(&1).unwrap().is_empty() {
        snow_graph
    } else {
        road_graph
    };

    // 4. Start depth-first search at house 1 and current graph^
    dfs(&current_graph, &1, &mut visited);
    for v in visited {
        if !v {
            return false;
        }
    }

    true
}

pub fn parse_input(lines: Vec<String>) -> (usize, usize, Vec<(usize, usize)>) {
    let desc: Vec<usize> = lines[0]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let n = desc[0];
    let m = desc[1];

    let mut pairs: Vec<(usize, usize)> = Vec::new();

    for i in 1..=m {
        let pair: Vec<usize> = lines[i]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        pairs.push((pair[0], pair[1]));
    }

    (n, m, pairs)
}

#[cfg(test)]
mod tests {
    use crate::algorithm;

    #[test]
    fn test_simple_graph() {
        let simple = vec![
            "4 4".to_string(),
            "1 2".to_string(),
            "2 3".to_string(),
            "3 4".to_string(),
            "4 1".to_string(),
        ];

        assert_eq!(algorithm(simple), true);
    }
}
