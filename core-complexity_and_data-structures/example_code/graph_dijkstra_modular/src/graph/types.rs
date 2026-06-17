//! Shared graph types.
//!
//! Keeping these definitions in their own file makes the project easier to navigate.

/// Numeric node identifier.
///
/// For a classroom example, numeric ids are simpler than generic labels.
pub type NodeId = usize;

/// Edge weight used by Dijkstra.
///
/// `u32` prevents negative weights. That matters because Dijkstra is not valid for
/// graphs with negative edge weights.
pub type Weight = u32;

/// Whether edges have one direction or two directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphKind {
    // Edge is stored only from source to destination.
    Directed,

    // Edge is mirrored so both endpoints can reach each other.
    Undirected,
}

/// A weighted edge between two nodes.
///
/// Derives:
/// - `Debug` lets us print edges during demos.
/// - `Clone` and `Copy` are safe because the fields are simple numbers.
/// - `PartialEq` and `Eq` make tests straightforward.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    // Source node.
    pub from: NodeId,

    // Destination node.
    pub to: NodeId,

    // Cost used by weighted algorithms such as Dijkstra.
    pub weight: Weight,
}

impl Edge {
    pub fn new(from: NodeId, to: NodeId, weight: Weight) -> Self {
        // Small constructor keeps edge creation explicit and readable.
        Self { from, to, weight }
    }
}
