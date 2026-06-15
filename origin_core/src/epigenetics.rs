// ============================================================================
// PHASE 36: EPIGENETIC NETWORK MEMORY (epiGA)
// ============================================================================
// Scientific mechanism: Biological Epigenetics & Epigenetic Algorithms
//
// To grant the Swarm long-term memory of bad actors or highly efficient nodes
// without storing massive reputation databases, Origin uses an Epigenetic layer.
// The Rust binary and deterministic rules are the node's static "DNA".
// Environmental stress applies chemical markers:
//   - Methylation (Suppression): Applied when a node drops packets or acts maliciously.
//     This physically down-regulates the node's routing multiplier.
//   - Acetylation (Enhancement): Applied when a node maintains perfect uptime.
//     This physically up-regulates the node's routing multiplier.
// The code (DNA) doesn't change, but its "expression" in the Swarm is dynamically
// controlled by its historical environment.
// ============================================================================

pub enum EnvironmentalStress {
    PacketDrop,
    MaliciousPayload, // Triggered by Artificial Immune System
    PerfectUptime,
    FastRouting,
}

pub struct EpigeneticState {
    pub node_id: usize,
    pub methylation_level: f64, // 0.0 to 1.0 (Higher = Suppressed)
    pub acetylation_level: f64, // 0.0 to 1.0 (Higher = Enhanced)
}

impl EpigeneticState {
    pub fn new(node_id: usize) -> Self {
        Self {
            node_id,
            methylation_level: 0.0, // Default state: no suppression
            acetylation_level: 0.0, // Default state: no enhancement
        }
    }

    /// Apply environmental stress to chemically modify the node's expression
    pub fn apply_environmental_stress(&mut self, stress: EnvironmentalStress) {
        match stress {
            EnvironmentalStress::PacketDrop => {
                self.methylation_level = (self.methylation_level + 0.1).min(1.0);
                self.acetylation_level = (self.acetylation_level - 0.05).max(0.0);
            }
            EnvironmentalStress::MaliciousPayload => {
                self.methylation_level = (self.methylation_level + 0.5).min(1.0); // Heavy suppression
                self.acetylation_level = 0.0; // Strip enhancement
            }
            EnvironmentalStress::PerfectUptime => {
                self.acetylation_level = (self.acetylation_level + 0.05).min(1.0);
                self.methylation_level = (self.methylation_level - 0.02).max(0.0); // Slow healing
            }
            EnvironmentalStress::FastRouting => {
                self.acetylation_level = (self.acetylation_level + 0.1).min(1.0);
            }
        }
    }

    /// Calculate the physical routing priority multiplier.
    /// If highly methylated, multiplier approaches 0.0 (node is ignored).
    /// If highly acetylated, multiplier approaches 2.0 (node is preferred).
    pub fn get_expression_multiplier(&self) -> f64 {
        let base_expression = 1.0;
        
        // Methylation physically crushes expression
        let suppressed = base_expression * (1.0 - self.methylation_level);
        
        // Acetylation enhances what remains
        suppressed * (1.0 + self.acetylation_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epigenetic_memory_suppression_and_enhancement() {
        let mut node = EpigeneticState::new(42);
        
        assert_eq!(node.get_expression_multiplier(), 1.0);
        
        // Node acts maliciously (caught by immune system)
        node.apply_environmental_stress(EnvironmentalStress::MaliciousPayload);
        assert!(node.methylation_level >= 0.5);
        assert!(node.get_expression_multiplier() <= 0.5); // Expression crushed
        
        // Node drops packets
        node.apply_environmental_stress(EnvironmentalStress::PacketDrop);
        assert!(node.get_expression_multiplier() <= 0.4); // Suppressed further
        
        let mut good_node = EpigeneticState::new(99);
        good_node.apply_environmental_stress(EnvironmentalStress::PerfectUptime);
        good_node.apply_environmental_stress(EnvironmentalStress::FastRouting);
        
        assert!(good_node.acetylation_level > 0.0);
        assert!(good_node.get_expression_multiplier() > 1.0); // Expression enhanced
    }
}
