use petgraph::dot::{Dot, Config};
use petgraph::graph::Graph;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut graph = Graph::<&str, &str>::new();

    //Adicionando Nós
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let _d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("F");

    //Arestas
    graph.add_edge(a, b, "");
    graph.add_edge(b, f, "");
    graph.add_edge(f, e, "");
    graph.add_edge(c, b, "");
    graph.add_edge(a, f, "");
    graph.add_edge(b, a, "");
    // .dot
    let dot_output = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let mut file = File::create("graph.dot")?;
    file.write_all(dot_output.as_bytes())?;

    println!("Arquivo gravado com sucesso");
    Ok(())
}

// CRIAÇÃO DE GRAFO QUE APAGUEI DA MAIN PARA PODER RODAR OS TESTES. *COLAR DNV DPS*

use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: &str) {
        self.adjacency_list.entry(vertex.to_string()).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacency_list.entry(from.to_string()).or_insert(Vec::new()).push(to.to_string());
        self.adjacency_list.entry(to.to_string()).or_insert(Vec::new()).push(from.to_string());
    }

    fn dfs(&self, start: &str) {
        let mut visited = HashSet::new();
        println!("DFS");
        self.dfs_recursive(start, &mut visited);
    }

    fn dfs_recursive(&self, vertex: &str, visited: &mut HashSet<String>) {
        if visited.contains(vertex) {
            return;
        }
        
        println!("{}", vertex);
        visited.insert(vertex.to_string());

        if let Some(neighbors) = self.adjacency_list.get(vertex){
            for neighbor in neighbors {
                self.dfs_recursive(neighbor, visited);
            }
        }
    }

    fn bfs(&self, start: &str) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        println!("BFS");

        while let Some(current) = queue.pop_front() {
            println!("{}", current);

            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }
}

fn main() {
    let mut graph = Graph::new();

    graph.add_vertex("A");
    graph.add_vertex("B");
    graph.add_vertex("C");
    graph.add_vertex("D");
    graph.add_vertex("E");

    graph.add_edge("A", "B");
    graph.add_edge("A", "C");
    graph.add_edge("B", "D");
    graph.add_edge("C", "E");

    graph.dfs("A");
    println!("---");
    graph.bfs("A");

}