# Graph Dijkstra Modular

This project focuses only on the graph slides from:

`SE1_Advanced_data_structure_procedural design.pdf`

It intentionally does not cover the other data structures from the deck.

## PDF Points Covered

- Graphs as vertices and edges.
- Directed and undirected graphs.
- Weighted edges.
- Adjacency-list memory layout for sparse graphs.
- Dijkstra shortest path.
- BFS for level-by-level traversal and connectivity.
- DFS for directed cycle detection.
- Modularity and separation of concerns.
- A declarative macro for graph construction.
- Derived traits on graph types.

## Project Layout

- `src/graph/`: graph storage, edge types, graph kind, and errors.
- `src/algorithms/`: Dijkstra, BFS, connectivity, and DFS cycle detection.
- `src/macros.rs`: `graph!` macro for compact graph construction.
- `src/main.rs`: runnable classroom demo.

## Run

```bash
cargo run
```

## Test

```bash
cargo test
```

## Example

```rust
use graph_dijkstra_modular::{dijkstra, graph};

let graph = graph! {
    directed;
    nodes: 3;
    edges: [
        0 => 1 : 5,
        1 => 2 : 7,
    ]
};

let result = dijkstra(&graph, 0).unwrap();
assert_eq!(result.path_to(2), Some(vec![0, 1, 2]));
```
