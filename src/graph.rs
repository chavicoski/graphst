use std::fmt;

/// The `Graph` struct provides the functionalities to create and manipulate `undirected graphs`.
/// It can use weighted edges or default edges (with weight `1.0`). The weights of the nodes are
/// of type `f32`, and the nodes are referenced by `usize` values from `0` to `n_nodes-1`.
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

    /// Creates a `Graph` from the definition of the graph edges and
    /// the number of nodes. The edges must be undirected.
    ///
    /// # Arguments
    ///
    /// * `n_nodes` - An `usize` value with the number of nodes in the graph.
    /// * `edges` - A vector of tuples with two `usize` values defining each
    ///             edge (`(node1, node2)`).
    ///
    /// # Panics
    ///
    /// * If some edge has an invalid node value.
    /// * If the edge `(node1, node2)` is repeated. Note that `(node1, node2)` is
    ///   the same edge than `(node2, node1)` because the graph is undirected.
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
                    "[Graph::from_edges] Error: The edge ({})--({}) is repeated!",
                    edge.0, edge.1
                );
            } else {
                // Set the edge in both directions
                adj_mat[edge.0][edge.1] = 1.0;
                adj_mat[edge.1][edge.0] = 1.0;
            }
        }
        Graph { n_nodes, adj_mat }
    }

    /// Creates a `Graph` from the definition of the graph edges (with weight)
    /// and the number of nodes.
    ///
    /// # Arguments
    ///
    /// * `n_nodes` - An `usize` value with the number of nodes in the graph.
    /// * `edges` - A vector of triplets with two `usize` values and a `f32`
    ///             defining each edge (`(node1, node2, weight)`).
    ///
    /// # Panics
    ///
    /// * If some edge has an invalid node value.
    /// * If the edge `(node1, node2, _)` is repeated. Note that `(node1, node2, _)` is
    ///   the same edge than `(node2, node1, _)` because the graph is undirected.
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
                    "[Graph::from_weighted_edges] Error: The edge ({})--({}) is repeated!",
                    edge.0, edge.1
                );
            } else {
                // Set the edge in both directions
                adj_mat[edge.0][edge.1] = edge.2;
                adj_mat[edge.1][edge.0] = edge.2;
            }
        }
        Graph { n_nodes, adj_mat }
    }

    /// Creates a `Graph` from an adjacency matrix. The `f32` values represent the weights
    /// of the edges. A `f32` value of 0.0 means that there is no edge.
    ///
    /// # Arguments
    ///
    /// * `adj_mat` - A squared matrix of `f32` values.
    ///
    /// # Panics
    ///
    /// * If the adjacency matrix is not squared.
    /// * If the adjacency matrix is not valid for an undirected graph.
    ///   For each pair of nodes `adj_mat[n1][n2] == adj_mat[n2][n1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 5;
    /// let mut adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// adj_mat[0][4] = 1.0;
    /// adj_mat[4][0] = 1.0;
    /// adj_mat[2][4] = 2.0;
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
        let g = Graph { n_nodes, adj_mat };
        if !g.check_is_undirected() {
            panic!(
                "[Graph::from_adjacency_matrix] Error: The adjacency matrix provided is \
                not a valid matrix for an undirected graph!"
            );
        }
        return g;
    }

    /// Returns the number of nodes in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// let g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// let nodes = g.get_n_nodes();
    /// assert_eq!(nodes, 3);
    /// ```
    pub fn get_n_nodes(&self) -> usize {
        self.n_nodes
    }

    /// Returns a vector with the nodes (`usize` references) of the graph.
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
    /// value of `0.0` means that there is no edge between those nodes.
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
    ///     vec![1.0, 0.0, 1.0],
    ///     vec![0.0, 1.0, 1.0],
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
    /// let edges = vec![(0, 1), (1, 2), (2, 2)];
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

    /// Gets the weight of the edge connecting the nodes `node1` and `node2`. If the graph is
    /// not weighted the value will be `1.0`. If the edge doesn't exist the returned value
    /// will be `None`.
    ///
    /// # Arguments
    ///
    /// * `node1` - `usize` value of the first node.
    /// * `node2` - `usize` value of the second node.
    ///
    /// # Panics
    ///
    /// * If the value of `node1` or `node2` is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// let mut g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// g.add_edge(1, 2);
    /// g.add_weighted_edge(0, 1, 3.5);
    /// let edge_1_2 = g.get_edge(1, 2).expect("The edge doesn't exist");
    /// let edge_0_1 = g.get_edge(0, 1).expect("The edge doesn't exist");
    /// let edge_0_2 = g.get_edge(0, 2).unwrap_or(0.0); // this edge doesn't exist
    /// assert_eq!(edge_1_2, 1.0);
    /// assert_eq!(edge_0_1, 3.5);
    /// assert_eq!(edge_0_2, 0.0);
    /// ```
    pub fn get_edge(&self, node1: usize, node2: usize) -> Option<f32> {
        if node1 >= self.n_nodes {
            panic!(
                "[Graph::get_edge] Error: The first node {} is not valid!",
                node1
            );
        } else if node2 >= self.n_nodes {
            panic!(
                "[Graph::get_edge] Error: The second node {} is not valid!",
                node2
            );
        }
        if self.adj_mat[node1][node2] != 0.0 {
            return Some(self.adj_mat[node1][node2]);
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

    /// Sets an undirected edge between nodes `node1` and `node2`.
    /// The weight of the edge is set to `1.0`.
    ///
    /// # Arguments
    ///
    /// * `node1` - `usize` value of the first node.
    /// * `node2` - `usize` value of the second node.
    ///
    /// # Panics
    ///
    /// * If the value of `node1` or `node2` is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let n_nodes = 3;
    /// let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
    /// let mut g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// g.add_edge(0, 1);
    /// g.add_edge(2, 1);
    /// ```
    pub fn add_edge(&mut self, node1: usize, node2: usize) {
        if node1 >= self.n_nodes {
            panic!(
                "[Graph::add_edge] Error: The first node {} is not valid!",
                node1
            );
        } else if node2 >= self.n_nodes {
            panic!(
                "[Graph::add_edge] Error: The second node {} is not valid!",
                node2
            );
        }
        self.adj_mat[node1][node2] = 1.0;
        self.adj_mat[node2][node1] = 1.0;
    }

    /// Sets an undirected edge between nodes `node1` and `node2`.
    /// The weight of the edge is set to the value of the parameter `weight`.
    ///
    /// # Arguments
    ///
    /// * `node1` - `usize` value of the first node.
    /// * `node2` - `usize` value of the second node.
    /// * `weight` - `f32` value of the edge weight.
    ///
    /// # Panics
    ///
    /// * If the value of `node1` or `node2` is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let adj_mat = vec![vec![0.0; 3]; 3]; // graph with 3 nodes
    /// let mut g = graphst::Graph::from_adjacency_matrix(adj_mat);
    /// g.add_weighted_edge(0, 1, 1.5);
    /// g.add_weighted_edge(2, 1, 2.0);
    /// ```
    pub fn add_weighted_edge(&mut self, node1: usize, node2: usize, weight: f32) {
        if node1 >= self.n_nodes {
            panic!(
                "[Graph::add_weighted_edge] Error: The first node {} \
                 is not valid!",
                node1
            );
        } else if node2 >= self.n_nodes {
            panic!(
                "[Graph::add_weighted_edge] Error: The second node {} \
                 is not valid!",
                node2
            );
        }
        self.adj_mat[node1][node2] = weight;
        self.adj_mat[node2][node1] = weight;
    }

    // Private functions

    fn check_is_undirected(&self) -> bool {
        for n in self.get_nodes() {
            for n2 in n..self.get_n_nodes() {
                if self.adj_mat[n][n2] != self.adj_mat[n2][n] {
                    return false;
                }
            }
        }
        return true;
    }
}

impl fmt::Display for Graph {
    /// Shows the info of the graph.
    /// The edges are represented in the format `node1 -(weigh)- node2`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph(edges=[\n")?;
        for (node1, node) in self.adj_mat.iter().enumerate() {
            for (node2, weight) in node.iter().enumerate() {
                if *weight != 0.0 {
                    write!(f, "{} -({})- {},\n", node1, weight, node2)?;
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
        let edges = vec![(0, 1), (1, 2)];
        let g = Graph::from_edges(n_nodes, edges);
        assert_eq!(g.n_nodes, 3);
        // Check that the adjacency matrix is squared
        assert_eq!(g.adj_mat.len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[0].len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[1].len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[2].len(), 3, "The matrix is not squared.");
        // Check edges weights and symetry (because are undirected)
        assert_eq!(
            g.adj_mat[0][1], 1.0,
            "This edge should be 1.0 but is {}",
            g.adj_mat[0][1]
        );
        assert_eq!(
            g.adj_mat[1][0], g.adj_mat[0][1],
            "This edge should be symetric, {} != {}",
            g.adj_mat[1][0], g.adj_mat[0][1]
        );
        assert_eq!(
            g.adj_mat[1][2], 1.0,
            "This edge should be 1.0 but is {}",
            g.adj_mat[1][2]
        );
        assert_eq!(
            g.adj_mat[2][1], g.adj_mat[1][2],
            "This edge should be symetric, {} != {}",
            g.adj_mat[2][1], g.adj_mat[1][2]
        );
    }

    #[test]
    #[should_panic(expected = "not valid")]
    fn constructor_from_edges_panic_not_valid_edge() {
        let n_nodes = 2;
        let edges = vec![(0, 1), (2, 0)];
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
        let edges = vec![(0, 1, 2.3), (1, 2, 1.2)];
        let g = Graph::from_weighted_edges(n_nodes, edges);
        assert_eq!(g.n_nodes, 3);
        // Check that the adjacency matrix is squared
        assert_eq!(g.adj_mat.len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[0].len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[1].len(), 3, "The matrix is not squared.");
        assert_eq!(g.adj_mat[2].len(), 3, "The matrix is not squared.");
        // Check edges weights and symetry (because are undirected)
        assert_eq!(
            g.adj_mat[0][1], 2.3,
            "This edge should be 2.3 but is {}",
            g.adj_mat[0][1]
        );
        assert_eq!(
            g.adj_mat[1][0], g.adj_mat[0][1],
            "This edge should be symetric, {} != {}",
            g.adj_mat[1][0], g.adj_mat[0][1]
        );
        assert_eq!(
            g.adj_mat[1][2], 1.2,
            "This edge should be 1.2 but is {}",
            g.adj_mat[1][2]
        );
        assert_eq!(
            g.adj_mat[2][1], g.adj_mat[1][2],
            "This edge should be symetric, {} != {}",
            g.adj_mat[2][1], g.adj_mat[1][2]
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
        adj_mat[1][0] = 1.0;
        adj_mat[1][1] = 0.5;
        adj_mat[1][2] = 2.0;
        adj_mat[2][1] = 2.0;
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
            g.adj_mat[1][0], 1.0,
            "This edge should be 1.0 but is {}",
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
            g.adj_mat[2][1], 2.0,
            "This edge should be 2.0 but is {}",
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
        let adj_mat = vec![vec![0.0, 1.1], vec![1.1, 0.0, 0.0]];
        let _g = Graph::from_adjacency_matrix(adj_mat);
    }

    #[test]
    fn get_n_nodes_check_value() {
        let n_nodes = 4;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let g = Graph::from_adjacency_matrix(adj_mat);
        let nodes = g.get_n_nodes();
        assert_eq!(nodes, 4);
    }

    #[test]
    fn get_n_nodes_check_empty_graph() {
        let g = Graph::new();
        let nodes = g.get_n_nodes();
        assert_eq!(nodes, 0);
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
            vec![1.0, 0.0, 1.0],
            vec![0.0, 1.0, 1.0],
        ];
        assert_eq!(g_adj_mat, &test_mat);
    }

    #[test]
    fn get_neighbors_of_check_values() {
        let n_nodes = 3;
        let edges = vec![(0, 1), (1, 2), (2, 2)];
        let g = Graph::from_edges(n_nodes, edges);
        assert_eq!(g.get_neighbors_of(0), vec![1]);
        assert_eq!(g.get_neighbors_of(1), vec![0, 2]);
        assert_eq!(g.get_neighbors_of(2), vec![1, 2]);
    }

    #[test]
    #[should_panic(expected = "not valid")]
    fn get_neighbors_of_panic_not_valid_node() {
        let n_nodes = 3;
        let edges = vec![(0, 1), (1, 2), (2, 2)];
        let g = Graph::from_edges(n_nodes, edges);
        let _neighbors = g.get_neighbors_of(3);
    }

    #[test]
    fn get_edge_check_values() {
        let adj_mat: Vec<Vec<f32>> = vec![
            vec![0.0, 1.0, 0.0],
            vec![1.0, 0.0, 0.5],
            vec![0.0, 0.5, 2.0],
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
    #[should_panic(expected = "first node")]
    fn get_edge_panic_not_valid_first_node() {
        let adj_mat: Vec<Vec<f32>> = vec![
            vec![0.0, 1.0, 0.0],
            vec![1.0, 0.0, 0.5],
            vec![0.0, 0.5, 2.0],
        ];
        let g = Graph::from_adjacency_matrix(adj_mat);
        let _edge_0_3 = g.get_edge(3, 2).expect("The edge doesn't exist");
    }

    #[test]
    #[should_panic(expected = "second node")]
    fn get_edge_panic_not_valid_second_node() {
        let adj_mat: Vec<Vec<f32>> = vec![
            vec![0.0, 1.0, 0.0],
            vec![1.0, 0.0, 0.5],
            vec![0.0, 0.5, 2.0],
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
    fn add_edge_check_status() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_edge(0, 1);
        g.add_edge(2, 2);
        assert_eq!(g.adj_mat[0][1], 1.0);
        assert_eq!(g.adj_mat[1][0], 1.0);
        assert_eq!(g.adj_mat[2][2], 1.0);
    }

    #[test]
    #[should_panic(expected = "first node")]
    fn add_edge_panic_not_valid_first_node() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_edge(3, 1);
    }

    #[test]
    #[should_panic(expected = "second node")]
    fn add_edge_panic_not_valid_second_node() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_edge(2, 3);
    }

    #[test]
    fn add_weighted_edge_check_status() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_weighted_edge(0, 1, 3.2);
        g.add_weighted_edge(2, 2, 2.0);
        assert_eq!(g.adj_mat[0][1], 3.2);
        assert_eq!(g.adj_mat[1][0], 3.2);
        assert_eq!(g.adj_mat[2][2], 2.0);
    }

    #[test]
    #[should_panic(expected = "first node")]
    fn add_weighted_edge_panic_not_valid_first_node() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_weighted_edge(3, 1, 2.0);
    }

    #[test]
    #[should_panic(expected = "second node")]
    fn add_weighted_edge_panic_not_valid_second_node() {
        let n_nodes = 3;
        let adj_mat = vec![vec![0.0; n_nodes]; n_nodes];
        let mut g = Graph::from_adjacency_matrix(adj_mat);
        g.add_weighted_edge(2, 3, 2.0);
    }

    #[test]
    fn check_is_undirected_must_be_true() {
        let adj_mat: Vec<Vec<f32>> = vec![
            vec![0.0, 1.0, 0.0],
            vec![1.0, 0.0, 0.5],
            vec![0.0, 0.5, 2.0],
        ];
        let g = Graph::from_adjacency_matrix(adj_mat);
        assert_eq!(g.check_is_undirected(), true);
        let adj_mat: Vec<Vec<f32>> = vec![
            vec![2.0, 0.0, 1.0],
            vec![0.0, 1.0, 0.0],
            vec![1.0, 0.0, 2.0],
        ];
        let g = Graph::from_adjacency_matrix(adj_mat);
        assert_eq!(g.check_is_undirected(), true);
    }

    #[test]
    #[should_panic(expected = "not a valid matrix for an undirected graph")]
    fn check_is_undirected_panic_not_valid_for_undirected() {
        let adj_mat: Vec<Vec<f32>> = vec![
            vec![0.0, 4.0, 0.0],
            vec![1.0, 0.0, 0.5],
            vec![0.0, 0.5, 2.0],
        ];
        let _g = Graph::from_adjacency_matrix(adj_mat);
    }
}
