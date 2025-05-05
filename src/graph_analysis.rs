//! Graph algorithms and metrics: shortest paths, degree distributions,
//! densest-subgraph, and centralities (closeness & betweenness).

use petgraph::{Graph, Undirected, prelude::NodeIndex};
use std::collections::{HashSet, VecDeque, HashMap};

/// Average shortest-path length (unweighted) by sampling up to `sample_size` seeds.
/// 
/// # Inputs
/// - `graph`: undirected graph with `usize` node payloads  
/// 
/// # Output
/// - average geodesic distance over all reachable pairs from sampled seeds  
/// 
/// # Logic
/// For each of up to 5 start nodes: BFS to compute distances in O(V+E), 
/// accumulate all nonzero finite distances.
pub fn average_shortest_path(graph: &Graph<usize, (), Undirected>) -> f64 {
    let node_count = graph.node_count();
    let sample_size = 5.min(node_count);

    let mut total_dist = 0.0;
    let mut total_pairs = 0.0;

    for start in graph.node_indices().take(sample_size) {
        let mut dist = vec![usize::MAX; node_count];
        let mut queue = VecDeque::new();
        dist[start.index()] = 0;
        queue.push_back(start);

        // BFS from `start`
        while let Some(u) = queue.pop_front() {
            let du = dist[u.index()];
            for v in graph.neighbors(u) {
                let vi = v.index();
                if dist[vi] == usize::MAX {
                    dist[vi] = du + 1;
                    queue.push_back(v);
                }
            }
        }

        // Sum all nonzero, finite distances
        for &d in &dist {
            if d > 0 && d < usize::MAX {
                total_dist += d as f64;
                total_pairs += 1.0;
            }
        }
    }

    total_dist / total_pairs
}

/// Result of the densest-subgraph peeling algorithm.
pub struct SubgraphResult {
    /// Node payloads included in the best subgraph
    pub nodes: Vec<usize>,
    /// maximum observed density = |E_sub|/|V_sub|
    pub density: f64,
}

/// 2-approximation for max-density subgraph via peeling.
/// 
/// # Logic
/// Repeatedly compute subgraph density, remove the lowest-degree node,
/// track the iteration with highest density.
pub fn densest_subgraph_peel(graph: &Graph<usize, (), Undirected>) -> SubgraphResult {
    let mut remaining: HashSet<NodeIndex> = graph.node_indices().collect();
    let mut best_density = 0.0;
    let mut best_nodes = Vec::new();

    while !remaining.is_empty() {
        let n = remaining.len() as f64;
        let mut edge_count = 0;
        // count edges internal to `remaining`
        for &u in &remaining {
            for v in graph.neighbors(u) {
                if remaining.contains(&v) {
                    edge_count += 1;
                }
            }
        }
        edge_count /= 2; // undirected double-count
        let density = edge_count as f64 / n;

        if density > best_density {
            best_density = density;
            best_nodes = remaining.iter().map(|&idx| graph[idx]).collect();
        }

        // peel off the minimum-degree node
        if let Some((min_u, _)) = remaining
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
            remaining.remove(&min_u);
        } else {
            break;
        }
    }

    SubgraphResult {
        nodes: best_nodes,
        density: best_density,
    }
}

/// 1-hop degree distribution: degree → count of nodes.
pub fn degree_distribution(
    graph: &Graph<usize, (), Undirected>,
) -> HashMap<usize, usize> {
    let mut dist = HashMap::new();
    for u in graph.node_indices() {
        let deg = graph.neighbors(u).count();
        *dist.entry(deg).or_insert(0) += 1;
    }
    dist
}

/// 2-hop neighbor histogram: for each node, count how many nodes are exactly 2 hops away.
pub fn two_hop_distribution(
    graph: &Graph<usize, (), Undirected>,
) -> HashMap<usize, usize> {
    let node_count = graph.node_count();
    let mut dist = HashMap::new();

    for start in graph.node_indices() {
        let mut distances = vec![usize::MAX; node_count];
        let mut queue = VecDeque::new();
        distances[start.index()] = 0;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            let du = distances[u.index()];
            for v in graph.neighbors(u) {
                let vi = v.index();
                if distances[vi] == usize::MAX {
                    distances[vi] = du + 1;
                    queue.push_back(v);
                }
            }
        }

        let two_hop = distances.iter().filter(|&&d| d == 2).count();
        *dist.entry(two_hop).or_insert(0) += 1;
    }

    dist
}

/// Compute closeness centrality: C(u) = (N-1) / Σ_v d(u,v).
/// Returns map from node payload → centrality score.
pub fn closeness_centrality(
    graph: &Graph<usize, (), Undirected>,
) -> HashMap<usize, f64> {
    let n = graph.node_count() as f64;
    let mut closeness = HashMap::new();

    for u in graph.node_indices() {
        let mut dist = vec![usize::MAX; graph.node_count()];
        let mut queue = VecDeque::new();
        dist[u.index()] = 0;
        queue.push_back(u);

        while let Some(v) = queue.pop_front() {
            let dv = dist[v.index()];
            for w in graph.neighbors(v) {
                let wi = w.index();
                if dist[wi] == usize::MAX {
                    dist[wi] = dv + 1;
                    queue.push_back(w);
                }
            }
        }

        let sum_d: usize = dist.iter().filter(|&&d| d > 0 && d < usize::MAX).sum();
        let c = if sum_d > 0 { (n - 1.0) / (sum_d as f64) } else { 0.0 };
        closeness.insert(graph[u], c);
    }

    closeness
}

/// Compute betweenness centrality (Brandes’ algorithm).
/// Returns map from node payload → betweenness score.
pub fn betweenness_centrality(
    graph: &Graph<usize, (), Undirected>,
) -> HashMap<usize, f64> {
    let mut cb: HashMap<NodeIndex, f64> =
        graph.node_indices().map(|u| (u, 0.0)).collect();

    for s in graph.node_indices() {
        let mut stack = Vec::new();
        let mut pred: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();
        let mut sigma: HashMap<NodeIndex, f64> = HashMap::new();
        let mut dist: HashMap<NodeIndex, i32> = HashMap::new();
        let mut queue = VecDeque::new();

        // initialize
        for v in graph.node_indices() {
            pred.insert(v, Vec::new());
            sigma.insert(v, 0.0);
            dist.insert(v, -1);
        }
        sigma.insert(s, 1.0);
        dist.insert(s, 0);
        queue.push_back(s);

        // BFS phase
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            let dv = dist[&v];
            for w in graph.neighbors(v) {
                if dist[&w] < 0 {
                    dist.insert(w, dv + 1);
                    queue.push_back(w);
                }
                if dist[&w] == dv + 1 {
                    let sw = sigma[&w];
                    sigma.insert(w, sw + sigma[&v]);
                    pred.get_mut(&w).unwrap().push(v);
                }
            }
        }

        // accumulation phase
        let mut delta: HashMap<NodeIndex, f64> =
            graph.node_indices().map(|v| (v, 0.0)).collect();
        while let Some(w) = stack.pop() {
            let dw = delta[&w];
            for &v in &pred[&w] {
                let c = (sigma[&v] / sigma[&w]) * (1.0 + dw);
                *delta.get_mut(&v).unwrap() += c;
            }
            if w != s {
                *cb.get_mut(&w).unwrap() += delta[&w];
            }
        }
    }

    // map back to payloads
    cb.into_iter()
      .map(|(idx, val)| (graph[idx], val))
      .collect()
}


