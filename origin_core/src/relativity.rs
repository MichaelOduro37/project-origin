// ============================================================================
// PHASE 46: RELATIVISTIC TIME DILATION (LORENTZ CONSENSUS)
// ============================================================================
// Scientific mechanism: Einstein's Theory of Special Relativity
//
// Traditional networks use rigid timeouts (e.g., "if node doesn't reply in 
// 5000ms, disconnect it"). This causes cascading failures under heavy load.
// Origin Abandons Absolute Time.
// We measure a node's data throughput as "velocity" (v).
// We define max theoretical bandwidth as the "speed of light" (c).
// We calculate the Lorentz Factor (gamma).
// If a node is highly congested (v -> c), its perception of Network Time
// dilates. The global network bends time around it, dynamically extending
// its timeout window so it doesn't fail.
// ============================================================================

/// Calculates the Lorentz Factor (gamma) based on Special Relativity.
/// gamma = 1 / sqrt(1 - (v^2 / c^2))
/// 
/// `velocity`: The node's current data processing rate (e.g., MB/s).
/// `speed_of_light`: The maximum theoretical bandwidth limit of the node (e.g., MB/s).
pub fn calculate_lorentz_factor(velocity: f64, speed_of_light: f64) -> f64 {
    // Prevent division by zero or NaN errors if velocity mathematically exceeds bounds.
    // In physics, v cannot exceed c. In our simulation, we clamp it slightly below c
    // to prevent an infinite Lorentz factor (which would mean time stops completely).
    let mut safe_v = velocity;
    if safe_v >= speed_of_light {
        safe_v = speed_of_light * 0.9999;
    }

    let v_squared = safe_v * safe_v;
    let c_squared = speed_of_light * speed_of_light;
    
    let ratio = v_squared / c_squared;
    let denominator = (1.0 - ratio).sqrt();
    
    1.0 / denominator
}

/// Dynamically extends the consensus timeout based on the calculated Lorentz Factor.
/// A higher gamma value means time has dilated significantly for the congested node.
pub fn dilate_timeout(base_timeout_ms: u64, lorentz_factor: f64) -> u64 {
    // We multiply the base time by the Lorentz factor.
    // E.g., if base is 5000ms, and gamma is 2.5 (heavy load), new timeout is 12,500ms.
    (base_timeout_ms as f64 * lorentz_factor).round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relativistic_time_dilation() {
        let speed_of_light = 1000.0; // Max bandwidth: 1000 MB/s
        let base_timeout = 5000; // 5000 ms

        // Scenario 1: Node at rest (almost no load)
        let v_rest = 10.0; // 10 MB/s
        let gamma_rest = calculate_lorentz_factor(v_rest, speed_of_light);
        let timeout_rest = dilate_timeout(base_timeout, gamma_rest);
        
        // At rest, gamma should be extremely close to 1.0, timeout ~5000ms.
        assert!(gamma_rest > 1.0 && gamma_rest < 1.001);
        assert_eq!(timeout_rest, 5000);

        // Scenario 2: Node under extreme load (near the speed of light)
        let v_heavy = 900.0; // 900 MB/s (90% capacity)
        let gamma_heavy = calculate_lorentz_factor(v_heavy, speed_of_light);
        let timeout_heavy = dilate_timeout(base_timeout, gamma_heavy);
        
        // At 0.9c, gamma should be roughly 2.29. Timeout should extend.
        assert!(gamma_heavy > 2.2 && gamma_heavy < 2.3);
        assert!(timeout_heavy > 11000); // Timeout extended to >11 seconds!

        // Scenario 3: Asymptotic Clamping (v >= c)
        let v_impossible = 1500.0;
        let gamma_max = calculate_lorentz_factor(v_impossible, speed_of_light);
        let timeout_max = dilate_timeout(base_timeout, gamma_max);
        
        // The clamp prevents a crash and caps at the max mathematical dilation
        assert!(gamma_max > 70.0);
        assert!(timeout_max > 350_000); // Time is massively bent to save the node
    }
}
