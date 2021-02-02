/// The `Graph` trait provides the basic common functions that every graph struct implements.
pub trait Graph {
    fn get_n_nodes(&self) -> usize;
    fn get_nodes(&self) -> Vec<usize>;
    fn get_adjacency_matrix(&self) -> &Vec<Vec<f32>>;
    fn get_edge(&self, node1: usize, node2: usize) -> Option<f32>;
    fn add_node(&mut self);
    fn add_edge(&mut self, node1: usize, node2: usize);
    fn add_weighted_edge(&mut self, node1: usize, node2: usize, weight: f32);
}
