A. Project Overview
Goal:
Answer the question: How cohesive, centralized, and heavy‐tailed is a real‐world friendship network?
I measured

Average shortest‐path length (“six degrees”)

Densest subgraph (2-approximation)

Degree distributions (1-hop & 2-hop) and fit a discrete power-law exponent

Centrality measures (closeness & betweenness)

Dataset:

Facebook “combined” friendship network from the SNAP repository (downloaded May 2025)

Public, CC-BY licensed at Stanford SNAP: https://snap.stanford.edu/data/ego-Facebook.html

Size: 4 039 nodes, 88 234 undirected edges, gzipped whitespace-delimited edge list (data/facebook_combined.txt.gz)

B. Data Processing
Loading

Used Rust’s flate2::GzDecoder + csv::ReaderBuilder (space delimiter, no headers).

Mapped each original usize user ID → a unique petgraph::NodeIndex; stored the same ID as the node payload.

Cleaning/Transformations

None beyond mapping IDs.

Duplicate edges produce multiple add_edge calls, but since our algorithms are unweighted, repeated edges do not alter BFS distances or density counts.

C. Code Structure
css
Copy
Edit
src/
├── main.rs
├── io.rs
├── graph_analysis.rs
├── stats.rs
└── utils.rs
tests/
├── test_centrality.rs
├── test_distribution.rs
└── test_stats.rs
io.rs
Purpose: load a gzipped edge list into a petgraph::Graph.
Key Function:

rust
Copy
Edit
load_facebook_graph(path: &str) -> Result<Graph<usize, (), Undirected>, Box<dyn Error>>
Inputs: file path

Outputs: undirected graph with node payloads = original IDs

Logic: decompress, read each line “u v,” map IDs → indices, insert edge.

graph_analysis.rs
Purpose: implement graph metrics.

average_shortest_path: BFS from up to 5 seeds → mean geodesic distance.

densest_subgraph_peel: 2-approx peeling algorithm → track max |E_sub|/|V_sub|.

degree_distribution: 1-hop degree histogram.

two_hop_distribution: per-node count at distance=2 → histogram.

closeness_centrality: for each node, BFS to sum distances, compute (N−1)/Σd.

betweenness_centrality: Brandes’ algorithm (stack + predecessor lists + accumulation).

stats.rs
Purpose: statistical fitting of degree histogram.

rust
Copy
Edit
mle_power_law_exponent(degree_counts: &HashMap<usize,usize>, k_min: usize) -> f64
Inputs: map degree→count, lower bound k_min

Outputs: estimated exponent α̂

Logic: discrete MLE from Clauset–Shalizi–Newman; corrects for integer bias by using ln(k/(k_min−0.5)).

utils.rs
Purpose: logging and timing helpers.

measure_time: wrap a closure, print elapsed seconds.

print_section: banner formatting.

main.rs
Purpose: orchestrate the end-to-end workflow.

Load graph

Compute each metric, print with timing

Fit and print power-law exponent

Compute and display top-10 closeness & betweenness nodes

D. Tests
bash
Copy
Edit
$ cargo test -- --nocapture
running 1 test
test stats::tests::mle_power_law_exponent_exact … ok
running 4 tests in test_centrality.rs … ok
running 2 tests in test_distribution.rs … ok
test_centrality.rs: verifies closeness & betweenness on triangle & 4-node path.

test_distribution.rs: checks degree and two-hop histograms on a 3-node path.

test_stats.rs: synthetic counts ∝ k^(−3), asserts that α̂ is within 1%.

E. Results
(Excerpt from analysis_output.txt in release mode)

diff
Copy
Edit
=== Loading Graph ===
Graph loaded: 4039 nodes, 88234 edges

=== Average Shortest-Path ===
… length ≈ 3.627

=== Densest Subgraph (2-approx) ===
… density = 77.347, 202 nodes

=== 1-Hop Degrees & Power-Law Fit ===
… α ≈ 0.766

=== 2-Hop Neighbor Distribution ===
… (histogram)

=== Closeness Centrality (top 10) ===
  node 107 → 0.4597  
  …

=== Betweenness Centrality (top 10) ===
  node 107 → 7 833 120.29  
  …
Interpretation:

Mean distance ≈ 3.6 → strong small-world effect.

A 202-node core at density ≈ 77 edges/node → very tight community.

Fitted α≈0.77 is much lower than the canonical 2.5 for social networks, suggesting a heavier tail (more extremely high-degree hubs) than a pure power law would predict.

Top centrality nodes identify both the graph’s “center” (closeness) and key bottlenecks (betweenness).

F. Usage Instructions
bash
Copy
Edit
1. Clone & enter
cd ds210-project
 2. Run tests
cargo test
3. Full analysis (release)
cargo run --release > analysis_output.txt 2>&1
less analysis_output.txt
No CLI arguments needed.
Runtimes (4 039 nodes, 88 234 edges, release):

BFS sampling: ~0.005 s

Peeling: ~2 s

Brandes centrality: ~5 s

G. AI-Assistance & Citations
Outlined and formatted this write-up with ChatGPT; all code and comments were authored manually.

Power-law MLE reference: Clauset, Shalizi & Newman (2009).

Data: SNAP “ego-Facebook.”


Thought for a second


H. Answering the Research Question

How cohesive is the network?
The average shortest-path length of approximately 3.63 (well below “six”) confirms a small-world structure: most nodes can reach each other in four steps or fewer, demonstrating high overall cohesion.

How heavy-tailed is the degree distribution?
A fitted power-law exponent of about 0.77 is much lower than the typical range of 2–3 for social networks, indicating an extremely heavy tail. In practice, this means the Facebook graph contains far more super-hubs (very high-degree nodes) than a standard power law would predict.

How centralized is the network?

* The densest subgraph of size 202 at density around 77 edges per node highlights a core of very tightly interconnected users, pointing to a centralized community.
* Top closeness centrality nodes (for example node 107) minimize average distance to all others, identifying those at the network’s “center of gravity.”
* Top betweenness centrality nodes (again node 107 among them) lie on the most shortest paths, acting as critical bridges. The raw betweenness values in the millions underscore how a few nodes dominate information-flow routes.

In summary, this real-world friendship network is highly cohesive (short global distances), extremely heavy-tailed (many super-connected hubs), and markedly centralized (a small core and a handful of bridge-nodes dominate connectivity).

