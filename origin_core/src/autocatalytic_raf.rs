// ============================================================================
// PHASE 27: AUTOCATALYTIC SETS & RAF THEORY
// ============================================================================
// Scientific mechanism: Based on Stuart Kauffman's origin-of-life models.
// A network achieves "Catalytic Closure" when a subset of nodes can 
// mutually support all necessary routing, security, and consensus operations
// without relying on the external chaotic network.
//
// Food (F): Initial network capabilities provided by seed nodes.
// Reactions (R): Protocol interactions (e.g., Handshakes producing Routing).
// Catalysts (C): Nodes facilitating the reactions.
// ============================================================================

use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Molecule(pub String); // Represents a Network Capability or State

#[derive(Debug, Clone)]
pub struct Reaction {
    pub id: usize,
    pub inputs: Vec<Molecule>,
    pub outputs: Vec<Molecule>,
    pub catalysts: Vec<Molecule>, // Capabilities/Nodes required to catalyze this
}

pub struct RAFEngine {
    pub food: HashSet<Molecule>,
    pub reactions: Vec<Reaction>,
}

impl RAFEngine {
    pub fn new(food: Vec<Molecule>, reactions: Vec<Reaction>) -> Self {
        let mut food_set = HashSet::new();
        for f in food {
            food_set.insert(f);
        }
        Self {
            food: food_set,
            reactions,
        }
    }

    /// Finds the Maximal RAF (Reflexively Autocatalytic and Food-generated) Set.
    /// Returns the IDs of the reactions that form the closed autocatalytic core.
    pub fn find_maximal_raf(&self) -> Vec<usize> {
        let mut current_r: HashSet<usize> = self.reactions.iter().map(|r| r.id).collect();

        loop {
            // 1. Compute the F-generated closure
            let mut closure = self.food.clone();
            let mut f_generated_r = HashSet::new();
            let mut changed = true;

            // Keep firing reactions until no new molecules are produced
            while changed {
                changed = false;
                for r in &self.reactions {
                    if current_r.contains(&r.id) && !f_generated_r.contains(&r.id) {
                        // Check if all inputs are present in the closure
                        let can_fire = r.inputs.iter().all(|input| closure.contains(input));
                        if can_fire {
                            f_generated_r.insert(r.id);
                            for output in &r.outputs {
                                if closure.insert(output.clone()) {
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }

            // 2. Filter for Reflexive Autocatalysis
            // A reaction is autocatalytic if at least one of its catalysts is present in the closure.
            let mut next_r = HashSet::new();
            for r_id in &f_generated_r {
                let r = self.reactions.iter().find(|x| x.id == *r_id).unwrap();
                let is_catalyzed = r.catalysts.iter().any(|cat| closure.contains(cat));
                if is_catalyzed {
                    next_r.insert(*r_id);
                }
            }

            // 3. If the set didn't shrink, we've found the Maximal RAF
            if next_r == current_r {
                break;
            }
            current_r = next_r;
        }

        let mut result: Vec<usize> = current_r.into_iter().collect();
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raf_catalytic_closure() {
        // M1: Peer Discovery (Food)
        // M2: Basic Key Exchange (Food)
        // M3: Encrypted Tunnel (Output)
        // M4: Topology Map (Output)
        // M5: Tensegrity Consensus (Output)
        // M6: Relay Node (Catalyst - Food)
        
        let food = vec![
            Molecule("PeerDiscovery".into()),
            Molecule("BasicKey".into()),
            Molecule("RelayNode".into()),
        ];

        let r1 = Reaction {
            id: 1,
            inputs: vec![Molecule("PeerDiscovery".into()), Molecule("BasicKey".into())],
            outputs: vec![Molecule("EncryptedTunnel".into())],
            catalysts: vec![Molecule("RelayNode".into())], // Catalyzed by Food
        };

        let r2 = Reaction {
            id: 2,
            inputs: vec![Molecule("EncryptedTunnel".into())],
            outputs: vec![Molecule("TopologyMap".into())],
            catalysts: vec![Molecule("EncryptedTunnel".into())], // Catalyzed by the network itself
        };

        // R3 represents a reaction that is NEVER catalyzed (Dead end)
        let r3 = Reaction {
            id: 3,
            inputs: vec![Molecule("TopologyMap".into())],
            outputs: vec![Molecule("TensegrityConsensus".into())],
            catalysts: vec![Molecule("MissingCatalyst".into())], // Unachievable
        };

        let engine = RAFEngine::new(food, vec![r1, r2, r3]);
        let max_raf = engine.find_maximal_raf();

        // R1 and R2 should form the Autocatalytic Set. R3 is rejected.
        assert_eq!(max_raf, vec![1, 2], "RAF engine failed to extract the self-sustaining network core!");
    }
}
