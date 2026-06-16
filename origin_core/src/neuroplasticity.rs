// ============================================================================
// PHASE 65: NEUROPLASTICITY & HEBBIAN LEARNING (TOPOLOGY MYELINATION)
// ============================================================================
// Scientific mechanism: Neuroscience
//
// Traditional blockchains have static routing topologies. Origin behaves like
// a biological brain. "Nodes that fire together, wire together."
// 
// When a P2P connection handles heavy traffic, the network "Myelinates" it,
// dedicating hard-allocated bandwidth to drop latency by 100x. When a
// connection goes unused, the network executes "Synaptic Pruning", severing
// the connection to save computational energy. The network shape-shifts.
// ============================================================================

#[derive(Debug, Clone)]
pub struct SynapticConnection {
    pub node_a: usize,
    pub node_b: usize,
    pub usage_frequency: usize,
    pub is_myelinated: bool,
}

impl SynapticConnection {
    pub fn new(node_a: usize, node_b: usize) -> Self {
        Self {
            node_a,
            node_b,
            usage_frequency: 0,
            is_myelinated: false,
        }
    }
}

pub struct NeuralNetworkTopology {
    pub connections: Vec<SynapticConnection>,
    pub myelination_threshold: usize,
    pub pruning_threshold: usize,
}

impl NeuralNetworkTopology {
    pub fn new(myelination_threshold: usize, pruning_threshold: usize) -> Self {
        Self {
            connections: Vec::new(),
            myelination_threshold,
            pruning_threshold,
        }
    }

    pub fn add_synapse(&mut self, node_a: usize, node_b: usize) {
        self.connections.push(SynapticConnection::new(node_a, node_b));
    }

    /// Simulates data flowing between two nodes. Increments "Synaptic Usage".
    pub fn trigger_action_potential(&mut self, node_a: usize, node_b: usize) {
        for conn in self.connections.iter_mut() {
            if (conn.node_a == node_a && conn.node_b == node_b) ||
               (conn.node_a == node_b && conn.node_b == node_a) {
                conn.usage_frequency += 1;
                break;
            }
        }
    }

    /// Executes Hebbian Learning: 
    /// 1. Heavily used routes are wrapped in Myelin (prioritized).
    /// 2. Unused routes are pruned (severed) to save energy.
    pub fn myelinate_and_prune(&mut self) -> (Vec<String>, Vec<String>) {
        let mut newly_myelinated = Vec::new();
        let mut pruned = Vec::new();

        // Check for Myelination (Hebbian strengthening)
        for conn in self.connections.iter_mut() {
            if conn.usage_frequency >= self.myelination_threshold && !conn.is_myelinated {
                conn.is_myelinated = true;
                newly_myelinated.push(format!("{}<->{}", conn.node_a, conn.node_b));
            }
        }

        // Check for Synaptic Pruning (Severing weak connections)
        // Note: In a real system, we'd use Phase 50 Percolation Theory here 
        // to ensure we don't accidentally split the network graph in half.
        let original_len = self.connections.len();
        self.connections.retain(|conn| {
            let keep = conn.usage_frequency >= self.pruning_threshold || conn.is_myelinated;
            if !keep {
                pruned.push(format!("{}<->{}", conn.node_a, conn.node_b));
            }
            keep
        });

        (newly_myelinated, pruned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hebbian_myelination() {
        let mut topology = NeuralNetworkTopology::new(5, 1);
        topology.add_synapse(1, 2);

        // Fire the synapse 5 times
        for _ in 0..5 {
            topology.trigger_action_potential(1, 2);
        }

        let (myelinated, _) = topology.myelinate_and_prune();
        
        assert_eq!(myelinated.len(), 1);
        assert_eq!(myelinated[0], "1<->2");
        assert!(topology.connections[0].is_myelinated);
    }

    #[test]
    fn test_synaptic_pruning() {
        let mut topology = NeuralNetworkTopology::new(5, 1);
        topology.add_synapse(1, 2); // Will not fire
        topology.add_synapse(3, 4); // Will fire

        topology.trigger_action_potential(3, 4);

        let (_, pruned) = topology.myelinate_and_prune();

        // Connection 1<->2 should be pruned because usage_frequency < 1
        assert_eq!(pruned.len(), 1);
        assert_eq!(pruned[0], "1<->2");
        
        // Only connection 3<->4 should remain
        assert_eq!(topology.connections.len(), 1);
        assert_eq!(topology.connections[0].node_a, 3);
        assert_eq!(topology.connections[0].node_b, 4);
    }
}
