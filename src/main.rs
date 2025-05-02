mod io;
mod graph_analysis;
mod utils;

use crate::utils::{print_section, measure_time};
use itertools::Itertools; // for .sorted_by_key()

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Load
    print_section("Loading Graph");
    let graph = io::load_facebook_graph("data/facebook_combined.txt.gz")?;
    println!("Graph loaded: {} nodes, {} edges\n", graph.node_count(), graph.edge_count());

    // 2) Average shortest-path
    print_section("Average Shortest-Path");
    let avg = measure_time("BFS sampling", || {
        graph_analysis::average_shortest_path(&graph)
    });
    println!("Average shortest-path length ≈ {:.3}\n", avg);

    // 3) Densest subgraph
    print_section("Densest Subgraph (2-approx)");
    let ds = measure_time("Peeling algorithm", || {
        graph_analysis::densest_subgraph_peel(&graph)
    });
    println!("Density = {:.3} with {} nodes\n", ds.density, ds.nodes.len());

    // 4) 1-hop degree distribution
    print_section("1-Hop Degree Distribution");
    let one_hop = graph_analysis::degree_distribution(&graph);
    for (deg, cnt) in one_hop.iter().sorted_by_key(|&(deg, _)| *deg) {
        println!("  degree {:>3} → {:>5} nodes", deg, cnt);
    }
    println!();

    // 5) 2-hop neighbor distribution
    print_section("2-Hop Neighbor Distribution");
    let two_hop = graph_analysis::two_hop_distribution(&graph);
    for (h2, cnt) in two_hop.iter().sorted_by_key(|&(h2, _)| *h2) {
        println!("  {:>3} two-hop neighbors → {:>5} nodes", h2, cnt);
    }
    println!();

    Ok(())
}

