// tests/test_stats.rs

use ds210_project::stats::mle_power_law_exponent;
use std::collections::HashMap;

#[test]
fn mle_power_law_exponent_exact() {
    // Degree counts exactly proportional to k^-3 for k=1..10
    let alpha_true = 3.0;
    let k_min = 1;
    let mut counts = HashMap::new();
    // Assign count = round(1000 * k^-3)
    for k in 1..=10 {
        let weight = (k as f64).powf(-alpha_true);
        counts.insert(k, (weight * 1000.0).round() as usize);
    }
    let alpha_hat = mle_power_law_exponent(&counts, k_min);
    // Expect within 1%
    assert!(
        (alpha_hat - alpha_true).abs() / alpha_true < 0.01,
        "α̂ = {}, expected ~{}",
        alpha_hat,
        alpha_true
    );
}
