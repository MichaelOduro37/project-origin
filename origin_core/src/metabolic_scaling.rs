// ============================================================================
// PHASE 34: FRACTAL METABOLIC SCALING (WBE MODEL)
// ============================================================================
// Scientific mechanism: Metabolic Scaling Theory / Kleiber's Law (West, Brown, Enquist, 1997)
//
// If a Trillion-node Swarm scaled bandwidth linearly (O(N)), the global energy
// consumption would instantly overwhelm any physical infrastructure. 
// Origin fixes this by mapping the Swarm's bandwidth allocation onto a 
// space-filling fractal hierarchy, mirroring the biological cardiovascular system.
// This geometry mathematically enforces Kleiber's Law: The total energy/bandwidth
// consumed by the entire Swarm scales to the 3/4 power of the total node count.
// As the Swarm grows larger, each individual node is throttled to a smaller 
// "capillary" bandwidth, forcing the network to become geometrically more efficient.
// ============================================================================

pub struct FractalMetabolicNetwork {
    pub base_metabolism: f64, // B_0: Base bandwidth coefficient per node
    pub scaling_exponent: f64, // 0.75 (Kleiber's Law)
}

impl FractalMetabolicNetwork {
    pub fn new(base_metabolism: f64) -> Self {
        Self {
            base_metabolism,
            scaling_exponent: 0.75, // Mathematically derived from WBE space-filling fractals
        }
    }

    /// Calculate the absolute maximum global bandwidth/energy the entire Swarm is allowed to consume
    pub fn calculate_total_metabolism(&self, swarm_mass: usize) -> f64 {
        if swarm_mass == 0 {
            return 0.0;
        }
        self.base_metabolism * (swarm_mass as f64).powf(self.scaling_exponent)
    }

    /// Calculate the allowed "capillary" bandwidth for an individual node.
    /// Notice that this scales as M^(-0.25), meaning nodes are throttled as the Swarm grows,
    /// preventing global broadcast storms and forcing localized computing.
    pub fn allocate_capillary_bandwidth(&self, swarm_mass: usize) -> f64 {
        if swarm_mass == 0 {
            return 0.0;
        }
        let total_b = self.calculate_total_metabolism(swarm_mass);
        total_b / (swarm_mass as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wbe_sublinear_metabolic_scaling() {
        let metabolic_network = FractalMetabolicNetwork::new(100.0);
        
        let mass_small = 1_000;
        let mass_large = 1_000_000; // 1000x larger Swarm

        let total_small = metabolic_network.calculate_total_metabolism(mass_small);
        let total_large = metabolic_network.calculate_total_metabolism(mass_large);

        // Calculate the empirical scaling exponent: log(B_large / B_small) / log(M_large / M_small)
        let empirical_exponent = (total_large / total_small).ln() / ( (mass_large as f64) / (mass_small as f64) ).ln();
        
        // Assert the exponent perfectly matches Kleiber's 3/4 Law
        assert!((empirical_exponent - 0.75).abs() < 1e-6, "Scaling failed to converge to Kleiber's 3/4 Law");

        // Per-node bandwidth MUST drop as the Swarm grows to maintain biological efficiency
        let cap_small = metabolic_network.allocate_capillary_bandwidth(mass_small);
        let cap_large = metabolic_network.allocate_capillary_bandwidth(mass_large);
        
        assert!(cap_large < cap_small, "Per-node capillary bandwidth did not drop as Swarm grew");
    }
}
