//! Module `io`: load Facebook edge list into a `petgraph::Graph`.

use std::{collections::HashMap, error::Error, fs::File};
use flate2::read::GzDecoder;
use csv::ReaderBuilder;
use petgraph::{Graph, Undirected};

/// Load gzipped space-delimited edge list into an undirected graph.
///
/// # Inputs
/// - `path`: file path to gzipped edge list (u v per line)
///
/// # Outputs
/// - `Ok(Graph<usize, (), Undirected>)`: nodes payload = original ID
/// - `Err(...)` on I/O or parse errors
///
/// # High-level logic
/// 1. Open & decompress via `GzDecoder`.  
/// 2. Use CSV reader with space delimiter, no headers.  
/// 3. Map each original node ID → unique `NodeIndex`.  
/// 4. Insert edge for each `(u,v)`.
pub fn load_facebook_graph(
    path: &str
) -> Result<Graph<usize, (), Undirected>, Box<dyn Error>> {
    let file = File::open(path)?;                 // open compressed file
    let decoder = GzDecoder::new(file);           // wrap in gzip decoder

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(decoder);

    let mut graph = Graph::<usize, (), Undirected>::new_undirected();
    let mut node_map: HashMap<usize, _> = HashMap::new();

    for record in reader.records() {
        let rec = record?;
        let u: usize = rec[0].parse()?;
        let v: usize = rec[1].parse()?;

        // Map or insert u
        let i = *node_map.entry(u)
            .or_insert_with(|| graph.add_node(u));
        // Map or insert v
        let j = *node_map.entry(v)
            .or_insert_with(|| graph.add_node(v));

        graph.add_edge(i, j, ());                // add undirected edge
    }

    Ok(graph)
}
