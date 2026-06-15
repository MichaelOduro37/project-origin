// ============================================================================
// PHASE 38: TRANSFORMATION OPTICS ROUTING (METAMATERIAL CLOAKING)
// ============================================================================
// Scientific mechanism: Transformation Optics & Fermat's Principle
//
// Instead of graph theory, Origin routes data like optical wavefronts through a
// physical medium. Nodes advertise a "refractive index" (n). Under heavy load
// or DDoS attacks, a node dynamically lowers n (creating a metamaterial cloak).
// Since light follows Fermat's Principle of Least Time, the "optical path length"
// increases dramatically, causing network traffic to naturally curve and bend
// around the stressed node, making it topologically invisible to the attack.
// ============================================================================

pub struct OpticsEngine {
    pub base_refractive_index: f64,
}

impl OpticsEngine {
    pub fn new() -> Self {
        Self {
            base_refractive_index: 1.0, // Normal space (vacuum/air equivalent)
        }
    }

    /// Compute the dynamic refractive index based on CPU load and network congestion.
    /// Under severe attack, n drops toward 0.01 (Metamaterial Cloak).
    pub fn compute_refractive_index(&self, cpu_load: f64, network_queue_size: usize) -> f64 {
        // Assume maximum queue size is ~10,000 packets before failure
        let queue_stress = (network_queue_size as f64) / 10_000.0;
        let load_stress = cpu_load / 100.0;
        
        let total_stress = (queue_stress + load_stress).min(1.0);
        
        // If stress is low, n is near 1.0.
        // If stress is critical, n drops precipitously to bend traffic away.
        // We use an exponential decay for the cloak activation.
        if total_stress > 0.85 {
            // Activate Invisibility Cloak
            (1.0 - total_stress).max(0.01)
        } else {
            1.0 - (total_stress * 0.2) // Slight bending for normal congestion
        }
    }

    /// Calculate the Optical Path Length (OPL) to a neighbor.
    /// OPL = physical_distance * (1 / refractive_index)
    /// Note: In standard optics, OPL = distance * n.
    /// However, in routing, lower n = slower speed/higher "cost" 
    /// (to mimic optical density repelling traffic), so we invert it for cost calculation.
    /// Alternatively, we treat n as the "speed of light in medium".
    /// By Fermat's Principle, time t = distance / v. If v is proportional to n, 
    /// then time = distance / n.
    pub fn calculate_optical_path_length(physical_distance: f64, neighbor_refractive_index: f64) -> f64 {
        let n = neighbor_refractive_index.max(0.001); // Prevent division by zero
        physical_distance / n
    }

    /// Determines the optimal neighbor to route to based on Fermat's Principle
    /// Returns the index of the optimal neighbor.
    pub fn bend_traffic_around_stress(
        distances: &[f64],
        refractive_indices: &[f64]
    ) -> Option<usize> {
        if distances.is_empty() || distances.len() != refractive_indices.len() {
            return None;
        }

        let mut best_index = 0;
        let mut min_time = f64::MAX;

        for i in 0..distances.len() {
            let time = Self::calculate_optical_path_length(distances[i], refractive_indices[i]);
            if time < min_time {
                min_time = time;
                best_index = i;
            }
        }

        Some(best_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metamaterial_cloaking_activation() {
        let engine = OpticsEngine::new();
        
        // Normal state
        let n_normal = engine.compute_refractive_index(10.0, 50);
        assert!(n_normal > 0.9);
        
        // Critical DDoS State
        let n_attack = engine.compute_refractive_index(99.0, 9500);
        assert!(n_attack < 0.05, "Refractive index should plummet to create cloak");
    }

    #[test]
    fn test_fermat_routing() {
        // Node A is physically closer but under DDoS (cloaked)
        // Node B is physically further but healthy
        let distances = vec![10.0, 25.0];
        let refractive_indices = vec![0.02, 1.0]; // Node 0 is cloaked
        
        // Route should bend to Node 1 (index 1) despite longer physical distance
        let best_route = OpticsEngine::bend_traffic_around_stress(&distances, &refractive_indices);
        
        assert_eq!(best_route, Some(1));
    }
}
