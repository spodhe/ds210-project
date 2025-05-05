//! CLI entrypoint: loads Facebook graph, runs analyses, and prints results.

mod io;
mod graph_analysis;
mod stats;
mod utils;

use crate::utils::{print_section, measure_time};
use itertools::Itertools; // for sorted_by_key

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Load the graph once from gzipped edge list
    print_section("Loading Graph");
    let graph = io::load_facebook_graph("data/facebook_combined.txt.gz")?;
    println!(
        "Graph loaded: {} nodes, {} edges\n",
        graph.node_count(),
        graph.edge_count()
    );

    // -- Average shortest-path (unweighted) via BFS sampling
    print_section("Average Shortest-Path");
    let avg = measure_time("BFS sampling", || {
        graph_analysis::average_shortest_path(&graph)
    });
    println!("Average shortest-path length ≈ {:.3}\n", avg);

    // -- Densest subgraph (2-approx) via peeling algorithm
    print_section("Densest Subgraph (2-approx)");
    let ds = measure_time("Peeling algorithm", || {
        graph_analysis::densest_subgraph_peel(&graph)
    });
    println!("Density = {:.3} with {} nodes\n", ds.density, ds.nodes.len());

    // -- 1-hop degree distribution histogram
    print_section("1-Hop Degree Distribution");
    let one_hop = graph_analysis::degree_distribution(&graph);
    for (deg, cnt) in one_hop.iter().sorted_by_key(|&(d, _)| *d) {
        println!("  degree {:>3} → {:>5} nodes", deg, cnt);
    }
    println!();

    // -- Discrete-MLE power-law fitting on 1-hop degrees
    print_section("Power-Law Fit (1-Hop Degrees)");
    let alpha_hat = stats::mle_power_law_exponent(&one_hop, /*k_min=*/ 1);
    println!("Estimated power-law exponent α ≈ {:.3}\n", alpha_hat);

    // -- 2-hop neighbor count distribution histogram
    print_section("2-Hop Neighbor Distribution");
    let two_hop = graph_analysis::two_hop_distribution(&graph);
    for (h2, cnt) in two_hop.iter().sorted_by_key(|&(h2, _)| *h2) {
        println!("  {:>3} two-hop neighbors → {:>5} nodes", h2, cnt);
    }
    println!();

    // -- Closeness centrality (top 10)
    print_section("Closeness Centrality (top 10)");
    let mut clos_vec: Vec<_> = graph_analysis::closeness_centrality(&graph)
        .into_iter()
        .collect();
    clos_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (node, score) in clos_vec.into_iter().take(10) {
        println!("  node {:>4} → {:.4}", node, score);
    }
    println!();

    // -- Betweenness centrality (top 10)
    print_section("Betweenness Centrality (top 10)");
    let mut betw_vec: Vec<_> = graph_analysis::betweenness_centrality(&graph)
        .into_iter()
        .collect();
    betw_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (node, score) in betw_vec.into_iter().take(10) {
        println!("  node {:>4} → {:.4}", node, score);
    }
    println!();

    Ok(())
}
