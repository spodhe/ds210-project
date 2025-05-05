// tests/test_distribution.rs
// Smoke tests for 1-hop and 2-hop distributions.

use ds210_project::graph_analysis;
use petgraph::{Graph, Undirected};

#[test]
fn degree_distribution_path_graph() {
    // Path 1—2—3 has degrees [1,2,1]
    let mut g = Graph::<usize, (), Undirected>::new_undirected();
    let n1 = g.add_node(0);
    let n2 = g.add_node(1);
    let n3 = g.add_node(2);
    g.add_edge(n1, n2, ());
    g.add_edge(n2, n3, ());

    let dist = graph_analysis::degree_distribution(&g);
    assert_eq!(dist.get(&1).cloned().unwrap_or(0), 2);
    assert_eq!(dist.get(&2).cloned().unwrap_or(0), 1);
}

#[test]
fn two_hop_distribution_path_graph() {
    // In the same path, nodes 1 & 3 each see each other at distance=2; node2 sees none.
    let mut g = Graph::<usize, (), Undirected>::new_undirected();
    let n1 = g.add_node(0);
    let n2 = g.add_node(1);
    let n3 = g.add_node(2);
    g.add_edge(n1, n2, ());
    g.add_edge(n2, n3, ());

    let dist2 = graph_analysis::two_hop_distribution(&g);
    assert_eq!(dist2.get(&1).cloned().unwrap_or(0), 2);
    assert_eq!(dist2.get(&0).cloned().unwrap_or(0), 1);
}
