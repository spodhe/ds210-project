mod io;
mod graph_analysis;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Load the graph
    let graph = io::load_facebook_graph("data/facebook_combined.txt.gz")?;
    println!("Graph loaded: {} nodes, {} edges",
             graph.node_count(), graph.edge_count());

    // 2) Compute and print average shortest-path length
    let avg = graph_analysis::average_shortest_path(&graph);
    println!("Average shortest-path length ≈ {:.3}", avg);

    // 3) Compute and print densest subgraph result
    let result = graph_analysis::densest_subgraph_peel(&graph);
    println!(
        "Densest subgraph density = {:.3} with {} nodes",
        result.density,
        result.nodes.len()
    );

    Ok(())
}

