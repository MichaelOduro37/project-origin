// ============================================================================
// PHASE 67: SYMBIOGENESIS (ENDOSYMBIOTIC THEORY)
// ============================================================================
// Scientific mechanism: Evolutionary Biology
//
// Traditional interoperability relies on fragile cross-chain bridges. Origin
// uses Symbiogenesis. When a Layer 2 sidechain connects to Origin, Origin
// executes "Phagocytosis" - physically engulfing the external chain.
// The sidechain loses its independent consensus mechanism and becomes an
// "Endosymbiotic Organelle" (like a mitochondrion) permanently living inside
// Origin's cellular membrane, pumping its compute directly into the main state.
// ============================================================================

#[derive(Debug, Clone)]
pub struct FreeLivingChain {
    pub name: String,
    pub consensus_overhead: usize, // Wasted energy/compute on its own consensus
    pub execution_power: usize,    // Useful compute it generates
}

impl FreeLivingChain {
    pub fn new(name: &str, consensus_overhead: usize, execution_power: usize) -> Self {
        Self {
            name: name.to_string(),
            consensus_overhead,
            execution_power,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EndosymbioticOrganelle {
    pub organelle_id: usize,
    pub name: String,
    pub execution_power: usize, // Retains execution, stripped of consensus overhead
}

impl EndosymbioticOrganelle {
    pub fn from_chain(id: usize, chain: FreeLivingChain) -> Self {
        Self {
            organelle_id: id,
            name: format!("{}_Mitochondrial_Subnet", chain.name),
            execution_power: chain.execution_power, // 100% of power now goes to Origin
        }
    }
}

pub struct OriginCell {
    pub total_execution_power: usize,
    pub organelles: Vec<EndosymbioticOrganelle>,
    next_organelle_id: usize,
}

impl OriginCell {
    pub fn new(initial_power: usize) -> Self {
        Self {
            total_execution_power: initial_power,
            organelles: Vec::new(),
            next_organelle_id: 1,
        }
    }

    /// Origin encounters a free-living chain, engulfs it, strips its native 
    /// consensus layer, and converts it into an internal organelle.
    pub fn phagocytosis(&mut self, target_chain: FreeLivingChain) -> usize {
        let organelle = EndosymbioticOrganelle::from_chain(self.next_organelle_id, target_chain);
        
        // The organelle's execution power is permanently added to the Origin Cell
        self.total_execution_power += organelle.execution_power;
        
        let id = organelle.organelle_id;
        self.organelles.push(organelle);
        self.next_organelle_id += 1;
        
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbiogenesis_assimilation() {
        let mut origin = OriginCell::new(1000); // Base Origin compute
        
        // A free living rollup chain burning 300 units on consensus to get 500 units of execution
        let layer_2 = FreeLivingChain::new("Arbitrum", 300, 500);
        
        let organelle_id = origin.phagocytosis(layer_2);
        
        assert_eq!(organelle_id, 1);
        assert_eq!(origin.organelles.len(), 1);
        
        let organelle = &origin.organelles[0];
        assert_eq!(organelle.name, "Arbitrum_Mitochondrial_Subnet");
        assert_eq!(organelle.execution_power, 500); // Consensus overhead (300) is stripped
        
        // Origin's total power should now be 1000 + 500
        assert_eq!(origin.total_execution_power, 1500);
    }
}
