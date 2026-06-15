// ============================================================================
// PHASE 35: NETWORK RESILIENCE (PERCOLATION THEORY)
// ============================================================================
// Scientific mechanism: Percolation Theory (Broadbent & Hammersley, 1957)
//
// In a massive P2P network, targeted attacks or major infrastructure outages
// remove nodes/links. If the density of active links (p) falls below a 
// mathematically precise critical threshold (p_c), the network undergoes a 
// phase transition: the "giant connected component" shatters into isolated islands.
//
// Origin uses Percolation Theory to dynamically calculate its own p_c based on 
// its degree distribution: p_c = <k> / (<k^2> - <k>).
// If the Swarm detects p approaching p_c, it autonomously triggers emergency 
// topological healing, dynamically weaving new connections before fragmentation 
// can physically occur.
// ============================================================================

pub struct PercolationMonitor {
    pub average_degree: f64,        // <k>
    pub average_sq_degree: f64,     // <k^2>
    pub current_density: f64,       // p (Current fraction of active links/nodes, 0.0 to 1.0)
}

pub enum PercolationState {
    Safe,
    Critical(f64), // Contains the critical threshold p_c
    Shattered,
}

impl PercolationMonitor {
    pub fn new(average_degree: f64, average_sq_degree: f64, current_density: f64) -> Self {
        Self {
            average_degree,
            average_sq_degree,
            current_density,
        }
    }

    /// Calculate the Critical Percolation Threshold (p_c) for the network topology.
    /// If p drops below p_c, the global Swarm shatters.
    pub fn calculate_critical_threshold(&self) -> f64 {
        if self.average_sq_degree <= self.average_degree {
            return 1.0; // Network is highly fragile
        }
        self.average_degree / (self.average_sq_degree - self.average_degree)
    }

    /// Check if the network is approaching fragmentation
    pub fn check_percolation_state(&self) -> PercolationState {
        let p_c = self.calculate_critical_threshold();
        
        if self.current_density < p_c {
            PercolationState::Shattered
        } else if self.current_density < p_c * 1.15 {
            // If we are within 15% of shattering, trigger critical state!
            PercolationState::Critical(p_c)
        } else {
            PercolationState::Safe
        }
    }

    /// Emergency protocol to heal the network by dynamically weaving new links.
    /// This effectively increases the average degree, lowering p_c, making the network safe again.
    pub fn trigger_emergency_healing(&mut self) {
        // Weave new connections (Constructal Bridges) to boost degree by 30%
        self.average_degree *= 1.30;
        self.average_sq_degree *= 1.69; // (1.3^2)
        // Assume active density slightly improves as isolated nodes are reconnected
        self.current_density = (self.current_density * 1.1).min(1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percolation_shattering_and_healing() {
        // Initial healthy network: <k> = 4, <k^2> = 20
        // p_c = 4 / (20 - 4) = 4 / 16 = 0.25
        let mut monitor = PercolationMonitor::new(4.0, 20.0, 1.0);
        
        assert!((monitor.calculate_critical_threshold() - 0.25).abs() < 1e-6);
        
        // Massive attack destroys 70% of the network. Density drops to 0.30.
        monitor.current_density = 0.28; 
        // 0.28 is close to p_c (0.25). This should trigger Critical state.
        
        match monitor.check_percolation_state() {
            PercolationState::Critical(p_c) => {
                assert!((p_c - 0.25).abs() < 1e-6);
            },
            _ => panic!("Network failed to detect critical percolation threshold!"),
        }

        // Trigger autonomous healing
        monitor.trigger_emergency_healing();

        // Healing should increase degrees, thus lowering p_c, and returning state to Safe
        match monitor.check_percolation_state() {
            PercolationState::Safe => {},
            _ => panic!("Network failed to heal from critical threshold!"),
        }
    }
}
