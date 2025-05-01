use std::{collections::HashMap, error::Error, fs::File};
use flate2::read::GzDecoder;
use csv::ReaderBuilder;
use petgraph::{Graph, Undirected};

/// Load the Facebook edge list from a gzipped file into an undirected Graph.
/// Each node ID in the file is mapped to a Graph node index.
pub fn load_facebook_graph(
    path: &str
) -> Result<Graph<usize, (), Undirected>, Box<dyn Error>> {
    // Open and decompress the file
    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);

    // Configure CSV reader for whitespace-delimited, no headers
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(decoder);

    // Create an undirected graph and a map from node ID to graph index
    let mut graph = Graph::new_undirected();
    let mut node_map: HashMap<usize, _> = HashMap::new();

    // Read each record, parse node IDs, add nodes/edges
    for result in reader.records() {
        let record = result?;
        let u: usize = record[0].parse()?;
        let v: usize = record[1].parse()?;

        // Insert node u if new, else get its index
        let i = *node_map
            .entry(u)
            .or_insert_with(|| graph.add_node(u));

        // Insert node v if new, else get its index
        let j = *node_map
            .entry(v)
            .or_insert_with(|| graph.add_node(v));

        // Add an undirected edge between i and j
        graph.add_edge(i, j, ());
    }

    Ok(graph)
}
