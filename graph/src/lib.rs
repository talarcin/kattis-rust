use std::collections::{HashMap, HashSet};

pub fn make_graph(pairs: &Vec<(usize, usize)>) -> Graph {
    let mut graph: Graph = Graph::new();
    for pair in pairs {
        graph.add_node(&pair.0);
        graph.add_node(&pair.1);
        graph.connect_nodes(&pair.0, &pair.1);
    }

    graph
}

pub fn make_complementary_graph(graph: &Graph) -> Graph {
    let mut comp_graph: Graph = Graph::new();

    for (u, edges) in &graph.nodes {
        comp_graph.add_node(&u);

        for v in &graph.get_nodes() {
            if !edges.contains(v) {
                comp_graph.add_node(v);
                comp_graph.connect_nodes(u, v);
            }
        }
    }

    comp_graph
}

pub struct Graph {
    nodes: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(self: &mut Self, u: &usize) {
        if !self.nodes.contains_key(u) {
            self.nodes.insert(*u, HashSet::new());
        }
    }

    fn connect_nodes(self: &mut Self, u: &usize, v: &usize) {
        self.nodes.get_mut(u).unwrap().insert(*v);
        self.nodes.get_mut(v).unwrap().insert(*u);
    }

    fn get_nodes(self: &Self) -> Vec<usize> {
        let mut node_vec = Vec::new();
        for (u, _) in &self.nodes {
            node_vec.push(*u);
        }

        node_vec
    }

    fn size(&self) -> usize {
        self.nodes.values().count()
    }
}

#[cfg(test)]
mod tests {}
