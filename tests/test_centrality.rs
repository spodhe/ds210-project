//! Tests for closeness and betweenness centrality implementations in `graph_analysis`.

use petgraph::Graph;
use petgraph::Undirected;
use ds210_project::graph_analysis::{closeness_centrality, betweenness_centrality};

/// Build a 3-node triangle: every pair connected.
/// All shortest paths are direct, so betweenness should be zero.
fn triangle_graph() -> Graph<usize, (), Undirected> {
    let mut g = Graph::new_undirected();
    let a = g.add_node(0);
    let b = g.add_node(1);
    let c = g.add_node(2);
    // connect all pairs
    g.add_edge(a, b, ());
    g.add_edge(b, c, ());
    g.add_edge(a, c, ());
    g
}

/// Build a simple path on 4 nodes: 0–1–2–3.
/// Nodes 1 and 2 lie on more shortest paths than the endpoints.
fn path_graph_4() -> Graph<usize, (), Undirected> {
    let mut g = Graph::new_undirected();
    let n0 = g.add_node(0);
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    // chain 0–1–2–3
    g.add_edge(n0, n1, ());
    g.add_edge(n1, n2, ());
    g.add_edge(n2, n3, ());
    g
}

#[test]
/// In a triangle, every node’s closeness = 1 (distance=1 to all others).
fn test_closeness_triangle() {
    let clos = closeness_centrality(&triangle_graph());
    for &v in clos.values() {
        assert!((v - 1.0).abs() < 1e-6, "expected closeness 1.0, got {}", v);
    }
}

#[test]
/// In a triangle, no node lies on any shortest path between others → betweenness = 0.
fn test_betweenness_triangle() {
    let betw = betweenness_centrality(&triangle_graph());
    for &v in betw.values() {
        assert!((v - 0.0).abs() < 1e-6, "expected betweenness 0.0, got {}", v);
    }
}

#[test]
/// On path 0–1–2–3, endpoints have closeness .5 (sum distances=3), middles .75 (sum=2).
fn test_closeness_path4() {
    let clos = closeness_centrality(&path_graph_4());
    assert!((clos[&0] - 0.5).abs() < 1e-6);
    assert!((clos[&1] - 0.75).abs() < 1e-6);
    assert!((clos[&2] - 0.75).abs() < 1e-6);
    assert!((clos[&3] - 0.5).abs() < 1e-6);
}

#[test]
/// On path 0–1–2–3, nodes 1 and 2 each lie on 4 ordered shortest paths; endpoints lie on none.
fn test_betweenness_path4() {
    let betw = betweenness_centrality(&path_graph_4());
    assert!((betw[&1] - 4.0).abs() < 1e-6, "node 1 betweenness; got {}", betw[&1]);
    assert!((betw[&2] - 4.0).abs() < 1e-6, "node 2 betweenness; got {}", betw[&2]);
    assert!((betw[&0] - 0.0).abs() < 1e-6);
    assert!((betw[&3] - 0.0).abs() < 1e-6);
}
