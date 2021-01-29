use std::fmt;

/// The `Graph` struct provides the functionalities to create and manipulate graphs.
/// It can represent directed and undirected graphs, and also
/// weighted edges between the nodes.
pub struct Graph {
    n_nodes: usize,
    adj_mat: Vec<Vec<f32>>,
}

impl Graph {
    /// Creates an empty `Graph`.
    ///
    /// # Examples
    ///
    /// ```
    /// let g = graphst::Graph::new();
    /// ```
    pub fn new() -> Graph {
        Graph {
            n_nodes: 0,
            adj_mat: vec![],
        }
    }

    /// Creates a `Graph` from the definition of the graph edges and the number of nodes.
    ///
    /// # Arguments
    ///
    /// * `n_nodes` - An `usize` value with the number of nodes in the graph.
    /// * `edges` - A vector of tuples with two `usize` values defining each edge (`(src, dest)`).
    ///
    /// # Panics
    ///
    /// * If some edge has an invalid node value.
    /// * If the pair (src, dest) of an edge is repeated.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let edges = vec![(0, 1), (1, 2), (2, 2)];
    /// let g = graphst::Graph::from_edges(n_nodes, edges);
    /// ```
    pub fn from_edges(n_nodes: usize, edges: Vec<(usize, usize)>) -> Graph {
        let mut adj_mat: Vec<Vec<f32>> = vec![vec![0.0; n_nodes]; n_nodes];
        for edge in edges {
            if edge.0 >= n_nodes || edge.1 >= n_nodes {
                panic!(
                    "[Graph::from_edges] Error: The edge {:?} is not valid!",
                    edge
                );
            }
            if adj_mat[edge.0][edge.1] != 0.0 {
                panic!(
                    "[Graph::from_edges] Error: The edge ({})->({}) is repeated!",
                    edge.0, edge.1
                );
            } else {
                adj_mat[edge.0][edge.1] = 1.0;
            }
        }
        Graph { n_nodes, adj_mat }
    }

    /// Creates a `Graph` from the definition of the graph edges (with weight) and the number of nodes.
    ///
    /// # Arguments
    ///
    /// * `n_nodes` - An `usize` value with the number of nodes in the graph.
    /// * `edges` - A vector of triplets with two `usize` values and a `f32` defining each edge (`(src, dest, weight)`).
    ///
    /// # Panics
    ///
    /// * If some edge has an invalid node value.
    /// * If the pair (src, dest, _) of an edge is repeated.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let edges = vec![(0, 1, 2.0), (1, 2, 1.5), (2, 2, -0.5)];
    /// let g = graphst::Graph::from_weighted_edges(n_nodes, edges);
    /// ```
    pub fn from_weighted_edges(n_nodes: usize, edges: Vec<(usize, usize, f32)>) -> Graph {
        let mut adj_mat: Vec<Vec<f32>> = vec![vec![0.0; n_nodes]; n_nodes];
        for edge in edges {
            if edge.0 >= n_nodes || edge.1 >= n_nodes {
                panic!(
                    "[Graph::from_weighted_edges] Error: The edge {:?} is not valid!",
                    edge
                );
            }
            if adj_mat[edge.0][edge.1] != 0.0 {
                panic!(
                    "[Graph::from_weighted_edges] Error: The edge ({})->({}) is repeated!",
                    edge.0, edge.1
                );
            } else {
                adj_mat[edge.0][edge.1] = edge.2;
            }
        }
        Graph { n_nodes, adj_mat }
    }

    /// Creates a `Graph` from an adjacency matrix. The `f32` values represent the weights of the connections.
    /// A `f32` value of 0.0 means that there is no connection.
    ///
    /// # Arguments
    ///
    /// * `adj_mat` - A squared matrix of `f32` values.
    ///
    /// # Panics
    ///
    /// * If the adjacency matrix is not squared.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 5;
    /// let mut adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// adj_mat[0][4] = 1.0;
    /// adj_mat[4][2] = 2.0;
    /// let g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// ```
    pub fn from_adjacency_matrix(adj_mat: Vec<Vec<f32>>) -> Graph {
        let n_nodes = adj_mat.len();
        for node_edges in &adj_mat {
            if node_edges.len() != n_nodes {
                panic!(
                    "[Graph::from_adjacency_matrix] Error: The adjacency matrix is not squared!"
                );
            }
        }
        Graph { n_nodes, adj_mat }
    }

