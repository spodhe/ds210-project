// tests/test_stats.rs
// Unit tests for stats module, verifying MLE power-law estimation.

use ds210_project::stats::mle_power_law_exponent;
use std::collections::HashMap;

#[test]
fn mle_power_law_exponent_known() {
    // Synthetic P(k) ∝ k^-3 for k=1..100, large sample size
    let alpha_true = 3.0;
    let k_min = 1;
    let mut counts = HashMap::new();
    let sample_size = 1_000_000;

    // Compute unnormalized weights
    let mut total_weight = 0.0;
    for k in k_min..=100 {
        let w = (k as f64).powf(-alpha_true);
        total_weight += w;
        counts.insert(k, 0usize);
    }
    // Assign counts proportional to weights
    for k in k_min..=100 {
        let w = (k as f64).powf(-alpha_true);
        let pk = w / total_weight;
        counts.insert(k, (pk * sample_size as f64) as usize);
    }

    let estimated = mle_power_law_exponent(&counts, k_min);
    // Expect within 1% of true
    let err = (estimated - alpha_true).abs() / alpha_true;
    assert!(
        err < 0.01,
        "Estimated α = {}, true α = {}, err = {:.2}%",
        estimated,
        alpha_true,
        err * 100.0
    );
}
