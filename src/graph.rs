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
            adj_mat[edge.0][edge.1] = 1.0;
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
                    "[Graph::from_edges] Error: The edge {:?} is not valid!",
                    edge
                );
            }
            adj_mat[edge.0][edge.1] = edge.2;
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
    pub fn get_nodes(self) -> Vec<usize> {
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
    /// let test_mat: Vec<Vec<f32>> = vec![vec![0.0, 1.0, 0.0],
    ///                                    vec![0.0, 0.0, 1.0],
    ///                                    vec![0.0, 0.0, 1.0]];
    /// assert_eq!(g_adj_mat, &test_mat);
    /// ```
    pub fn get_adjacency_matrix<'a>(&'a self) -> &'a Vec<Vec<f32>> {
        &self.adj_mat
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
            panic!("[Graph::add_connection] Error: The source node is not valid!");
        } else if dest >= self.n_nodes {
            panic!("[Graph::add_connection] Error: The destination node is not valid!");
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
            panic!("[Graph::add_undirected_connection] Error: The source node is not valid!");
        } else if dest >= self.n_nodes {
            panic!("[Graph::add_undirected_connection] Error: The destination node is not valid!");
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
            panic!("[Graph::add_weighted_connection] Error: The source node is not valid!");
        } else if dest >= self.n_nodes {
            panic!("[Graph::add_weighted_connection] Error: The destination node is not valid!");
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
                "[Graph::add_undirected_weighted_connection] Error: The source node is not valid!"
            );
        } else if dest >= self.n_nodes {
            panic!("[Graph::add_undirected_weighted_connection] Error: The destination node is not valid!");
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
