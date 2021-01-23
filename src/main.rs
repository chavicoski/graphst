use graphst::Graph;

fn main() {
    let empty_g = Graph::new();
    println!("empty_g:\n{}\n", empty_g);

    let n_nodes = 5;
    let mat = vec![vec![0.0; n_nodes]; n_nodes];
    let mut g = Graph::from_adjacency_matrix(mat);
    g.add_connection(0, 0);
    g.add_connection(0, 3);
    g.add_connection(4, 2);

    println!("g:\n{}", g);
    println!("The nodes of g are: {:?}\n", g.get_nodes());

    let edges = vec![
        vec![0, 1],
        vec![1, 2],
        vec![2, 4],
        vec![4, 3],
        vec![3, 0],
        vec![3, 1],
        vec![3, 3],
    ];
    let g2 = Graph::from_edges(n_nodes, edges);
    println!("g2:\n{}", g2);
    println!("The nodes of g2 are: {:?}\n", g2.get_nodes());
}
