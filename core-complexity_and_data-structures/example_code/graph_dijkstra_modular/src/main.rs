use graph_dijkstra_modular::{bfs_order, dijkstra, graph, has_cycle_directed, is_connected_from};

fn main() {
    // Main is intentionally small.
    // Real logic lives in modules so each concern is isolated.

    // This graph is directed and weighted.
    //
    // PDF connection:
    // - Vertices are numeric ids: 0, 1, 2, 3, 4.
    // - Edges represent relationships between vertices.
    // - Weights represent costs, such as distance or travel time.
    // - The `graph!` macro keeps the example compact.
    let graph = graph! {
        directed;
        nodes: 5;
        edges: [
            0 => 1 : 4,
            0 => 2 : 1,
            // This cheaper route makes Dijkstra prefer 0 -> 2 -> 1 over 0 -> 1.
            2 => 1 : 2,
            1 => 3 : 1,
            2 => 3 : 5,
            3 => 4 : 3,
        ]
    };

    println!("Graph demo based on the PDF graph slides\n");

    // Basic graph metadata.
    println!("Nodes: {}", graph.node_count());
    println!("Edges: {}", graph.edge_count());

    // BFS explores neighbors by distance in number of edges.
    println!("BFS from node 0: {:?}", bfs_order(&graph, 0).unwrap());

    // Connectivity answers: "Can start reach every node?"
    println!(
        "All nodes reachable from 0: {}",
        is_connected_from(&graph, 0).unwrap()
    );

    // DFS cycle detection is useful for dependency graphs.
    println!("Directed cycle detected: {}", has_cycle_directed(&graph));

    // Dijkstra handles weighted shortest paths.
    let shortest_paths = dijkstra(&graph, 0).unwrap();

    println!("\nDijkstra from node 0:");
    for (node, distance) in shortest_paths.distances.iter().enumerate() {
        // None would mean unreachable.
        println!("- distance to {node}: {distance:?}");
    }

    // The previous array reconstructs the actual shortest path.
    println!(
        "Shortest path from 0 to 4: {:?}",
        shortest_paths.path_to(4).unwrap()
    );
}
