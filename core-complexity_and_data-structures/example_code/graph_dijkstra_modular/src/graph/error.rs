//! Error types for graph operations.

use std::fmt::{self, Display};

use super::NodeId;

/// Errors that can happen when manipulating or traversing a graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// A requested node id does not exist in the graph.
    MissingNode { node: NodeId, node_count: usize },
}

impl Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode { node, node_count } => {
                // Human-readable error for examples and CLI output.
                write!(
                    f,
                    "node {node} does not exist; graph has {node_count} node(s)"
                )
            }
        }
    }
}

// Implementing Error makes GraphError compatible with standard Rust error handling.
impl std::error::Error for GraphError {}
