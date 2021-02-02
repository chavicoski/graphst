use crate::graph::Graph;

fn min_distance_node<G>(g: &G, dist: &Vec<f32>, visited: &Vec<bool>) -> usize
where
    G: Graph,
{
    let mut min_dist = f32::INFINITY;
    let mut min_idx = g.get_n_nodes(); // default is an invalid node

    for node in g.get_nodes() {
        if dist[node] < min_dist && visited[node] == false {
            min_dist = dist[node];
            min_idx = node;
        }
    }

    return min_idx;
}

/// Given a graph (that implements `Graph`) and a source node, returns the
/// shortest path to each node from the source provided.
///
/// # Examples
///
/// ```
/// let n_nodes = 3;
/// let edges = vec![(0, 1), (1, 2), (2, 2)];
/// let g = graphst::UGraph::from_edges(n_nodes, edges);
/// let short_paths = graphst::algorithm::dijkstra(&g, 0);
/// assert_eq!(short_paths, vec![0.0, 1.0, 2.0]);
/// ```
pub fn dijkstra<G>(g: &G, src: usize) -> Vec<f32>
where
    G: Graph,
{
    // dist: For keeping track of the current closest distance to
    //       each node during the algorithm iterations
    let mut dist = vec![f32::INFINITY; g.get_n_nodes()];
    // visited: To know which nodes we have visited and we already have a minimum path
    let mut visited = vec![false; g.get_n_nodes()];

    dist[src] = 0.0; // Initialize with distance to src

    for _ in g.get_nodes() {
        // Select the closest not visited node
        let current = min_distance_node(g, &dist, &visited);
        visited[current] = true;
        for n in g.get_nodes() {
            let edge_weight = match g.get_edge(current, n) {
                Some(edge) => edge, // The edge exists, take the weight
                None => continue,   // There is no edge, skip to the next node
            };
            if visited[n] == false && dist[n] > dist[current] + edge_weight {
                dist[n] = dist[current] + edge_weight; // Set the new best distance
            }
        }
    }

    return dist;
}
