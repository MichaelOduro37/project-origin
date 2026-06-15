// ============================================================================
// PHASE 40: BOSE-EINSTEIN CONDENSATE CONSENSUS
// ============================================================================
// Scientific mechanism: Quantum Mechanics (Statistical Physics)
//
// In quantum mechanics, a Bose-Einstein Condensate (BEC) forms when a gas of 
// bosons is cooled below a critical temperature (Tc). At this threshold, a 
// macroscopic fraction of particles instantly collapses into the lowest quantum 
// state (the ground state), acting as a single entity.
//
// In Origin, we replace standard voting algorithms (Paxos/Raft) with a purely 
// thermodynamic phase transition.
// 1. "Temperature" (T) is defined as the statistical variance of proposed states.
// 2. As nodes synchronize (via Kuramoto), variance drops, mathematically "cooling" the network.
// 3. When T < Tc, the network undergoes a phase transition into a BEC.
// 4. All nodes collapse into the Ground State (Consensus) with O(1) message complexity.
// ============================================================================

#[derive(Debug, PartialEq, Clone)]
pub enum CondensateState {
    /// Disagreement. Variance is high.
    ThermalGas,
    /// Instant, spontaneous consensus.
    BoseEinsteinCondensate { ground_state: String },
}

pub struct BoseGasEngine {
    pub critical_temperature: f64, // Tc
}

impl BoseGasEngine {
    pub fn new(critical_temperature: f64) -> Self {
        Self { critical_temperature }
    }

    /// Calculate the network "temperature" based on the variance of state proposals.
    /// In a real distributed system, these proposals would be numerical representations 
    /// (e.g. hashes) of the desired global state block.
    pub fn calculate_temperature(&self, proposals: &[f64]) -> f64 {
        if proposals.is_empty() {
            return 0.0;
        }
        
        let mean = proposals.iter().sum::<f64>() / proposals.len() as f64;
        let variance = proposals.iter().map(|value| {
            let diff = mean - *value;
            diff * diff
        }).sum::<f64>() / proposals.len() as f64;

        // Temperature is directly proportional to variance
        variance
    }

    /// Check if the system has cooled enough to undergo a phase transition.
    /// If T < Tc, we achieve instantaneous BEC consensus (the mean of the proposals).
    pub fn check_condensation(&self, temperature: f64, proposals: &[f64]) -> CondensateState {
        if proposals.is_empty() {
            return CondensateState::ThermalGas;
        }

        if temperature < self.critical_temperature {
            // The ground state is the converged mean of the synchronized proposals
            let mean = proposals.iter().sum::<f64>() / proposals.len() as f64;
            
            CondensateState::BoseEinsteinCondensate {
                // In production, this would map back to the block hash
                ground_state: format!("Converged_State_{:.2}", mean),
            }
        } else {
            CondensateState::ThermalGas
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_gas_no_consensus() {
        let engine = BoseGasEngine::new(0.5); // Tc = 0.5
        
        // High variance proposals
        let proposals = vec![10.0, 50.0, 100.0, 5.0];
        let t = engine.calculate_temperature(&proposals);
        
        // T should be very high
        assert!(t > 500.0);
        
        let state = engine.check_condensation(t, &proposals);
        assert_eq!(state, CondensateState::ThermalGas);
    }

    #[test]
    fn test_bec_phase_transition_consensus() {
        let engine = BoseGasEngine::new(0.5); // Tc = 0.5
        
        // Low variance proposals (post-Kuramoto cooling)
        let proposals = vec![42.1, 42.0, 42.2, 41.9];
        let t = engine.calculate_temperature(&proposals);
        
        // T should be below Tc (0.5)
        assert!(t < 0.5);
        
        let state = engine.check_condensation(t, &proposals);
        
        match state {
            CondensateState::BoseEinsteinCondensate { ground_state } => {
                assert_eq!(ground_state, "Converged_State_42.05");
            },
            _ => panic!("Expected BEC Phase Transition!"),
        }
    }
}
