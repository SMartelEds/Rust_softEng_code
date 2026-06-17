//! Macro helpers.
//!
//! The PDF macro section says macros can reduce boilerplate and generate code.
//! Here the macro avoids repetitive calls to `add_node` and `add_edge`.

/// Builds a graph with compact syntax.
///
/// Example:
///
/// ```
/// use graph_dijkstra_modular::{dijkstra, graph};
///
/// let graph = graph! {
///     directed;
///     nodes: 3;
///     edges: [
///         0 => 1 : 7,
///         1 => 2 : 2,
///     ]
/// };
///
/// let result = dijkstra(&graph, 0).unwrap();
/// assert_eq!(result.distances[2], Some(9));
/// ```
#[macro_export]
macro_rules! graph {
    // Directed syntax:
    // 0 => 1 : 7 means "edge from 0 to 1 with weight 7".
    (
        directed;
        nodes: $nodes:expr;
        edges: [$( $from:tt => $to:tt : $weight:expr ),* $(,)?]
    ) => {{
        // Use `$crate` so the macro works both inside and outside this crate.
        $crate::Graph::from_edges(
            $crate::GraphKind::Directed,
            $nodes,
            // Build the slice expected by Graph::from_edges.
            &[$(($from, $to, $weight)),*],
        )
        // In a teaching macro, panic is acceptable for invalid literal examples.
        .expect("graph! received an invalid directed edge")
    }};
    // Undirected syntax:
    // 0 -- 1 : 7 means "two-way connection with weight 7".
    (
        undirected;
        nodes: $nodes:expr;
        edges: [$( $from:tt -- $to:tt : $weight:expr ),* $(,)?]
    ) => {{
        $crate::Graph::from_edges(
            $crate::GraphKind::Undirected,
            $nodes,
            &[$(($from, $to, $weight)),*],
        )
        // Invalid node ids are caught by the normal graph API.
        .expect("graph! received an invalid undirected edge")
    }};
}
