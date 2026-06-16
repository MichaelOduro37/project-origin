// ============================================================================
// PHASE 64: HORIZONTAL GENE TRANSFER (ZERO-DAY IMMUNITY PLASMIDS)
// ============================================================================
// Scientific mechanism: Evolutionary Biology / Microbiology
//
// Traditional networks require slow software updates to patch zero-day exploits.
// In biology, bacteria pass antibiotic resistance instantly to their neighbors 
// using "Plasmids" (circular DNA payloads) via Horizontal Gene Transfer (HGT).
//
// Origin emulates this. When a node develops a defense against a novel zero-day
// attack, it packages the bytecode into a ResistancePlasmid and shoots it 
// horizontally to peers. The peers hot-load the plasmid directly into their 
// runtime, gaining instant immunity without restarting.
// ============================================================================

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ResistancePlasmid {
    pub attack_signature: String, // The hash or signature of the zero-day exploit
    pub defense_bytecode: Vec<u8>, // The executable logic required to neutralize it
}

impl ResistancePlasmid {
    pub fn new(attack_signature: String, defense_bytecode: Vec<u8>) -> Self {
        Self {
            attack_signature,
            defense_bytecode,
        }
    }
}

pub struct BacterialNode {
    pub node_id: usize,
    pub active_plasmids: HashMap<String, Vec<u8>>, // Hot-loaded resistance genes
}

impl BacterialNode {
    pub fn new(node_id: usize) -> Self {
        Self {
            node_id,
            active_plasmids: HashMap::new(),
        }
    }

    /// Checks if the node is vulnerable to a specific attack signature.
    pub fn is_vulnerable(&self, attack_signature: &str) -> bool {
        !self.active_plasmids.contains_key(attack_signature)
    }

    /// The node processes an incoming zero-day attack.
    /// If vulnerable, it returns Err.
    /// If it has the plasmid, it executes the defense bytecode and survives.
    pub fn process_attack(&self, attack_signature: &str) -> Result<(), &'static str> {
        if self.is_vulnerable(attack_signature) {
            Err("NODE COMPROMISED: Vulnerable to zero-day attack.")
        } else {
            // In a full implementation, the defense_bytecode would be executed 
            // via the Ribosomal VM (Phase 29) to neutralize the threat.
            Ok(())
        }
    }

    /// Receives a plasmid horizontally from a peer and hot-loads it into active memory.
    /// No restart required. The node gains instant immunity.
    pub fn hot_load_plasmid(&mut self, plasmid: ResistancePlasmid) {
        // Here we would use Negative Selection Algorithms (Phase 21) to ensure
        // the plasmid is not actually malicious code designed to brick the node.
        
        self.active_plasmids.insert(plasmid.attack_signature, plasmid.defense_bytecode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_day_vulnerability() {
        let node = BacterialNode::new(1);
        let attack = "CVE-2027-ZERO-DAY";
        
        assert!(node.is_vulnerable(attack));
        assert!(node.process_attack(attack).is_err());
    }

    #[test]
    fn test_horizontal_gene_transfer() {
        let mut node_a = BacterialNode::new(1);
        let mut node_b = BacterialNode::new(2);
        let attack = "CVE-2027-ZERO-DAY";

        // Node A gets compromised
        assert!(node_a.process_attack(attack).is_err());
        
        // Node A (or another peer) synthesizes a defense plasmid
        let plasmid = ResistancePlasmid::new(
            attack.to_string(),
            b"DEFEND".to_vec(), // Hypothetical defense bytecode
        );

        // Horizontal Gene Transfer from A to B
        node_b.hot_load_plasmid(plasmid);

        // Node B is instantly immune without a software update
        assert!(!node_b.is_vulnerable(attack));
        assert!(node_b.process_attack(attack).is_ok());
    }
}
