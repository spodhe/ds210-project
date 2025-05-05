//! Tests for the discrete power-law MLE estimator in `stats`.

use ds210_project::stats::mle_power_law_exponent;
use std::collections::HashMap;

#[test]
/// Synthetic degree counts ∝ k^(−3), k=1..10 → expect estimated exponent ≈ 3 within 1%.
fn mle_power_law_exponent_exact() {
    let alpha_true = 3.0;
    let k_min = 1;
    let mut counts = HashMap::new();
    // generate counts = round(1000 * k^(-3))
    for k in 1..=10 {
        let weight = (k as f64).powf(-alpha_true);
        counts.insert(k, (weight * 1000.0).round() as usize);
    }

    let alpha_hat = mle_power_law_exponent(&counts, k_min);
    let rel_err = (alpha_hat - alpha_true).abs() / alpha_true;
    assert!(
        rel_err < 0.01,
        "α̂ = {:.3}, true = {:.3}, rel_err = {:.3}",
        alpha_hat,
        alpha_true,
        rel_err
    );
}
