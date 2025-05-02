use petgraph::{Graph, Undirected, prelude::NodeIndex};
use std::collections::{HashSet, VecDeque, HashMap};

/// Average shortest‐path length (unweighted) by sampling a few seed nodes.
pub fn average_shortest_path(graph: &Graph<usize, (), Undirected>) -> f64 {
    let node_count = graph.node_count();
    let sample_size = 5.min(node_count);

    let mut total_dist = 0.0;
    let mut total_pairs = 0.0;

    for start in graph.node_indices().take(sample_size) {
        let mut dist = vec![usize::MAX; node_count];
        let mut queue = VecDeque::new();
        let si = start.index();
        dist[si] = 0;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            let ui = u.index();
            for v in graph.neighbors(u) {
                let vi = v.index();
                if dist[vi] == usize::MAX {
                    dist[vi] = dist[ui] + 1;
                    queue.push_back(v);
                }
            }
        }

        for &d in &dist {
            if d > 0 && d < usize::MAX {
                total_dist += d as f64;
                total_pairs += 1.0;
            }
        }
    }

    total_dist / total_pairs
}

/// Result of the densest‐subgraph peeling algorithm.
pub struct SubgraphResult {
    pub nodes: Vec<usize>,
    pub density: f64,
}

/// 2‐approximation for the densest subgraph via the “peeling” algorithm.
pub fn densest_subgraph_peel(graph: &Graph<usize, (), Undirected>) -> SubgraphResult {
    let mut remaining: HashSet<NodeIndex> = graph.node_indices().collect();
    let mut best_density = 0.0;
    let mut best_nodes = Vec::new();

    while !remaining.is_empty() {
        let n = remaining.len() as f64;
        let mut edge_count = 0;
        for &u in &remaining {
            for v in graph.neighbors(u) {
                if remaining.contains(&v) {
                    edge_count += 1;
                }
            }
        }
        edge_count /= 2; // undirected counted twice
        let density = edge_count as f64 / n;

        if density > best_density {
            best_density = density;
            best_nodes = remaining.iter().map(|&idx| graph[idx]).collect();
        }

        // remove the node with minimum degree in the subgraph
        if let Some((min_node, _)) = remaining
            .iter()
            .map(|&u| {
                let deg = graph
                    .neighbors(u)
                    .filter(|v| remaining.contains(v))
                    .count();
                (u, deg)
            })
            .min_by_key(|&(_, deg)| deg)
        {
            remaining.remove(&min_node);
        } else {
            break;
        }
    }

    SubgraphResult {
        nodes: best_nodes,
        density: best_density,
    }
}

/// Compute the 1-hop degree distribution: degree → number of nodes with that degree.
pub fn degree_distribution(
    graph: &Graph<usize, (), Undirected>
) -> HashMap<usize, usize> {
    let mut dist = HashMap::new();
    for node in graph.node_indices() {
        let deg = graph.neighbors(node).count();
        *dist.entry(deg).or_insert(0) += 1;
    }
    dist
}

/// Compute the 2-hop neighbor distribution: “# nodes at distance 2” → count of nodes.
pub fn two_hop_distribution(
    graph: &Graph<usize, (), Undirected>
) -> HashMap<usize, usize> {
    let node_count = graph.node_count();
    let mut dist = HashMap::new();

    for start in graph.node_indices() {
        // BFS to compute distances
        let mut distances = vec![usize::MAX; node_count];
        let mut queue = VecDeque::new();
        let si = start.index();
        distances[si] = 0;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            for v in graph.neighbors(u) {
                let vi = v.index();
                if distances[vi] == usize::MAX {
                    distances[vi] = distances[u.index()] + 1;
                    queue.push_back(v);
                }
            }
        }

        // count exactly distance-2 neighbors
        let two_hop = distances.iter().filter(|&&d| d == 2).count();
        *dist.entry(two_hop).or_insert(0) += 1;
    }

    dist
}
