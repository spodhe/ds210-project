// src/stats.rs
// Statistical functions for graph metrics including discrete‐power‐law MLE.

use std::collections::HashMap;

/// Estimate discrete power‐law exponent α̂ via Clauset–Shalizi–Newman MLE:
///   α̂ = 1 + n [∑_{i=1..n} ln(k_i / (k_min - 0.5)) ]^{-1}
///
/// # Inputs
/// - `degree_counts`: map degree k → count of nodes with degree k
/// - `k_min`: inclusive lower bound for degrees in the tail
///
/// # Returns
/// - Estimated α̂
///
pub fn mle_power_law_exponent(
    degree_counts: &HashMap<usize, usize>,
    k_min: usize,
) -> f64 {
    let mut n = 0.0;
    let mut log_sum = 0.0;

    for (&k, &count) in degree_counts {
        if k >= k_min && count > 0 {
            let kf = k as f64;
            n += count as f64;
            // discrete correction of 0.5
            log_sum += (kf / ((k_min as f64) - 0.5)).ln() * (count as f64);
        }
    }

    // According to Clauset et al. (2009) MLE
    1.0 + n / log_sum
}
