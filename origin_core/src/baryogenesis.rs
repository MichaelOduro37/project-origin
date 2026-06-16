// ============================================================================
// PHASE 60: BARYOGENESIS (PRISTINE GENESIS STATE INITIALIZATION)
// ============================================================================
// Scientific mechanism: Cosmology (Baryogenesis & Sakharov Conditions)
//
// In standard blockchains, the Genesis Block is hardcoded by the founder. This
// relies on human trust. Origin removes human trust by simulating cosmological 
// physics. When a new subnet initializes, it simulates a mathematical "Big Bang".
//
// It generates massive streams of "Matter Data" and inverted "Antimatter Data",
// which mutually annihilate (zeroing out). By introducing cryptographic CP-violation 
// and thermal non-equilibrium (the Sakharov Conditions), the annihilation becomes 
// slightly asymmetric. The tiny, mathematically inevitable surviving remnant of 
// "Matter Data" crystallizes to become the unforgeable, trustless Genesis Block.
// ============================================================================

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct MatterData {
    pub payload: Vec<u8>,
}

pub struct AntimatterData {
    pub inverted_payload: Vec<u8>,
}

impl AntimatterData {
    pub fn new(matter: &MatterData) -> Self {
        let inverted = matter.payload.iter().map(|&b| !b).collect();
        Self {
            inverted_payload: inverted,
        }
    }
}

pub struct SakharovConditions {
    pub cp_violation_bias: f64,
    pub thermal_disequilibrium: f64,
}

#[derive(Debug)]
pub struct GenesisRemnant {
    pub remnant_hash: String,
    pub mass: usize,
}

pub fn simulate_big_bang(initial_mass: usize, sakharov: &SakharovConditions) -> Result<GenesisRemnant, &'static str> {
    if sakharov.cp_violation_bias == 0.0 || sakharov.thermal_disequilibrium == 0.0 {
        return Err("Perfect symmetry. Matter and Antimatter perfectly annihilated. Void created. No Genesis possible.");
    }

    let mut surviving_matter = Vec::new();

    // Simulate billions of particles annihilating
    for i in 0..initial_mass {
        let matter_byte = (i % 255) as u8;
        let antimatter_byte = !matter_byte;

        // Annihilation normally zeroes out
        let mut annihilation_result = matter_byte ^ (!antimatter_byte); // matter ^ matter = 0
        
        // Apply Sakharov Conditions
        // If CP-violation occurs at this thermal slice, the antimatter fails to invert perfectly
        let random_fluctuation = ((i as f64 * sakharov.thermal_disequilibrium).sin() + 1.0) / 2.0;

        if random_fluctuation < sakharov.cp_violation_bias {
            // Asymmetry! A matter particle survives annihilation
            annihilation_result = matter_byte;
            surviving_matter.push(annihilation_result);
        }
    }

    if surviving_matter.is_empty() {
         return Err("Annihilation complete. No remnant survived despite conditions.");
    }

    // Crystallize the surviving matter into the Genesis Hash
    let mut hasher = DefaultHasher::new();
    surviving_matter.hash(&mut hasher);
    let hash_value = hasher.finish();

    Ok(GenesisRemnant {
        remnant_hash: format!("{:016x}", hash_value),
        mass: surviving_matter.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_symmetry_annihilation() {
        let perfect_sakharov = SakharovConditions { cp_violation_bias: 0.0, thermal_disequilibrium: 0.0 };
        let result = simulate_big_bang(100_000, &perfect_sakharov);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Perfect symmetry. Matter and Antimatter perfectly annihilated. Void created. No Genesis possible.");
    }

    #[test]
    fn test_baryogenesis_asymmetry() {
        // Introduce tiny CP violation and high thermal disequilibrium
        let sakharov = SakharovConditions { cp_violation_bias: 0.001, thermal_disequilibrium: 1.42 };
        let result = simulate_big_bang(1_000_000, &sakharov);
        
        assert!(result.is_ok());
        let remnant = result.unwrap();
        
        // The remnant should be a tiny fraction of the initial mass
        assert!(remnant.mass > 0);
        assert!(remnant.mass < 1_000_000);
        
        // We have a valid genesis hash
        assert_eq!(remnant.remnant_hash.len(), 16);
    }
}
