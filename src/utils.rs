// src/utils.rs
// Utility functions for timing, logging, and formatting throughout the DS210 project.

use std::time::Instant;

/// Measure execution time of a closure `f`, printing the elapsed duration with `label`.
///
/// # Inputs
/// - `label`: a descriptive name for the timed block  
/// - `f`: a zero-argument closure whose execution you want to measure  
///
/// # Outputs
/// - Returns whatever `f()` returns  
///
/// # Examples
/// ```rust,no_run
/// use ds210_project::graph_analysis;
/// # let graph = graph_analysis::load_facebook_graph("data/facebook_combined.txt.gz").unwrap();
/// let result = ds210_project::utils::measure_time("Compute Avg Path", || {
///     graph_analysis::average_shortest_path(&graph)
/// });
/// ```
pub fn measure_time<F, R>(label: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    println!(
        "[{}] completed in {}.{:03} secs",
        label,
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
    result
}

/// Print a formatted section header to console for better log readability.
///
/// # Inputs
/// - `title`: the section title to display  
///
/// # Examples
/// ```rust,no_run
/// use ds210_project::utils::print_section;
/// print_section("Loading Graph");
/// ```
pub fn print_section(title: &str) {
    println!("\n=== {} ===", title);
}

/// Format a fraction `x` into a percent string with two decimal places.
///
/// # Inputs
/// - `x`: value between 0.0 and 1.0 representing the fraction  
///
/// # Outputs
/// - A `String`, e.g. `"42.00%"`  
///
/// # Examples
/// ```rust
/// use ds210_project::utils::format_pct;
/// let pct = format_pct(0.4235);
/// assert_eq!(pct, "42.35%");
/// ```
pub fn format_pct(x: f64) -> String {
    format!("{:.2}%", x * 100.0)
}
