//! Modular graph project focused on the graph slides from the PDF.
//!
//! Covered PPT points:
//! - Graphs are vertices plus edges.
//! - Graphs can be directed or undirected.
//! - Graphs can be weighted.
//! - Adjacency lists are memory-friendly for sparse graphs.
//! - Dijkstra solves shortest path on weighted graphs with non-negative weights.
//! - BFS explores level by level for connectivity.
//! - DFS explores deeply and can detect cycles in directed graphs.
//! - Macros can reduce repetitive graph construction code.
//! - `derive` can provide common behavior such as `Debug`, `Clone`, `Copy`, `Eq`,
//!   and `PartialEq`.

pub mod algorithms;
pub mod graph;
pub mod macros;

pub use algorithms::{DijkstraResult, bfs_order, dijkstra, has_cycle_directed, is_connected_from};
pub use graph::{Edge, Graph, GraphError, GraphKind, NodeId, Weight};
