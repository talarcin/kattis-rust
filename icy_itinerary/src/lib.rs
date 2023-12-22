use std::collections::HashSet;

pub struct Node {
    value: usize,
    adjacent: HashSet<Node>,
}

pub struct Graph {
    nodes: HashSet<Node>,
}

impl Node {
    fn value(&self) -> &usize {
        &self.value
    }

    fn adj(&self) -> &HashSet<Node> {
        &self.adjacent
    }
}

impl Graph {}

pub fn solve() -> String {}

#[cfg(test)]
mod tests {}
