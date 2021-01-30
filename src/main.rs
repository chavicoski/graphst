use graphst::Graph;
use graphst::algorithm;

fn main() {
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
    let g = Graph::from_weighted_edges_undirected(n_nodes, edges);
    println!("g:\n{}", g);
    println!("dijkstra g:\n{:?}", algorithm::dijkstra(&g, 0));
}
