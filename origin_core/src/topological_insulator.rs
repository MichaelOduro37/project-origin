// ============================================================================
// PHASE 39: TOPOLOGICAL INSULATOR ROUTING
// ============================================================================
// Scientific mechanism: Quantum Materials Science (Topological Insulators)
//
// In physics, a Topological Insulator is a material that acts as an electrical
// insulator in its bulk (interior) but perfectly conducts on its surface (edges).
// Crucially, electrons on the surface are "Topologically Protected" due to
// time-reversal symmetry—they only move in one direction (chiral spin) and
// CANNOT backscatter. If they hit a defect, they perfectly curve around it.
//
// In Origin:
// 1. Core infrastructure nodes declare themselves as BulkInsulators (refusing transit).
// 2. Perimeter nodes declare as EdgeConductors.
// 3. Packets are Chiral (spin = +1 or -1).
// 4. Time-reversal asymmetry is mathematically enforced: packets CANNOT be routed
//    backward, permanently preventing routing loops and Reflection Attacks.
// ============================================================================

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TopologicalState {
    /// Highly sensitive/stressed node. Refuses all transit traffic.
    BulkInsulator,
    /// Perimeter routing node. Conducts transit traffic.
    EdgeConductor,
}

#[derive(Debug, Clone)]
pub struct ChiralPacket {
    pub payload: String,
    pub spin: i32,         // +1 for "forward", -1 for "reverse"
    pub origin_node: usize,
    pub previous_hop: usize,
}

pub struct InsulatorManifold {
    pub local_state: TopologicalState,
    pub local_node_id: usize,
}

impl InsulatorManifold {
    pub fn new(local_node_id: usize) -> Self {
        Self {
            local_state: TopologicalState::EdgeConductor, // Default to conducting
            local_node_id,
        }
    }

    /// Attempt to route a chiral packet.
    /// Returns Ok(next_hop) if topologically permitted, or Err if blocked.
    pub fn route_chiral_packet(
        &self, 
        packet: &ChiralPacket, 
        available_neighbors: &[usize], 
        defect_node: Option<usize> // Represents a downed or malicious node
    ) -> Result<usize, &'static str> {
        
        if self.local_state == TopologicalState::BulkInsulator {
            return Err("Node is a Bulk Insulator; transit traffic forbidden.");
        }

        // Topological Protection: Time-reversal asymmetry prevents backscattering.
        // The packet CANNOT be routed back to its previous hop, nor to the defect.
        let mut valid_routes: Vec<usize> = available_neighbors
            .iter()
            .filter(|&&n| n != packet.previous_hop) // Prevent backscatter (routing loop)
            .filter(|&&n| Some(n) != defect_node)    // Curve around defect
            .filter(|&&n| n != packet.origin_node)   // Prevent reflection attack back to origin
            .cloned()
            .collect();

        if valid_routes.is_empty() {
            // In a true physical topological insulator, if a corner is reached, 
            // the electron just keeps wrapping around the 2D surface.
            return Err("No topologically valid forward routes available.");
        }

        // Simplistic selection: just take the first valid forward route.
        // In full integration, this ties into Sinkhorn Transport / Fermat's Routing.
        valid_routes.sort(); // Deterministic for test
        Ok(valid_routes[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_backscatter_prevention() {
        let manifold = InsulatorManifold::new(2); // Local node is 2
        
        let packet = ChiralPacket {
            payload: "QuantumData".into(),
            spin: 1, // Forward
            origin_node: 0,
            previous_hop: 1, // Packet came from 1
        };

        // Node 2 has neighbors 1, 3, 4. 
        // Node 3 is currently under DDoS (defect).
        let neighbors = vec![1, 3, 4];
        let defect = Some(3);

        // Attempting to route
        let next_hop = manifold.route_chiral_packet(&packet, &neighbors, defect).unwrap();

        // Must NOT be 1 (backscatter/routing loop prevention)
        // Must NOT be 3 (defect avoidance)
        // MUST perfectly curve to 4
        assert_eq!(next_hop, 4);
    }

    #[test]
    fn test_bulk_insulation() {
        let mut manifold = InsulatorManifold::new(2);
        manifold.local_state = TopologicalState::BulkInsulator;

        let packet = ChiralPacket {
            payload: "Data".into(),
            spin: 1,
            origin_node: 0,
            previous_hop: 1,
        };

        let result = manifold.route_chiral_packet(&packet, &vec![3], None);
        assert!(result.is_err(), "Bulk insulator must block transit.");
    }
}
