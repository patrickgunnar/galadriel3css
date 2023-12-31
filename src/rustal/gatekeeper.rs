use petgraph::graph::{DiGraph, NodeIndex};

pub struct Gatekeeper {
  graph: DiGraph<String, ()>,
}

impl Gatekeeper {
  pub fn new() -> Self {
    Gatekeeper {
      graph: DiGraph::new(),
    }
  }

  fn add_module(&mut self, module_name: &str) -> NodeIndex {
    self.graph.add_node(module_name.to_string())
  }

  pub fn add_import(&mut self, from_module: &str, to_module: &str) {
    let from_node = self.add_module(from_module);
    let to_node = self.add_module(to_module);

    self.graph.add_edge(from_node, to_node, ());
  }

  pub fn print_graph(&self) {
    for node in self.graph.node_indices() {
      println!("Module: {:?}", self.graph[node]);

      for neighbor in self.graph.neighbors(node) {
        println!("  ---> Imports: {:?}", neighbor);
      }
    }
  }
}
