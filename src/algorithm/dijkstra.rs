use crate::graph::Graph;

fn min_distance_node(g: &Graph, dist: &Vec<f32>, path: &Vec<bool>) -> usize {
    let mut min_dist = f32::INFINITY;
    let mut min_idx = g.get_n_nodes(); // default is an invalid node

    for node in g.get_nodes() {
        if dist[node] < min_dist && path[node] == false {
            min_dist = dist[node];
            min_idx = node;
        }
    }

    return min_idx;
}

/// Given a `Graph` and a source node, returns the shortest path to each node
/// from the source provided.
///
/// # Examples
///
/// ```
/// let n_nodes = 3;
/// let edges = vec![(0, 1), (1, 2), (2, 2)];
/// let g = graphst::Graph::from_edges(n_nodes, edges);
/// let short_paths = graphst::algorithm::dijkstra(&g, 0);
/// assert_eq!(short_paths, vec![0.0, 1.0, 2.0]);
/// ```
pub fn dijkstra(g: &Graph, src: usize) -> Vec<f32> {
    let mut dist = vec![f32::INFINITY; g.get_n_nodes()];
    let mut path = vec![false; g.get_n_nodes()];

    dist[src] = g.get_edge(src, src).unwrap_or_else(|| 0.0);

    for _ in g.get_nodes() {
        let c = min_distance_node(g, &dist, &path); // Closest node
        path[c] = true;
        for n in g.get_nodes() {
            if g.get_edge(c, n).is_some()
                && path[n] == false
                && dist[n] > dist[c] + g.get_edge(c, n).unwrap()
            {
                dist[n] = dist[c] + g.get_edge(c, n).unwrap();
            }
        }
    }

    return dist;
}
