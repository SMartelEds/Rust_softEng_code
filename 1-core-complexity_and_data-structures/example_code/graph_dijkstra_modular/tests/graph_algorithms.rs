use graph_dijkstra_modular::{
    Edge, Graph, GraphError, GraphKind, bfs_order, dijkstra, graph, has_cycle_directed,
    is_connected_from,
};

#[test]
fn dijkstra_finds_the_cheapest_path() {
    // Weighted directed graph from the demo.
    let graph = graph! {
        directed;
        nodes: 5;
        edges: [
            0 => 1 : 4,
            0 => 2 : 1,
            2 => 1 : 2,
            1 => 3 : 1,
            2 => 3 : 5,
            3 => 4 : 3,
        ]
    };

    let result = dijkstra(&graph, 0).expect("start node exists");

    // Node 1 is cheaper through node 2: 0 -> 2 -> 1 costs 3.
    assert_eq!(
        result.distances,
        vec![Some(0), Some(3), Some(1), Some(4), Some(7)]
    );

    // Path reconstruction follows the `previous` links.
    assert_eq!(result.path_to(4), Some(vec![0, 2, 1, 3, 4]));
}

#[test]
fn bfs_and_connectivity_work_from_a_start_node() {
    // Node 2 has no outgoing edges, so it cannot reach every node.
    let graph = graph! {
        directed;
        nodes: 4;
        edges: [
            0 => 1 : 1,
            0 => 2 : 1,
            1 => 3 : 1,
        ]
    };

    assert_eq!(bfs_order(&graph, 0).unwrap(), vec![0, 1, 2, 3]);
    assert!(is_connected_from(&graph, 0).unwrap());
    assert!(!is_connected_from(&graph, 2).unwrap());
}

#[test]
fn directed_cycle_detection_uses_dfs() {
    // Cycle: 0 -> 1 -> 2 -> 0.
    let cyclic = graph! {
        directed;
        nodes: 3;
        edges: [
            0 => 1 : 1,
            1 => 2 : 1,
            2 => 0 : 1,
        ]
    };

    // Same shape without the back edge.
    let acyclic = graph! {
        directed;
        nodes: 3;
        edges: [
            0 => 1 : 1,
            1 => 2 : 1,
        ]
    };

    assert!(has_cycle_directed(&cyclic));
    assert!(!has_cycle_directed(&acyclic));
}

#[test]
fn undirected_graph_stores_reverse_neighbors_but_counts_one_logical_edge() {
    // Undirected storage duplicates the edge internally for easy traversal.
    let graph = graph! {
        undirected;
        nodes: 2;
        edges: [
            0 -- 1 : 9,
        ]
    };

    assert_eq!(graph.edge_count(), 1);
    assert_eq!(graph.neighbors(0).unwrap(), &[Edge::new(0, 1, 9)]);
    assert_eq!(graph.neighbors(1).unwrap(), &[Edge::new(1, 0, 9)]);
}

#[test]
fn invalid_node_returns_a_result_error() {
    let mut graph = Graph::new(GraphKind::Directed);
    graph.add_node();

    // Node 9 does not exist, so add_edge returns a typed error.
    let error = graph.add_edge(0, 9, 1).unwrap_err();

    assert_eq!(
        error,
        GraphError::MissingNode {
            node: 9,
            node_count: 1,
        }
    );
}
