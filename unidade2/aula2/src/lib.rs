use petgraph::graph::{Graph};
use petgraph::visit::Dfs;

pub struct MyGraph {
    graph: Graph<&'static str, ()>,
}

impl MyGraph {
    pub fn new() -> Self {
        let mut graph = Graph::new();

        let node1 = graph.add_node("1");
        let node2 = graph.add_node("2");
        let node3 = graph.add_node("3");
        let node4 = graph.add_node("4");
        let node5 = graph.add_node("5");
        let node6 = graph.add_node("6");

        graph.add_edge(node1, node2, ());
        graph.add_edge(node1, node3, ());
        graph.add_edge(node2, node4, ());
        graph.add_edge(node3, node5, ());
        graph.add_edge(node4, node6, ());
        graph.add_edge(node5, node6, ());

        MyGraph { graph }
    }

    pub fn dfs_from_node1(&self) -> Vec<&'static str> {
        let mut result = Vec::new();
        let mut dfs = Dfs::new(&self.graph, self.graph.node_indices().find(|&i| self.graph[i] == "1").unwrap());

        while let Some(node) = dfs.next(&self.graph) {
            result.push(self.graph[node]);
        }

        result
    }
}

// -----------------------------------------------------------
// TESTES
// -----------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_visits_all_nodes() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();

        let mut sorted_result = result.clone();
        sorted_result.sort();

        let mut expected = vec!["1", "2", "3", "4", "5", "6"];
        expected.sort();

        assert_eq!(sorted_result, expected, "Todos os nós devem ser visitados");
    }

    #[test]
    fn test_dfs_starts_at_node1() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();
        assert_eq!(result.first(), Some(&"1"), "DFS deve começar pelo nó 1");
    }
}
