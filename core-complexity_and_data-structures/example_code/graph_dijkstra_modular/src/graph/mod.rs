//! Graph representation module.
//!
//! This module is responsible only for storing nodes and edges.
//! Algorithms such as Dijkstra, BFS, and DFS live in `algorithms`.
//! This separation matches the PDF point about modularity and separation of concerns.

mod error;
mod types;

pub use error::GraphError;
pub use types::{Edge, GraphKind, NodeId, Weight};

/// A graph stored as an adjacency list.
///
/// PDF connection:
/// - The graph slide says adjacency lists are efficient for sparse graphs.
/// - Instead of allocating a full V x V matrix, each node stores only its outgoing edges.
/// - This is useful when most possible edges do not exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    // Directed means A -> B is different from B -> A.
    // Undirected means A -- B is stored in both directions.
    kind: GraphKind,

    // adjacency[node] stores all outgoing edges from that node.
    // This is the adjacency-list model from the graph slide.
    adjacency: Vec<Vec<Edge>>,

    // Logical edge count.
    // In an undirected graph, A -- B counts once even if stored twice.
    edge_count: usize,
}

impl Graph {
    /// Creates an empty graph.
    pub fn new(kind: GraphKind) -> Self {
        // No nodes exist yet, so the adjacency list starts empty.
        Self {
            kind,
            adjacency: Vec::new(),
            edge_count: 0,
        }
    }

    /// Convenience constructor for a directed graph.
    pub fn directed() -> Self {
        Self::new(GraphKind::Directed)
    }

    /// Convenience constructor for an undirected graph.
    pub fn undirected() -> Self {
        Self::new(GraphKind::Undirected)
    }

    /// Adds one vertex and returns its numeric id.
    pub fn add_node(&mut self) -> NodeId {
        // The new node id is the next index in the adjacency vector.
        let id = self.adjacency.len();

        // Each node owns a list of outgoing edges.
        self.adjacency.push(Vec::new());
        id
    }

    /// Returns the number of vertices.
    pub fn node_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Returns the logical number of edges.
    ///
    /// For undirected graphs, one connection A-B counts as one edge even though the
    /// adjacency list stores both A -> B and B -> A.
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Returns the graph kind: directed or undirected.
    pub fn kind(&self) -> GraphKind {
        self.kind
    }

    /// Checks whether a node id exists.
    pub fn has_node(&self, node: NodeId) -> bool {
        // Node ids are valid indexes into `adjacency`.
        node < self.node_count()
    }

    /// Adds a weighted edge.
    ///
    /// PDF connection:
    /// - Weighted graphs are required for Dijkstra because every edge has a cost.
    /// - This implementation uses `u32`, so weights are naturally non-negative.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, weight: Weight) -> Result<(), GraphError> {
        // Validate first so the graph cannot enter a partially updated state.
        self.validate_node(from)?;
        self.validate_node(to)?;

        // Store the outgoing edge from `from`.
        self.adjacency[from].push(Edge::new(from, to, weight));

        if self.kind == GraphKind::Undirected {
            // Undirected graphs need the reverse edge for traversal from both sides.
            self.adjacency[to].push(Edge::new(to, from, weight));
        }

        self.edge_count += 1;
        Ok(())
    }

    /// Returns the outgoing edges of one node.
    pub fn neighbors(&self, node: NodeId) -> Result<&[Edge], GraphError> {
        // Returning a slice avoids copying the edge list.
        self.validate_node(node)?;
        Ok(&self.adjacency[node])
    }

    /// Builds a graph with a fixed node count and a slice of edges.
    ///
    /// This helper is used by the `graph!` macro.
    pub fn from_edges(
        kind: GraphKind,
        node_count: usize,
        edges: &[(NodeId, NodeId, Weight)],
    ) -> Result<Self, GraphError> {
        let mut graph = Self::new(kind);

        // Allocate all vertices before inserting edges.
        for _ in 0..node_count {
            graph.add_node();
        }

        // Add every edge through the normal checked API.
        for &(from, to, weight) in edges {
            graph.add_edge(from, to, weight)?;
        }

        Ok(graph)
    }

    fn validate_node(&self, node: NodeId) -> Result<(), GraphError> {
        // This private helper centralizes bounds checking.
        if self.has_node(node) {
            Ok(())
        } else {
            Err(GraphError::MissingNode {
                node,
                node_count: self.node_count(),
            })
        }
    }
}
