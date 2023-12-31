use petgraph::graph::{DiGraph, NodeIndex};

pub struct Gatekeeper {
  graph: DiGraph<String, ()>,
}

/*
  - Gatekeeper is responsible for generating the relation between the import groups.
*/
impl Gatekeeper {
  pub fn new() -> Self {
    Gatekeeper {
      graph: DiGraph::new(),
    }
  }

  // Depth-First Search (DFS) traversal on a graph starting from a given node.
  // Parameters:
  // - start_node: The index of the starting node for DFS.
  // - visited: A mutable vector representing visited nodes.
  // - path_group: A mutable vector to store the nodes visited in the DFS path.
  fn dfs(&self, start_node: NodeIndex, visited: &mut Vec<bool>, path_group: &mut Vec<String>) {
    // Mark the current node as visited and add it to the path group.
    visited[start_node.index()] = true;
    path_group.push(self.graph[start_node].clone());

    // Explore neighbors of the current node in DFS.
    for neighbor in self.graph.neighbors(start_node) {
      // If the neighbor has not been visited, recursively perform DFS on the neighbor.
      if !visited[neighbor.index()] {
        self.dfs(neighbor, visited, path_group);
      }
    }
  }

  // Group paths in the graph using Depth-First Search (DFS).
  // Returns a vector of vectors, where each inner vector represents a connected component.
  pub fn group_paths(&self) -> Vec<Vec<String>> {
    // Create a vector to track visited nodes.
    let mut visited = vec![false; self.graph.node_count()];
    // Create a vector to store groups of connected components.
    let mut groups = Vec::new();

    // Iterate through all nodes in the graph.
    for node in self.graph.node_indices() {
      // If the node has not been visited, start a new DFS to explore the connected component.
      if !visited[node.index()] {
        // Create a vector to store the nodes in the current connected component.
        let mut group = Vec::new();

        // Perform DFS on the current node to explore the connected component.
        self.dfs(node, &mut visited, &mut group);
        // Add the connected component to the list of groups.
        groups.push(group);
      }
    }

    // Return the vector of groups.
    groups
  }

  // Find or add a module to the graph.
  // Returns the NodeIndex of the existing or newly added module.
  fn find_or_add_module(&mut self, module_name: &str) -> NodeIndex {
    // Check if a node with the given module_name already exists in the graph.
    if let Some(node) = self
      .graph
      .node_indices()
      .find(|&node| self.graph[node] == module_name)
    {
      // If the node exists, return its NodeIndex.
      node
    } else {
      // If the node does not exist, add a new node with the module_name to the graph.
      self.graph.add_node(module_name.to_string())
    }
  }

  // Add an import relationship between two modules in the graph.
  // Nodes in the graph represent modules, and edges represent imports.
  pub fn add_import(&mut self, from_module: &str, to_module: &str) {
    // Find or add nodes for the source (from_module) and target (to_module) modules.
    let from_node = self.find_or_add_module(from_module);
    let to_node = self.find_or_add_module(to_module);

    // Add an edge between the source and target nodes to represent the import relationship.
    self.graph.add_edge(from_node, to_node, ());
  }

  // Print the module graph, displaying modules and their import relationships.
  pub fn print_graph(&self) {
    for node in self.graph.node_indices() {
      println!("Module: {:?}", self.graph[node]);

      for neighbor in self.graph.neighbors(node) {
        println!("  ---> Imports: {:?}", neighbor);
      }
    }

    println!("{:#?}", self.graph);
  }
}
