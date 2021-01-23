use graphst::Graph;

fn main() {
    let empty_g = Graph::new();
    println!("empty_g:\n{}", empty_g);

    let n_nodes = 5;
    let mat = vec![vec![0.0; n_nodes]; n_nodes];
    let mut g = Graph::from_adjacency_matrix(mat);
    g.add_connection(0, 0);
    g.add_connection(0, 3);
    g.add_connection(4, 2);

    println!("g:\n{}", g);
    println!("The nodes of g are: {:?}", g.get_nodes());
}
