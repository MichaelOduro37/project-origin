// ============================================================================
// PHASE 7: QUANTUM-INSPIRED GENETIC ALGORITHM (QGA) ROUTING
// ============================================================================

use serde::{Serialize, Deserialize};

/// Represents a single Qubit in a Quantum Chromosome.
/// `alpha`: probability amplitude of state |0> (Path Not Chosen)
/// `beta`: probability amplitude of state |1> (Path Chosen)
/// Constraint: |alpha|^2 + |beta|^2 = 1
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Qubit {
    pub alpha: f64,
    pub beta: f64,
}

impl Qubit {
    /// Initialize a qubit in a perfect 50/50 superposition
    pub fn new_superposition() -> Self {
        let inv_sqrt_2 = 1.0 / 2.0_f64.sqrt();
        Qubit {
            alpha: inv_sqrt_2,
            beta: inv_sqrt_2,
        }
    }

    /// Apply a quantum rotation gate to shift the probability amplitude
    /// Positive delta_theta increases the probability of |1> (Path Chosen)
    /// Negative delta_theta decreases the probability of |1>
    pub fn apply_rotation_gate(&mut self, delta_theta: f64) {
        let mut current_theta = self.beta.atan2(self.alpha);
        current_theta += delta_theta;
        
        // Clamp to [0, PI/2] so alpha and beta stay in [0, 1]
        let max_theta = std::f64::consts::PI / 2.0;
        if current_theta > max_theta {
            current_theta = max_theta;
        } else if current_theta < 0.0 {
            current_theta = 0.0;
        }

        self.alpha = current_theta.cos();
        self.beta = current_theta.sin();
    }

    /// Measure the qubit. Collapses the superposition into a classical bit (true = 1, false = 0)
    /// based on the |beta|^2 probability.
    pub fn measure(&self) -> bool {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as f64;
        let threshold = (timestamp % 1000.0) / 1000.0;
        let prob_1 = self.beta.powi(2);
        threshold < prob_1
    }
}

/// A Quantum Chromosome representing a superposition of all possible routing paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QChromosome {
    /// Map of peer IPs to their routing probability qubit
    pub paths: std::collections::HashMap<String, Qubit>,
}

impl QChromosome {
    pub fn new() -> Self {
        QChromosome {
            paths: std::collections::HashMap::new(),
        }
    }

    /// Add a newly discovered peer to the superposition state
    pub fn register_peer(&mut self, ip: String) {
        if !self.paths.contains_key(&ip) {
            self.paths.insert(ip, Qubit::new_superposition());
        }
    }

    /// Update the fitness of a path. Higher fitness = positive rotation.
    /// Fitness should be derived from inverse latency + low Tensegrity tension
    pub fn update_fitness(&mut self, ip: &str, fitness: f64) {
        if let Some(qubit) = self.paths.get_mut(ip) {
            // Delta theta is proportional to the fitness advantage
            // We use a small learning rate (e.g., 0.05) to ensure stable convergence
            let delta_theta = fitness * 0.05; 
            qubit.apply_rotation_gate(delta_theta);
        }
    }

    /// Collapse the entire routing table into a single optimal peer destination.
    /// This resolves the superposition for a specific packet transmission.
    pub fn collapse_to_optimal_route(&self) -> Option<String> {
        let mut best_ip = None;
        let mut highest_beta_sq = -1.0;

        for (ip, qubit) in &self.paths {
            // When we actually need to send, we probabilistically collapse
            // Or we can deterministically pick the highest amplitude for the greedy physical route
            let prob_1 = qubit.beta.powi(2);
            if prob_1 > highest_beta_sq {
                highest_beta_sq = prob_1;
                best_ip = Some(ip.clone());
            }
        }
        best_ip
    }

    /// Retrieve all registered peers to be used by other algorithms (e.g. Fermionic Routing)
    pub fn get_all_peers(&self) -> Vec<String> {
        self.paths.keys().cloned().collect()
    }

    /// Retrieves all peers along with their physical cost (1.0 - probability of selection)
    /// Used by Optimal Transport (Sinkhorn) to build the Cost Matrix.
    pub fn get_all_peers_with_cost(&self) -> Vec<(String, f64)> {
        self.paths.iter().map(|(ip, qubit)| {
            let prob_1 = qubit.beta.powi(2);
            let cost = 1.0 - prob_1; // High fitness = high prob = low cost
            (ip.clone(), cost.max(0.01))
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qubit_normalization() {
        let mut q = Qubit::new_superposition();
        assert!((q.alpha.powi(2) + q.beta.powi(2) - 1.0).abs() < 1e-6);
        
        q.apply_rotation_gate(0.15); // Rotate by 0.15 radians
        assert!((q.alpha.powi(2) + q.beta.powi(2) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_qga_convergence() {
        let mut table = QChromosome::new();
        table.register_peer("192.168.1.100".to_string());
        table.register_peer("192.168.1.101".to_string());

        // Simulate 100 iterations where .101 is much faster (higher fitness) than .100
        for _ in 0..100 {
            table.update_fitness("192.168.1.100", -0.5); // Poor fitness
            table.update_fitness("192.168.1.101", 1.2);  // High fitness
        }

        let best = table.collapse_to_optimal_route().unwrap();
        assert_eq!(best, "192.168.1.101");

        // Mathematically verify that .101's beta (probability of selection) approached 1.0
        let prob_101 = table.paths.get("192.168.1.101").unwrap().beta.powi(2);
        assert!(prob_101 > 0.95);
        
        let prob_100 = table.paths.get("192.168.1.100").unwrap().beta.powi(2);
        assert!(prob_100 < 0.05);
    }
}
