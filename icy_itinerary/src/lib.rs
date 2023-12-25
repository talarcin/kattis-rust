use graph::{make_complementary_graph, make_graph, Graph};

pub fn solve() -> String {
    String::from("not implemented")
}

pub fn algorithm(lines: Vec<String>) {
    let (n, m, pairs) = parse_input(lines);
    let mut graph: Graph = make_graph(&pairs);
    let mut comp_graph: Graph = make_complementary_graph(&graph);
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
mod tests {}
