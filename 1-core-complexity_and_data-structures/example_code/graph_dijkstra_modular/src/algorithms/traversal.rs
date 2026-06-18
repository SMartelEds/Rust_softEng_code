//! BFS, connectivity, and DFS cycle detection.
//!
//! PDF connection:
//! - BFS explores level by level.
//! - DFS explores as far as possible along each branch.
//! - Connectivity asks whether all nodes are reachable from a starting node.
//! - Cycle detection is useful in directed graphs, for example dependency graphs.

use std::collections::{HashSet, VecDeque};

use crate::graph::{Graph, GraphError, NodeId};

/// Returns the BFS visit order from `start`.
pub fn bfs_order(graph: &Graph, start: NodeId) -> Result<Vec<NodeId>, GraphError> {
    // Validate start early so the caller gets a Result error.
    graph.neighbors(start)?;

    // HashSet prevents visiting the same node twice.
    let mut visited = HashSet::new();

    // Queue gives BFS its level-by-level behavior.
    let mut queue = VecDeque::from([start]);

    // Keep the traversal order for display and tests.
    let mut order = Vec::new();

    while let Some(node) = queue.pop_front() {
        // insert returns false when the node was already visited.
        if visited.insert(node) {
            order.push(node);

            for edge in graph.neighbors(node)? {
                // Push neighbors after current level nodes already in the queue.
                queue.push_back(edge.to);
            }
        }
    }

    Ok(order)
}

/// Checks whether every node is reachable from `start`.
pub fn is_connected_from(graph: &Graph, start: NodeId) -> Result<bool, GraphError> {
    // Reuse BFS instead of duplicating traversal logic.
    let order = bfs_order(graph, start)?;

    // Connected from start means BFS reached every vertex.
    Ok(order.len() == graph.node_count())
}

/// Detects whether a directed graph has a cycle.
///
/// The function still works on the stored outgoing edges of an undirected graph, but the
/// teaching purpose here is directed dependency-style cycle detection.
pub fn has_cycle_directed(graph: &Graph) -> bool {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum VisitState {
        // Node has not been explored yet.
        NotVisited,

        // Node is currently in the recursion stack.
        Visiting,

        // Node and all descendants are fully explored.
        Done,
    }

    fn dfs(graph: &Graph, node: NodeId, states: &mut [VisitState]) -> bool {
        // Mark as active in the current DFS branch.
        states[node] = VisitState::Visiting;

        let Ok(edges) = graph.neighbors(node) else {
            // Should not happen with internal node ids, but keep the helper total.
            return false;
        };

        for edge in edges {
            match states[edge.to] {
                // Reaching an active node means a back edge, hence a cycle.
                VisitState::Visiting => return true,
                VisitState::NotVisited => {
                    // Explore deeper along this branch.
                    if dfs(graph, edge.to, states) {
                        return true;
                    }
                }
                // Already checked; no need to explore again.
                VisitState::Done => {}
            }
        }

        // This node is no longer active in the recursion stack.
        states[node] = VisitState::Done;
        false
    }

    // Initially no node has been visited.
    let mut states = vec![VisitState::NotVisited; graph.node_count()];

    for node in 0..graph.node_count() {
        // Start DFS from every component, not only node 0.
        if states[node] == VisitState::NotVisited && dfs(graph, node, &mut states) {
            return true;
        }
    }

    false
}
