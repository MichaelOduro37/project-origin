// ============================================================================
// PHASE 21: CAUSAL INFERENCE & DO-CALCULUS
// ============================================================================
// Scientific mechanism: Blindly reacting to system metrics causes catastrophic
// cascading failures. (e.g. Node A sheds load -> Node B overheats -> Node B sheds -> Crash).
//
// Using Judea Pearl's Do-Calculus, we map the system as a Causal DAG:
// [NodeLoad] --> [ShedLoad] --> [NeighborCurvature] --> [GlobalHealth]
//
// Before making a heuristic decision, the Swarm simulates an intervention: P(Y | do(X)).
// If `do(shed_load)` mathematically predicts a collapse in GlobalHealth due to 
// a vulnerable NeighborCurvature, the Swarm suppresses its heuristic reaction!
// ============================================================================

pub struct CausalEngine;

impl CausalEngine {
    /// Evaluates the causal effect of an intervention using Do-Calculus structural equations.
    /// 
    /// Structural Equations:
    /// NeighborCurvature = BaseCurvature + (0.8 * ShedLoad_amount)
    /// GlobalHealth = 100.0 - (NodeLoad * 0.2) - (NeighborCurvature^2 * 0.05)
    ///
    /// By simulating `do(shed_load = true)` vs `do(shed_load = false)`, we can foresee 
    /// if the intervention is helpful or if it triggers a non-linear curvature cascade.
    pub fn evaluate_intervention(
        do_shed_load: bool,
        current_node_load: f64,
        base_neighbor_curvature: f64,
    ) -> f64 {
        // Step 1: Apply the Intervention `do(X)`
        // If we shed load, our load drops to 0, but we push that load to neighbors.
        let (simulated_node_load, shed_amount) = if do_shed_load {
            (0.0, current_node_load)
        } else {
            (current_node_load, 0.0)
        };

        // Step 2: Causal Link: ShedLoad -> NeighborCurvature
        // The load we shed increases the curvature of our neighbors.
        let simulated_neighbor_curvature = base_neighbor_curvature + (0.1 * shed_amount);

        // Step 3: Causal Link: {NodeLoad, NeighborCurvature} -> GlobalHealth
        // High neighbor curvature causes non-linear damage to global health (cascading failure).
        let global_health = 100.0 
            - (simulated_node_load * 0.2) 
            - (simulated_neighbor_curvature.powf(2.0) * 0.05);

        global_health
    }

    /// Queries the Causal DAG to determine if an intervention should be executed.
    pub fn should_intervene(current_node_load: f64, base_neighbor_curvature: f64) -> (bool, f64) {
        // Simulate P(Health | do(shed = false))
        let health_without_intervention = Self::evaluate_intervention(false, current_node_load, base_neighbor_curvature);
        
        // Simulate P(Health | do(shed = true))
        let health_with_intervention = Self::evaluate_intervention(true, current_node_load, base_neighbor_curvature);

        // We only intervene if it CAUSALLY improves global health.
        let should_shed = health_with_intervention > health_without_intervention;
        let predicted_benefit = health_with_intervention - health_without_intervention;

        (should_shed, predicted_benefit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_inference_safe_to_shed() {
        // Node is at 80% load. Neighbors are completely empty (Curvature 0.0).
        // Shedding should cleanly improve global health without cascading.
        let (should_shed, benefit) = CausalEngine::should_intervene(80.0, 0.0);
        assert!(should_shed, "Engine should shed load when neighbors are empty.");
        assert!(benefit > 0.0);
    }

    #[test]
    fn test_causal_inference_prevent_cascade() {
        // Node is at 80% load. BUT Neighbors are already highly stressed (Curvature 35.0).
        // Shedding now would push neighbors into a non-linear cascading failure.
        let (should_shed, benefit) = CausalEngine::should_intervene(80.0, 35.0);
        assert!(!should_shed, "Engine must suppress shedding to prevent causal cascade!");
        assert!(benefit < 0.0);
    }
}