    /// Returns a vector with nodes (`usize` references) of the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// let g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// let nodes = g.get_nodes();
    /// assert_eq!(nodes, vec![0, 1, 2]);
    /// ```
    pub fn get_nodes(&self) -> Vec<usize> {
        (0..self.n_nodes).collect()
    }

    /// Returns a reference to the bidimensional vector of `f32` with the adjacency
    /// matrix of the graph. The `f32` values are the weights of the edges, and a
    /// value of 0.0 means that there is no edge between those nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let edges = vec![(0, 1), (1, 2), (2, 2)];
    /// let g = graphst::Graph::from_edges(n_nodes, edges);
    /// let g_adj_mat = g.get_adjacency_matrix();
    /// let test_mat: Vec<Vec<f32>> = vec![
    ///     vec![0.0, 1.0, 0.0],
    ///     vec![0.0, 0.0, 1.0],
    ///     vec![0.0, 0.0, 1.0],
    /// ];
    /// assert_eq!(g_adj_mat, &test_mat);
    /// ```
    pub fn get_adjacency_matrix<'a>(&'a self) -> &'a Vec<Vec<f32>> {
        &self.adj_mat
    }

    /// Returns a vector with the nodes that are neighbors of the node passed as a parameter.
    ///
    /// # Arguments
    ///
    /// * `node` - `usize` value of the node to find its neighbours from.
    ///
    /// # Panics
    ///
    /// * If the node passed as a parameter is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let edges = vec![(0, 1), (1, 2), (2, 1), (2, 2)];
    /// let g = graphst::Graph::from_edges(n_nodes, edges);
    /// let neighbors_of_2 = g.get_neighbors_of(2);
    /// assert_eq!(neighbors_of_2, vec![1, 2]);
    /// ```
    pub fn get_neighbors_of(&self, node: usize) -> Vec<usize> {
        if node >= self.n_nodes {
            panic!(
                "[Graph::get_neighbors_of] Error: The node {} is not valid",
                node
            );
        }
        self.adj_mat[node]
            .iter()
            .enumerate()
            .filter(|(_, w)| **w != 0.0)
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Gets the weight of the edge from the node `src` to `dest`. If the graph is not weighted
    /// the value will be `1.0`. If the edge doesn't exist the returned value will be `None`.
    ///
    /// # Arguments
    ///
    /// * `src` - `usize` value of the source node.
    /// * `dest` - `usize` value of the destination node.
    ///
    /// # Panics
    ///
    /// * If the value of `src` or `dest` is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// let mut g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// g.add_connection(1, 2);
    /// g.add_weighted_connection(0, 1, 3.5);
    /// let edge_1_2 = g.get_edge(1, 2).expect("The edge doesn't exist");
    /// let edge_0_1 = g.get_edge(0, 1).expect("The edge doesn't exist");
    /// let edge_0_2 = g.get_edge(0, 2).unwrap_or(0.0); // this edge doesn't exist
    /// assert_eq!(edge_1_2, 1.0);
    /// assert_eq!(edge_0_1, 3.5);
    /// assert_eq!(edge_0_2, 0.0);
    /// ```
    pub fn get_edge(&self, src: usize, dest: usize) -> Option<f32> {
        if src >= self.n_nodes {
            panic!(
                "[Graph::get_edge] Error: The source node {} is not valid!",
                src
            );
        } else if dest >= self.n_nodes {
            panic!(
                "[Graph::get_edge] Error: The destination node {} is not valid!",
                dest
            );
        }
        if self.adj_mat[src][dest] != 0.0 {
            return Some(self.adj_mat[src][dest]);
        } else {
            return None;
        }
    }

    /// Adds a node to the graph without any edge.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut g = graphst::Graph::new(); // empty graph
    /// g.add_node();
    /// g.add_node();
    /// let g_nodes = g.get_nodes();
    /// assert_eq!(g_nodes, vec![0, 1]);
    /// ```
    pub fn add_node(&mut self) {
        for node in &mut self.adj_mat {
            node.push(0.0); // add a new value for setting the edges to the new node
        }
        self.n_nodes += 1;
        self.adj_mat.push(vec![0.0; self.n_nodes]); // add the new node edges vector
    }

    /// Sets a directed connection from the node `src` to the node `dest`.
    /// The weight of the edge is set to `1.0`.
    ///
    /// # Arguments
    ///
    /// * `src` - `usize` value of the source node.
    /// * `dest` - `usize` value of the destination node.
    ///
    /// # Panics
    ///
    /// * If the value of `src` or `dest` is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// let mut g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// g.add_connection(0, 1);
    /// g.add_connection(1, 2);
    /// g.add_connection(2, 2);
    /// ```
    pub fn add_connection(&mut self, src: usize, dest: usize) {
        if src >= self.n_nodes {
            panic!(
                "[Graph::add_connection] Error: The source node {} is not valid!",
                src
            );
        } else if dest >= self.n_nodes {
            panic!(
                "[Graph::add_connection] Error: The destination node {} is not valid!",
                dest
            );
        }
        self.adj_mat[src][dest] = 1.0;
    }

    /// Sets an undirected connection between nodes `src` and `dest`.
    /// The weight of the edge is set to `1.0`.
    ///
    /// # Arguments
    ///
    /// * `src` - `usize` value of the source node.
    /// * `dest` - `usize` value of the destination node.
    ///
    /// # Panics
    ///
    /// * If the value of `src` or `dest` is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// let mut g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// g.add_undirected_connection(0, 1);
    /// g.add_undirected_connection(2, 1);
    /// ```
    pub fn add_undirected_connection(&mut self, src: usize, dest: usize) {
        if src >= self.n_nodes {
            panic!(
                "[Graph::add_undirected_connection] Error: The source node {} is not valid!",
                src
            );
        } else if dest >= self.n_nodes {
            panic!(
                "[Graph::add_undirected_connection] Error: The destination node {} is not valid!",
                dest
            );
        }
        self.adj_mat[src][dest] = 1.0;
        self.adj_mat[dest][src] = 1.0;
    }

    /// Sets a directed connection from the node `src` to the node `dest`.
    /// The weight of the edge is set to the value of the parameter `weight`.
    ///
    /// # Arguments
    ///
    /// * `src` - `usize` value of the source node.
    /// * `dest` - `usize` value of the destination node.
    /// * `weight` - `f32` value of the edge weight.
    ///
    /// # Panics
    ///
    /// * If the value of `src` or `dest` is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// let mut g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// g.add_weighted_connection(0, 1, 2.0);
    /// g.add_weighted_connection(1, 2, 3.2);
    /// g.add_weighted_connection(2, 2, -1.0);
    /// ```
    pub fn add_weighted_connection(&mut self, src: usize, dest: usize, weight: f32) {
        if src >= self.n_nodes {
            panic!(
                "[Graph::add_weighted_connection] Error: The source node {} is not valid!",
                src
            );
        } else if dest >= self.n_nodes {
            panic!(
                "[Graph::add_weighted_connection] Error: The destination node {} is not valid!",
                dest
            );
        }
        self.adj_mat[src][dest] = weight;
    }

    /// Sets an undirected connection between nodes `src` and `dest`.
    /// The weight of the edge is set to the value of the parameter `weight`.
    ///
    /// # Arguments
    ///
    /// * `src` - `usize` value of the source node.
    /// * `dest` - `usize` value of the destination node.
    /// * `weight` - `f32` value of the edge weight.
    ///
    /// # Panics
    ///
    /// * If the value of `src` or `dest` is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let adj_mat = vec![vec![0.0; 3]; 3]; // graph with 3 nodes
    /// let mut g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// g.add_undirected_weighted_connection(0, 1, 1.5);
    /// g.add_undirected_weighted_connection(2, 1, 2.0);
    /// ```
    pub fn add_undirected_weighted_connection(&mut self, src: usize, dest: usize, weight: f32) {
        if src >= self.n_nodes {
            panic!(
                "[Graph::add_undirected_weighted_connection] Error: The source node {} is not valid!", 
                src
            );
        } else if dest >= self.n_nodes {
            panic!(
                "[Graph::add_undirected_weighted_connection] Error: The destination node {} is not valid!", 
                dest
            );
        }
        self.adj_mat[src][dest] = weight;
        self.adj_mat[dest][src] = weight;
    }
}

