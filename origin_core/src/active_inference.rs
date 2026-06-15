// ============================================================================
// PHASE 25: ACTIVE INFERENCE AND THE FREE ENERGY PRINCIPLE
// ============================================================================
// Scientific mechanism: Karl Friston's Free Energy Principle states that biological
// systems maintain their existence by minimizing Variational Free Energy 
// (mathematically equivalent to prediction error or "surprise").
// 
// Instead of reactive consensus protocols (Paxos/Raft), Origin uses Active Inference.
// 1. The Generative Model predicts the optimal state (e.g., target network load).
// 2. Incoming traffic acts as sensory input.
// 3. The node calculates Variational Free Energy (divergence from prediction).
// 4. Active Inference physically acts on the network (rerouting packets) to force
//    the sensory input to match the internal prediction.
// ============================================================================

pub struct GenerativeModel {
    pub expected_mu: f64,
    pub expected_sigma: f64,
}

#[derive(Debug, PartialEq)]
pub enum InferenceAction {
    ConsensusMaintained,
    ActivelyShedLoad(f64),
    ActivelyPullLoad(f64),
}

impl GenerativeModel {
    pub fn new(expected_mu: f64, expected_sigma: f64) -> Self {
        Self {
            expected_mu,
            expected_sigma,
        }
    }

    /// Calculates the Variational Free Energy (Surprise) of the sensory input.
    /// F ≈ (x - μ)² / 2σ² + ln(σ)
    pub fn calculate_free_energy(&self, sensory_input: f64) -> f64 {
        let variance = self.expected_sigma.powi(2);
        let error_term = (sensory_input - self.expected_mu).powi(2) / (2.0 * variance);
        let complexity_term = self.expected_sigma.ln();
        
        error_term + complexity_term
    }

    /// Performs Active Inference: Decides what physical action to take to minimize Free Energy.
    pub fn active_inference(&self, sensory_input: f64, free_energy: f64) -> InferenceAction {
        // If surprise is low, we are in predictive equilibrium.
        // Assuming ln(sigma) is the baseline, if error_term is small.
        let variance = self.expected_sigma.powi(2);
        let error_term = (sensory_input - self.expected_mu).powi(2) / (2.0 * variance);

        // If prediction error is within 1 standard deviation (error_term <= 0.5)
        if error_term <= 0.5 {
            return InferenceAction::ConsensusMaintained;
        }

        // Free Energy is too high! The agent must act to change the sensory input.
        // How much action to take is proportional to the gradient of the Free Energy.
        // dF/dx = (x - μ) / σ²
        let gradient = (sensory_input - self.expected_mu) / variance;

        if gradient > 0.0 {
            // Input is too high. Actively shed load.
            InferenceAction::ActivelyShedLoad(gradient * self.expected_sigma)
        } else {
            // Input is too low. Actively pull load.
            InferenceAction::ActivelyPullLoad(gradient.abs() * self.expected_sigma)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_energy_minimization() {
        // Generative model expects network load to be 50% with a 5% standard deviation.
        let agent = GenerativeModel::new(50.0, 5.0);

        // Sensory input matches prediction exactly.
        let perfect_input = 50.0;
        let fe_perfect = agent.calculate_free_energy(perfect_input);
        let action_perfect = agent.active_inference(perfect_input, fe_perfect);
        assert_eq!(action_perfect, InferenceAction::ConsensusMaintained);

        // Sensory input spikes (Surprise!)
        let spike_input = 70.0;
        let fe_spike = agent.calculate_free_energy(spike_input);
        assert!(fe_spike > fe_perfect, "Free Energy must increase with prediction error!");
        
        let action_spike = agent.active_inference(spike_input, fe_spike);
        match action_spike {
            InferenceAction::ActivelyShedLoad(amt) => {
                assert!(amt > 0.0);
            },
            _ => panic!("Agent failed to perform active inference to shed load!"),
        }

        // Sensory input drops
        let drop_input = 30.0;
        let fe_drop = agent.calculate_free_energy(drop_input);
        let action_drop = agent.active_inference(drop_input, fe_drop);
        match action_drop {
            InferenceAction::ActivelyPullLoad(amt) => {
                assert!(amt > 0.0);
            },
            _ => panic!("Agent failed to perform active inference to pull load!"),
        }
    }
}
