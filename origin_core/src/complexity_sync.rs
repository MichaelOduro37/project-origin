// ============================================================================
// PHASE 23: COMPLEXITY SYNCHRONIZATION AS DISTRIBUTED CONTROL
// ============================================================================
// Scientific mechanism: In a multi-trillion node Swarm, central autoscaling 
// or static threshold heuristics fail. The network behaves like a chaotic system.
//
// To achieve self-healing distributed control, we use Complexity Synchronization.
// Each node calculates its local Lyapunov Exponent (a measure of chaos and 
// trajectory divergence).
// 
// - If the exponent is low (highly stable), the node is under-utilized and pulls load.
// - If the exponent is high (highly chaotic), the node is diverging and sheds load.
// 
// By aligning local chaos to a Network Target Chaos, the Swarm achieves organic 
// equilibrium without a coordinator.
// ============================================================================

pub enum LoadAction {
    PullLoad(f64), // Amount of load to request
    ShedLoad(f64), // Amount of load to shed
    Stable,
}

pub struct ComplexityEngine;

impl ComplexityEngine {
    /// Calculates the largest Lyapunov Exponent for a 1D time series (e.g., NodeLoad).
    /// A positive exponent indicates chaos (divergence).
    /// A negative exponent indicates stability (convergence).
    pub fn calculate_lyapunov_exponent(history: &[f64]) -> f64 {
        if history.len() < 2 {
            return 0.0;
        }

        let mut sum_log_divergence = 0.0;
        let mut valid_steps = 0;

        for i in 0..(history.len() - 1) {
            let dx = (history[i + 1] - history[i]).abs();
            // Avoid log(0) by using a tiny epsilon
            let divergence = dx.max(1e-6);
            sum_log_divergence += divergence.ln();
            valid_steps += 1;
        }

        sum_log_divergence / (valid_steps as f64)
    }

    /// Synchronizes the node's local complexity with the Swarm's target complexity.
    pub fn synchronize(local_lyapunov: f64, target_lyapunov: f64, current_load: f64) -> LoadAction {
        let chaos_delta = local_lyapunov - target_lyapunov;

        // If delta is close to 0, we are perfectly synchronized with the Swarm.
        if chaos_delta.abs() < 0.5 {
            return LoadAction::Stable;
        }

        if chaos_delta > 0.0 {
            // We are TOO CHAOTIC. Our state is diverging rapidly.
            // We must SHED load to restabilize.
            let shed_amount = (chaos_delta * 5.0).min(current_load * 0.5); // Cap shedding at 50%
            LoadAction::ShedLoad(shed_amount)
        } else {
            // We are TOO STABLE (low entropy). We are under-utilizing our capacity
            // compared to the rest of the Swarm.
            // We PULL load to help the network.
            let pull_amount = chaos_delta.abs() * 5.0;
            LoadAction::PullLoad(pull_amount)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lyapunov_stable_trajectory() {
        // A stable node where load barely changes.
        let history = vec![50.0, 50.1, 50.0, 49.9, 50.0];
        let lambda = ComplexityEngine::calculate_lyapunov_exponent(&history);
        
        // Stable systems have negative or very low log-divergence
        assert!(lambda < -1.0, "Stable trajectory should yield a very low Lyapunov exponent. Got {}", lambda);
        
        // Syncing a stable node to a highly active swarm target (target = 1.0)
        let action = ComplexityEngine::synchronize(lambda, 1.0, 50.0);
        match action {
            LoadAction::PullLoad(amt) => assert!(amt > 0.0),
            _ => panic!("Node should pull load when it is too stable!"),
        }
    }

    #[test]
    fn test_lyapunov_chaotic_trajectory() {
        // A highly chaotic node where load fluctuates wildly.
        let history = vec![10.0, 90.0, 20.0, 85.0, 15.0];
        let lambda = ComplexityEngine::calculate_lyapunov_exponent(&history);
        
        // Chaotic systems have high positive log-divergence
        assert!(lambda > 3.0, "Chaotic trajectory should yield a high Lyapunov exponent. Got {}", lambda);
        
        // Syncing a chaotic node to a standard swarm target (target = 1.0)
        let action = ComplexityEngine::synchronize(lambda, 1.0, 50.0);
        match action {
            LoadAction::ShedLoad(amt) => assert!(amt > 0.0),
            _ => panic!("Node should shed load when it is too chaotic!"),
        }
    }
}
