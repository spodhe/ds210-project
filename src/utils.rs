//! Module `utils`: timing and logging helpers.

use std::time::Instant;

/// Measure execution time of closure `f`, print elapsed with `label`,
/// and return `f`’s result.
///
/// # Inputs
/// - `label`: description of timed block
/// - `f`: zero-arg closure
///
/// # Outputs
/// - returns whatever `f()` returns
pub fn measure_time<F, R>(label: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    println!("[{}] completed in {}.{:03} secs",
             label, elapsed.as_secs(), elapsed.subsec_millis());
    result
}

/// Print a formatted section header.
///
/// # Inputs
/// - `title`: text to display
pub fn print_section(title: &str) {
    println!("\n=== {} ===", title);
}
