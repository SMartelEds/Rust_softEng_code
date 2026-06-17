//! Dijkstra shortest-path algorithm.
//!
//! PDF connection:
//! - The graph slide lists shortest path as a major graph problem.
//! - Dijkstra finds the cheapest path from one start node to every reachable node.
//! - Complexity can be expensive for large graphs, which is why implementation choices matter.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::graph::{Graph, GraphError, NodeId, Weight};

/// Result of running Dijkstra from one start node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DijkstraResult {
    /// `distances[node]` is the shortest known distance from the start node.
    /// `None` means the node is unreachable.
    pub distances: Vec<Option<Weight>>,

    /// `previous[node]` stores the previous node on the shortest path.
    /// It is used to rebuild the actual path after distances are computed.
    pub previous: Vec<Option<NodeId>>,
}

impl DijkstraResult {
    /// Reconstructs the shortest path from the start node to `target`.
    pub fn path_to(&self, target: NodeId) -> Option<Vec<NodeId>> {
        // If the target is out of range or unreachable, there is no path.
        self.distances.get(target).copied().flatten()?;

        // Rebuild backward: target -> previous -> previous -> start.
        let mut path = Vec::new();
        let mut current = Some(target);

        while let Some(node) = current {
            // Store the current step.
            path.push(node);

            // Move to the predecessor chosen by Dijkstra.
            current = self.previous[node];
        }

        // The path was built backward, so reverse it for start -> target order.
        path.reverse();
        Some(path)
    }
}

/// Computes shortest paths from `start` to every reachable node.
pub fn dijkstra(graph: &Graph, start: NodeId) -> Result<DijkstraResult, GraphError> {
    // Validate the start node before allocating algorithm state.
    graph.neighbors(start)?;

    let node_count = graph.node_count();

    // None means "infinite distance" or "not reached yet".
    let mut distances = vec![None; node_count];

    // previous[node] stores the parent used for path reconstruction.
    let mut previous = vec![None; node_count];

    // BinaryHeap is a max-heap, so Reverse makes it behave like a min-priority queue.
    // The queue stores (distance, node), always processing the cheapest known node next.
    let mut queue = BinaryHeap::new();

    // Distance from the start to itself is always zero.
    distances[start] = Some(0);

    // Seed the priority queue with the start node.
    queue.push((Reverse(0), start));

    while let Some((Reverse(current_distance), node)) = queue.pop() {
        // Skip stale queue entries.
        // A better distance may have been discovered after this entry was pushed.
        if Some(current_distance) > distances[node] {
            continue;
        }

        // Relax every outgoing edge from the current node.
        for edge in graph.neighbors(node)? {
            // Candidate path cost: start -> node -> edge.to.
            let next_distance = current_distance + edge.weight;

            // Keep the candidate only if it improves the best known distance.
            if distances[edge.to].map_or(true, |best| next_distance < best) {
                distances[edge.to] = Some(next_distance);
                previous[edge.to] = Some(node);

                // Revisit this neighbor later with its improved distance.
                queue.push((Reverse(next_distance), edge.to));
            }
        }
    }

    Ok(DijkstraResult {
        distances,
        previous,
    })
}
