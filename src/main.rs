use graphst::algorithm::dijkstra;
use graphst::{DGraph, UGraph};

fn main() {
    // Test an undirected graph
    let n_nodes = 9;
    let edges = vec![
        (0, 1, 4.0),
        (0, 7, 8.0),
        (1, 2, 8.0),
        (1, 7, 11.0),
        (2, 3, 7.0),
        (2, 5, 4.0),
        (2, 8, 2.0),
        (3, 4, 9.0),
        (3, 5, 14.0),
        (4, 5, 10.0),
        (5, 6, 2.0),
        (6, 7, 1.0),
        (6, 8, 6.0),
        (7, 8, 8.0),
    ];
    let g = UGraph::from_weighted_edges(n_nodes, edges);
    println!("\n{}", g);
    println!("dijkstra: {:?}", dijkstra(&g, 0));

    // Test an directed graph
    let n_nodes = 5;
    let edges = vec![
        (0, 1, 4.0),
        (0, 2, 3.0),
        (0, 3, 5.0),
        (1, 2, 2.0),
        (1, 3, 3.0),
        (2, 4, 7.0),
        (3, 4, 2.0),
        (4, 0, 7.0),
    ];
    let g = DGraph::from_weighted_edges(n_nodes, edges);
    println!("\n{}", g);
    println!("dijkstra: {:?}", dijkstra(&g, 0));
}
