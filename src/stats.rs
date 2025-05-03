// src/stats.rs
// Statistical functions for graph metrics such as power-law fitting using MLE.

use std::collections::HashMap;

/// Estimate the power-law exponent α via MLE for discrete data k ≥ k_min.
/// 
/// Formula (Clauset–Shalizi–Newman):  
///   α̂ = 1 + n [∑_{i=1..n} ln(k_i / (k_min - 0.5)) ]^{-1}
///
/// # Inputs
/// - `degree_counts`: map degree k → number of nodes with degree k
/// - `k_min`: the minimum degree to include in the tail fit (commonly 1)
///
/// # Outputs
/// - Estimated exponent α̂ (f64)
pub fn mle_power_law_exponent(
    degree_counts: &HashMap<usize, usize>,
    k_min: usize,
) -> f64 {
    // Collect all k_i values (with multiplicity)
    let mut log_sum = 0.0;
    let mut n = 0.0;
    for (&k, &count) in degree_counts.iter() {
        if k >= k_min && count > 0 {
            let kf = k as f64;
            // each occurrence contributes to the sum
            log_sum += (kf / ((k_min as f64) - 0.5)).ln() * (count as f64);
            n += count as f64;
        }
    }
    // MLE estimate
    1.0 + n / log_sum
}

