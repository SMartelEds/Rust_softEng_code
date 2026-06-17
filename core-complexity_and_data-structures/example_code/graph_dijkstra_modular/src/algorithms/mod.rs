//! Graph algorithms module.
//!
//! This module owns graph behavior such as shortest paths and traversal.
//! The graph storage module does not know about these algorithms.

mod dijkstra;
mod traversal;

pub use dijkstra::{DijkstraResult, dijkstra};
pub use traversal::{bfs_order, has_cycle_directed, is_connected_from};
