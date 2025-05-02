// utils.rs
// Utility functions for timing, logging, and formatting throughout the DS210 project.

use std::time::{Duration, Instant};

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
/// ```
/// let result = measure_time("Compute Avg Path", || {
///     graph_analysis::average_shortest_path(&graph)
/// });
/// ```
pub fn measure_time<F, R>(label: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();            // Record start time
    let result = f();                      // Run the user closure
    let elapsed = start.elapsed();         // Compute elapsed duration

    // Print in seconds.milliseconds format
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
/// # Example
/// ```
/// print_section("Loading Graph");
/// ```
pub fn print_section(title: &str) {
    // Surround title with separators
    println!("\n=== {} ===", title);
}

/// Format a fraction `x` into a percent string with two decimal places.
///
/// # Inputs
/// - `x`: value between 0.0 and 1.0 representing the fraction  
///
/// # Outputs
/// - A `String`, e.g. `\"42.00%\"`  
///
/// # Example
/// ```
/// let pct = format_pct(0.4235); // \"42.35%\"
/// ```
pub fn format_pct(x: f64) -> String {
    format!("{:.2}%", x * 100.0)            // Multiply by 100 and format
}