impl fmt::Display for Graph {
    /// Shows the info of the graph.
    /// The edges are represented in the format `src -(weight)-> dest`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph(edges=[\n")?;
        for (src, node) in self.adj_mat.iter().enumerate() {
            for (dest, weight) in node.iter().enumerate() {
                if *weight != 0.0 {
                    write!(f, "{} -({})-> {},\n", src, weight, dest)?;
                }
            }
        }
        write!(f, "], n_nodes={})", self.n_nodes)
    }
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    #[test]
    fn constructor_new() {
        let g = Graph::new();
        assert_eq!(g.n_nodes, 0);
        assert_eq!(g.adj_mat.len(), 0);
    }

    #[test]
    fn constructor_from_edges() {
        let n_nodes = 3;
        let edges = vec![(0, 1), (1, 2), (2, 2)];
        let g = Graph::from_edges(n_nodes, edges);
        assert_eq!(g.n_nodes, 3);
        // Check that the adjacency matrix is squared
        assert_eq!(g.adj_mat.len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[0].len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[1].len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[2].len(), 3, "The matrix is not squared.");
        // Check all the adjacency matrix values
        assert_eq!(
            g.adj_mat[0][0], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[0][0]
        );
        assert_eq!(
            g.adj_mat[0][1], 1.0,
            "This edge should be 1.0 but is {}",
            g.adj_mat[0][1]
        );
        assert_eq!(
            g.adj_mat[0][2], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[0][2]
        );
        assert_eq!(
            g.adj_mat[1][0], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[1][0]
        );
        assert_eq!(
            g.adj_mat[1][1], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[1][1]
        );
        assert_eq!(
            g.adj_mat[1][2], 1.0,
            "This edge should be 1.0 but is {}",
            g.adj_mat[1][2]
        );
        assert_eq!(
            g.adj_mat[2][0], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[2][0]
        );
        assert_eq!(
            g.adj_mat[2][1], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[2][1]
        );
        assert_eq!(
            g.adj_mat[2][2], 1.0,
            "This edge should be 1.0 but is {}",
            g.adj_mat[2][2]
        );
    }

    #[test]
    #[should_panic(expected = "not valid")]
    fn constructor_from_edges_panic_not_valid_edge() {
        let n_nodes = 1;
        let edges = vec![(0, 1)];
        let _g = Graph::from_edges(n_nodes, edges);
    }

    #[test]
    #[should_panic(expected = "is repeated")]
    fn constructor_from_edges_panic_repeated_edge() {
        let n_nodes = 3;
        let edges = vec![(0, 1), (1, 2), (1, 2)];
        let _g = Graph::from_edges(n_nodes, edges);
    }

    #[test]
    fn constructor_from_weighted_edges() {
        let n_nodes = 3;
        let edges = vec![(0, 1, 2.3), (1, 2, 1.2), (2, 2, -1.0)];
        let g = Graph::from_weighted_edges(n_nodes, edges);
        assert_eq!(g.n_nodes, 3);
        // Check that the adjacency matrix is squared
        assert_eq!(g.adj_mat.len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[0].len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[1].len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[2].len(), 3, "The matrix is not squared.");
        // Check all the adjacency matrix values
        assert_eq!(
            g.adj_mat[0][0], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[0][0]
        );
        assert_eq!(
            g.adj_mat[0][1], 2.3,
            "This edge should be 2.3 but is {}",
            g.adj_mat[0][1]
        );
        assert_eq!(
            g.adj_mat[0][2], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[0][2]
        );
        assert_eq!(
            g.adj_mat[1][0], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[1][0]
        );
        assert_eq!(
            g.adj_mat[1][1], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[1][1]
        );
        assert_eq!(
            g.adj_mat[1][2], 1.2,
            "This edge should be 1.2 but is {}",
            g.adj_mat[1][2]
        );
        assert_eq!(
            g.adj_mat[2][0], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[2][0]
        );
        assert_eq!(
            g.adj_mat[2][1], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[2][1]
        );
        assert_eq!(
            g.adj_mat[2][2], -1.0,
            "This edge should be -1.0 but is {}",
            g.adj_mat[2][2]
        );
    }

    #[test]
    #[should_panic(expected = "not valid")]
    fn constructor_from_weighted_edges_panic_not_valid_edge() {
        let n_nodes = 2;
        let edges = vec![(0, 1, 1.5), (2, 0, 1.1)];
        let _g = Graph::from_weighted_edges(n_nodes, edges);
    }

    #[test]
    #[should_panic(expected = "is repeated")]
    fn constructor_from_weighted_edges_panic_repeated_edge() {
        let n_nodes = 3;
        let edges = vec![(0, 1, 2.3), (1, 2, 1.2), (1, 2, -1.0)];
        let _g = Graph::from_weighted_edges(n_nodes, edges);
    }

    #[test]
    fn constructor_from_adjacency_matrix() {
        let n_nodes = 3;
        let mut adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        adj_mat[0][1] = 1.0;
        adj_mat[1][1] = 0.5;
        adj_mat[1][2] = 2.0;
        adj_mat[2][2] = -1.0;
        let g = Graph::from_adjacency_matrix(adj_mat);
        assert_eq!(
            g.adj_mat[0][0], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[0][0]
        );
        assert_eq!(
            g.adj_mat[0][1], 1.0,
            "This edge should be 1.0 but is {}",
            g.adj_mat[0][1]
        );
        assert_eq!(
            g.adj_mat[0][2], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[0][2]
        );
        assert_eq!(
            g.adj_mat[1][0], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[1][0]
        );
        assert_eq!(
            g.adj_mat[1][1], 0.5,
            "This edge should be 0.5 but is {}",
            g.adj_mat[1][1]
        );
        assert_eq!(
            g.adj_mat[1][2], 2.0,
            "This edge should be 2.0 but is {}",
            g.adj_mat[1][2]
        );
        assert_eq!(
            g.adj_mat[2][0], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[2][0]
        );
        assert_eq!(
            g.adj_mat[2][1], 0.0,
            "This edge should be 0.0 but is {}",
            g.adj_mat[2][1]
        );
        assert_eq!(
            g.adj_mat[2][2], -1.0,
            "This edge should be -1.0 but is {}",
            g.adj_mat[2][2]
        );
    }

    #[test]
    #[should_panic(expected = "not squared")]
    fn constructor_from_adjacency_matrix_panic_not_squared() {
        let adj_mat = vec![vec![0.0, 1.1], vec![1.0, 0.0, 0.0]];
        let _g = Graph::from_adjacency_matrix(adj_mat);
    }

    #[test]
    fn get_nodes_check_values() {
        let n_nodes = 4;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let g = Graph::from_adjacency_matrix(adj_mat);
        let nodes = g.get_nodes();
        assert_eq!(nodes, vec![0, 1, 2, 3]);
    }

    #[test]
    fn get_nodes_check_empty_case() {
        let g = Graph::new();
        let nodes = g.get_nodes();
        assert_eq!(nodes, vec![]);
    }

    #[test]
    fn get_adjacency_matrix_check_values() {
        let n_nodes = 3;
        let edges = vec![(0, 1), (1, 2), (2, 2)];
        let g = Graph::from_edges(n_nodes, edges);
        let g_adj_mat = g.get_adjacency_matrix();
        let test_mat: Vec<Vec<f32>> = vec![
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
            vec![0.0, 0.0, 1.0],
        ];
        assert_eq!(g_adj_mat, &test_mat);
    }

    #[test]
    fn get_neighbors_of_check_values() {
        let n_nodes = 3;
        let edges = vec![(0, 1), (1, 2), (2, 1), (2, 2)];
        let g = Graph::from_edges(n_nodes, edges);
        assert_eq!(g.get_neighbors_of(0), vec![1]);
        assert_eq!(g.get_neighbors_of(1), vec![2]);
        assert_eq!(g.get_neighbors_of(2), vec![1, 2]);
    }

    #[test]
    #[should_panic(expected = "not valid")]
    fn get_neighbors_of_panic_not_valid_node() {
        let n_nodes = 3;
        let edges = vec![(0, 1), (1, 2), (2, 1), (2, 2)];
        let g = Graph::from_edges(n_nodes, edges);
        let _neighbors = g.get_neighbors_of(3);
    }

    #[test]
    fn get_edge_check_values() {
        let adj_mat: Vec<Vec<f32>> = vec![
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.5],
            vec![0.0, 0.0, 2.0],
        ];
        let g = Graph::from_adjacency_matrix(adj_mat);
        let edge_0_1 = g.get_edge(0, 1).expect("The edge doesn't exist");
        let edge_1_2 = g.get_edge(1, 2).expect("The edge doesn't exist");
        let edge_2_2 = g.get_edge(2, 2).expect("The edge doesn't exist");
        assert_eq!(edge_0_1, 1.0);
        assert_eq!(edge_1_2, 0.5);
        assert_eq!(edge_2_2, 2.0);
    }

    #[test]
    #[should_panic(expected = "source node")]
    fn get_edge_panic_not_valid_src() {
        let adj_mat: Vec<Vec<f32>> = vec![
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.5],
            vec![0.0, 0.0, 2.0],
        ];
        let g = Graph::from_adjacency_matrix(adj_mat);
        let _edge_0_3 = g.get_edge(3, 2).expect("The edge doesn't exist");
    }

    #[test]
    #[should_panic(expected = "destination node")]
    fn get_edge_panic_not_valid_dest() {
        let adj_mat: Vec<Vec<f32>> = vec![
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.5],
            vec![0.0, 0.0, 2.0],
        ];
        let g = Graph::from_adjacency_matrix(adj_mat);
        let _edge_0_3 = g.get_edge(0, 3).expect("The edge doesn't exist");
    }

    #[test]
    fn add_node_check_status() {
        let n_nodes = 3;
        let edges = vec![(0, 1), (1, 2), (2, 2)];
        let mut g = Graph::from_edges(n_nodes, edges);
        let g_nodes = g.get_nodes();
        assert_eq!(g_nodes, vec![0, 1, 2]);
        g.add_node();
        let g_nodes = g.get_nodes();
        assert_eq!(g_nodes, vec![0, 1, 2, 3]);
        g.add_node();
        let g_nodes = g.get_nodes();
        assert_eq!(g_nodes, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn add_connection_check_status() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_connection(0, 1);
        g.add_connection(1, 2);
        g.add_connection(2, 2);
        assert_eq!(g.adj_mat[0][1], 1.0);
        assert_eq!(g.adj_mat[1][2], 1.0);
        assert_eq!(g.adj_mat[2][2], 1.0);
    }

    #[test]
    #[should_panic(expected = "source node")]
    fn add_connection_panic_not_valid_src() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_connection(3, 1);
    }

    #[test]
    #[should_panic(expected = "destination node")]
    fn add_connection_panic_not_valid_dest() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_connection(2, 3);
    }

    #[test]
    fn add_undirected_connection_check_status() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_undirected_connection(0, 1);
        g.add_undirected_connection(2, 2);
        assert_eq!(g.adj_mat[0][1], 1.0);
        assert_eq!(g.adj_mat[1][0], 1.0);
        assert_eq!(g.adj_mat[2][2], 1.0);
    }

    #[test]
    #[should_panic(expected = "source node")]
    fn add_undirected_connection_panic_not_valid_src() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_undirected_connection(3, 1);
    }

    #[test]
    #[should_panic(expected = "destination node")]
    fn add_undirected_connection_panic_not_valid_dest() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_undirected_connection(2, 3);
    }

    #[test]
    fn add_weighted_connection_check_status() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_weighted_connection(0, 1, 2.0);
        g.add_weighted_connection(1, 2, 1.5);
        g.add_weighted_connection(2, 2, -1.2);
        assert_eq!(g.adj_mat[0][1], 2.0);
        assert_eq!(g.adj_mat[1][2], 1.5);
        assert_eq!(g.adj_mat[2][2], -1.2);
    }

    #[test]
    #[should_panic(expected = "source node")]
    fn add_weighted_connection_panic_not_valid_src() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_weighted_connection(3, 1, 2.0);
    }

    #[test]
    #[should_panic(expected = "destination node")]
    fn add_weighted_connection_panic_not_valid_dest() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_weighted_connection(2, 3, 2.0);
    }

    #[test]
    fn add_undirected_weighted_connection_check_status() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_undirected_weighted_connection(0, 1, 3.2);
        g.add_undirected_weighted_connection(2, 2, 2.0);
        assert_eq!(g.adj_mat[0][1], 3.2);
        assert_eq!(g.adj_mat[1][0], 3.2);
        assert_eq!(g.adj_mat[2][2], 2.0);
    }

    #[test]
    #[should_panic(expected = "source node")]
    fn add_undirected_weighted_connection_panic_not_valid_src() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_undirected_weighted_connection(3, 1, 2.0);
    }

    #[test]
    #[should_panic(expected = "destination node")]
    fn add_undirected_weighted_connection_panic_not_valid_dest() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_undirected_weighted_connection(2, 3, 2.0);
    }
}
