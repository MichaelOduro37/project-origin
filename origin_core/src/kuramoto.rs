// ============================================================================
// PHASE 37: KURAMOTO DISTRIBUTED CLOCK (DECENTRALIZED TIMEKEEPING)
// ============================================================================
// Scientific mechanism: Kuramoto Model of Coupled Oscillators
//
// Eliminates the need for centralized NTP servers. Every node acts as an oscillator
// with an intrinsic frequency. As nodes exchange phases with neighbors, they apply
// the Kuramoto differential equation to gently pull their phase toward the consensus.
// Over time, the entire global Swarm mathematically synchronizes to a single
// heartbeat, providing a perfect, zero-trust temporal ordering layer.
// ============================================================================

pub struct KuramotoOscillator {
    pub phase: f64,             // Current phase (0 to 2π)
    pub natural_frequency: f64, // Intrinsic frequency (omega_i)
    pub coupling_strength: f64, // How strongly it's pulled by neighbors (K)
}

impl KuramotoOscillator {
    pub fn new(natural_frequency: f64, coupling_strength: f64, initial_phase: f64) -> Self {
        Self {
            phase: initial_phase,
            natural_frequency,
            coupling_strength,
        }
    }

    /// Update phase based on neighbor phases using the Kuramoto equation:
    /// dθ_i/dt = ω_i + (K/N) * Σ sin(θ_j - θ_i)
    pub fn update_phase(&mut self, neighbor_phases: &[f64], dt: f64) {
        if neighbor_phases.is_empty() {
            // Free running if no neighbors
            self.phase += self.natural_frequency * dt;
            self.phase %= std::f64::consts::TAU;
            return;
        }

        let n = neighbor_phases.len() as f64;
        let mut sum_sin = 0.0;

        for &neighbor_phase in neighbor_phases {
            sum_sin += (neighbor_phase - self.phase).sin();
        }

        let d_theta = self.natural_frequency + (self.coupling_strength / n) * sum_sin;
        
        self.phase += d_theta * dt;
        
        // Normalize phase between 0 and 2π
        self.phase %= std::f64::consts::TAU;
        if self.phase < 0.0 {
            self.phase += std::f64::consts::TAU;
        }
    }

    /// Retrieve the abstract "Global Time Ticks" based on total phase cycles.
    /// In a real implementation, this would track total accumulated phase,
    /// but for this demonstration, it maps the 0-2π phase to a heartbeat tick.
    pub fn get_global_time(&self) -> f64 {
        self.phase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kuramoto_synchronization() {
        // Initialize an array of oscillators with random phases and slightly varying frequencies
        let mut oscillators = vec![
            KuramotoOscillator::new(1.0, 5.0, 0.5),
            KuramotoOscillator::new(1.05, 5.0, 3.1),
            KuramotoOscillator::new(0.95, 5.0, 1.2),
            KuramotoOscillator::new(1.02, 5.0, 5.0),
        ];

        let dt = 0.01;

        // Simulate coupling over time
        for _ in 0..1000 {
            // Snapshot current phases
            let phases: Vec<f64> = oscillators.iter().map(|o| o.phase).collect();
            
            // Update all oscillators (all-to-all coupling for this test)
            for i in 0..oscillators.len() {
                // In a P2P network, it would only be immediate neighbors, 
                // but here we simulate a fully connected cluster.
                let mut neighbor_phases = phases.clone();
                neighbor_phases.remove(i);
                oscillators[i].update_phase(&neighbor_phases, dt);
            }
        }

        // After sufficient iterations with high coupling, the phases should be nearly identical (synchronized)
        let final_phases: Vec<f64> = oscillators.iter().map(|o| o.phase).collect();
        let first_phase = final_phases[0];
        
        for &phase in &final_phases[1..] {
            // Because they wrap at 2PI, we check the cyclic difference
            let diff = (phase - first_phase).abs();
            let cyclic_diff = diff.min(std::f64::consts::TAU - diff);
            
            assert!(cyclic_diff < 0.1, "Oscillators failed to synchronize! Diff: {}", cyclic_diff);
        }
    }
}
