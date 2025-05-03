// tests/test_stats.rs
use ds210_project::stats::estimate_power_law_exponent;
use std::collections::HashMap;

#[test]
fn power_law_exponent_known() {
    // Create a synthetic distribution P(k) ~ k^-3 for k = 1..100
    let mut counts = HashMap::new();
    let alpha_true = 3.0;
    let n_points = 100_000;
    // Generate counts proportional to k^-alpha_true
    let mut total = 0.0;
    for k in 1..=100 {
        let weight = (k as f64).powf(-alpha_true);
        total += weight;
        counts.insert(k, 0usize);
    }
    // Normalize and assign integer counts summing to n_points
    for k in 1..=100 {
        let weight = (k as f64).powf(-alpha_true);
        let pk = weight / total;
        counts.insert(k, (pk * n_points as f64) as usize);
    }

    let estimated = estimate_power_law_exponent(&counts);
    // Allow some tolerance due to sampling discretization
    assert!((estimated - alpha_true).abs() < 0.1, "Expected alpha ~3.0, got {}", estimated);
}
