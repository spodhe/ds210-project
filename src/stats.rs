//! Module `stats`: weighted least‐squares power‐law exponent estimation.

use std::collections::HashMap;

/// Estimate power‐law exponent α by weighted least‐squares fit
/// of log(count_k) vs log(k), weighting each point by its count.
/// That is, we solve for slope in
///     y = a + b x,   b = –α
/// by minimizing Σ wᵢ (yᵢ – a – b xᵢ)².
///
/// # Inputs
/// - `degree_counts`: map degree k → count of nodes with that degree
/// - `k_min`: inclusive lower bound for degrees to include in the fit
///
/// # Returns
/// - Estimated α̂ (>0), or 0 if insufficient data
pub fn mle_power_law_exponent(
    degree_counts: &HashMap<usize, usize>,
    k_min: usize,
) -> f64 {
    // First pass: accumulate weighted sums for means
    let mut sum_w = 0.0;
    let mut sum_wx = 0.0;
    let mut sum_wy = 0.0;

    for (&k, &cnt) in degree_counts {
        if k >= k_min && cnt > 0 {
            let w = cnt as f64;
            let x = (k as f64).ln();
            let y = (cnt as f64).ln();
            sum_w += w;
            sum_wx += w * x;
            sum_wy += w * y;
        }
    }

    if sum_w == 0.0 {
        return 0.0;
    }

    let mean_x = sum_wx / sum_w;
    let mean_y = sum_wy / sum_w;

    // Second pass: compute weighted covariance and variance
    let mut cov_xy = 0.0;
    let mut var_x = 0.0;
    for (&k, &cnt) in degree_counts {
        if k >= k_min && cnt > 0 {
            let w = cnt as f64;
            let x = (k as f64).ln();
            let y = (cnt as f64).ln();
            let dx = x - mean_x;
            let dy = y - mean_y;
            cov_xy += w * dx * dy;
            var_x += w * dx * dx;
        }
    }

    if var_x == 0.0 {
        return 0.0;
    }

    // In y = a + b x, slope b = cov_xy / var_x, and α̂ = –b
    - (cov_xy / var_x)
}

#[cfg(test)]
mod tests {
    use super::mle_power_law_exponent;
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
}
