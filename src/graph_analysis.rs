use petgraph::{Graph, Undirected, prelude::NodeIndex};
use std::collections::{HashSet, VecDeque, HashMap};

/// Average shortest-path length (unweighted) by sampling a few seed nodes.
/// We only sample up to `sample_size` start nodes to keep runtime O(s·(V+E)),
/// where s = sample_size, V = node_count, E = edge_count.
pub fn average_shortest_path(graph: &Graph<usize, (), Undirected>) -> f64 {
    let node_count = graph.node_count();
    // Limit sample size to avoid running BFS from every node in large graphs
    let sample_size = 5.min(node_count);

    let mut total_dist = 0.0;
    let mut total_pairs = 0.0;

    // For each sampled start node, perform a BFS in O(V+E)
    for start in graph.node_indices().take(sample_size) {
        let mut dist = vec![usize::MAX; node_count];
        let mut queue = VecDeque::new();
        let s_idx = start.index();
        dist[s_idx] = 0;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            let ui = u.index();
            for v in graph.neighbors(u) {
                let vi = v.index();
                // First time we see v
                if dist[vi] == usize::MAX {
                    dist[vi] = dist[ui] + 1;
                    queue.push_back(v);
                }
            }
        }

        // Accumulate all finite distances (skip self at distance 0)
        for &d in &dist {
            if d > 0 && d < usize::MAX {
                total_dist += d as f64;
                total_pairs += 1.0;
            }
        }
    }

    // Divide total length by number of pairs to get average
    total_dist / total_pairs
}

/// The result of the densest-subgraph peeling algorithm.
pub struct SubgraphResult {
    /// The actual node payloads included in the densest subgraph found.
    pub nodes: Vec<usize>,
    /// Maximum density observed = |E_sub| / |V_sub|.
    pub density: f64,
}

/// 2-approximation for the densest subgraph via the “peeling” algorithm.
/// Each iteration removes the lowest-degree node in O(V·d) time; overall ~O(V²).
pub fn densest_subgraph_peel(graph: &Graph<usize, (), Undirected>) -> SubgraphResult {
    // Track which nodes remain in the current subgraph
    let mut remaining: HashSet<NodeIndex> = graph.node_indices().collect();
    let mut best_density = 0.0;
    let mut best_nodes = Vec::new();

    // Repeat until no nodes remain
    while !remaining.is_empty() {
        let n = remaining.len() as f64;

        // Count edges within 'remaining': O(E_sub)
        let mut edge_count = 0;
        for &u in &remaining {
            for v in graph.neighbors(u) {
                if remaining.contains(&v) {
                    edge_count += 1;
                }
            }
        }
        edge_count /= 2; // each undirected edge counted twice
        let density = edge_count as f64 / n;

        // Update best if this density is higher
        if density > best_density {
            best_density = density;
            best_nodes = remaining.iter().map(|&idx| graph[idx]).collect();
        }

        // Find the node with minimum degree in current subgraph – O(V_sub·d)
        if let Some((min_node, _min_deg)) = remaining
            .iter()
            .map(|&u| {
                // Compute degree restricted to remaining nodes
                let deg = graph
                    .neighbors(u)
                    .filter(|v| remaining.contains(v))
                    .count();
                (u, deg)
            })
            .min_by_key(|&(_, deg)| deg)
        {
            // Remove that node and repeat
            remaining.remove(&min_node);
        } else {
            // Shouldn’t happen, but break to avoid infinite loop
            break;
        }
    }

    SubgraphResult {
        nodes: best_nodes,
        density: best_density,
    }
}

/// Compute the 1-hop degree distribution: how many nodes have each degree.
/// Runs in O(V + E) by visiting each node’s adjacency once.
pub fn degree_distribution(
    graph: &Graph<usize, (), Undirected>
) -> HashMap<usize, usize> {
    let mut dist = HashMap::new();
    for node in graph.node_indices() {
        let deg = graph.neighbors(node).count(); // O(deg(node))
        *dist.entry(deg).or_insert(0) += 1;
    }
    dist
}

/// Compute the 2-hop neighbor count distribution: for each node,
/// count how many nodes are exactly distance=2 away, then histogram that.
/// Each BFS is O(V+E), so overall O(V·(V+E))—avoid on very large graphs.
pub fn two_hop_distribution(
    graph: &Graph<usize, (), Undirected>
) -> HashMap<usize, usize> {
    let node_count = graph.node_count();
    let mut dist = HashMap::new();

    for start in graph.node_indices() {
        // Standard BFS to compute distances
        let mut distances = vec![usize::MAX; node_count];
        let mut queue = VecDeque::new();
        let s_idx = start.index();
        distances[s_idx] = 0;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            let ui = u.index();
            for v in graph.neighbors(u) {
                let vi = v.index();
                if distances[vi] == usize::MAX {
                    distances[vi] = distances[ui] + 1;
                    queue.push_back(v);
                }
            }
        }

        // Count nodes at exactly two hops away
        let two_hop_count = distances.iter().filter(|&&d| d == 2).count();
        *dist.entry(two_hop_count).or_insert(0) += 1;
    }

    dist
}
