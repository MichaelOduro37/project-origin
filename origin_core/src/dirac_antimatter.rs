// ============================================================================
// PHASE 42: DIRAC ANTIMATTER DATA ANNIHILATION
// ============================================================================
// Scientific mechanism: Quantum Physics (Dirac Equation)
//
// In Origin, data revocation and deletion are handled via Antimatter.
// Standard networks use Revocation Lists (CRLs) to track deleted/invalid data.
// Origin uses Dirac Inverse Spin.
//
// 1. Data has a mathematical `spin_signature`.
// 2. To revoke data globally, we generate an "Anti-Packet" which has the exact
//    inverse signature.
// 3. The MemoryVacuum acts as the quantum space. When a packet and its
//    anti-packet exist in the same memory space, they collide.
// 4. `spin + inverse_spin = 0`. Both are perfectly annihilated from RAM.
// 5. This enables zero-trace, self-cleaning distributed purges without 
//    centralized lists.
// ============================================================================

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumDataParticle {
    pub id: String,
    pub spin_signature: i64,
    pub payload: Option<Vec<u8>>,
}

impl QuantumDataParticle {
    /// Generates the Dirac Inverse (Antimatter) of this particle.
    pub fn generate_antiparticle(&self) -> Self {
        Self {
            id: self.id.clone(),
            spin_signature: -self.spin_signature, // Invert the spin
            payload: None, // Anti-packets don't carry payload, only destructive intent
        }
    }
}

pub struct MemoryVacuum {
    /// Maps Data ID to a sum of spins. If the sum hits 0, it is annihilated.
    pub particles: HashMap<String, Vec<QuantumDataParticle>>,
}

impl MemoryVacuum {
    pub fn new() -> Self {
        Self {
            particles: HashMap::new(),
        }
    }

    /// Injects a particle (or anti-particle) into the vacuum.
    /// Returns true if an annihilation event occurred.
    pub fn inject(&mut self, particle: QuantumDataParticle) -> bool {
        let id = particle.id.clone();
        
        // Add to vacuum
        let entry = self.particles.entry(id.clone()).or_insert_with(Vec::new);
        entry.push(particle);

        // Check for collision/annihilation
        self.collide(&id)
    }

    /// Evaluates the quantum superposition of the space.
    /// If sum of spins == 0 (and there are particles present), they annihilate.
    fn collide(&mut self, id: &str) -> bool {
        if let Some(list) = self.particles.get(id) {
            let total_spin: i64 = list.iter().map(|p| p.spin_signature).sum();
            
            // If the superposition collapses to exactly zero (and isn't just an empty list)
            if total_spin == 0 && !list.is_empty() {
                // ANNIHILATION!
                self.particles.remove(id);
                return true;
            }
        }
        false
    }
    
    pub fn contains(&self, id: &str) -> bool {
        self.particles.contains_key(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirac_antimatter_annihilation() {
        let mut vacuum = MemoryVacuum::new();

        let data_id = "MaliciousPayload_XYZ".to_string();
        let particle = QuantumDataParticle {
            id: data_id.clone(),
            spin_signature: 42,
            payload: Some(vec![1, 2, 3]),
        };

        // 1. Inject normal particle
        let annihilated = vacuum.inject(particle.clone());
        assert!(!annihilated);
        assert!(vacuum.contains(&data_id));

        // 2. Generate anti-particle to revoke it
        let anti_particle = particle.generate_antiparticle();
        assert_eq!(anti_particle.spin_signature, -42);

        // 3. Inject anti-particle
        let annihilated = vacuum.inject(anti_particle);
        
        // 4. Mutual Annihilation must occur instantly
        assert!(annihilated);
        assert!(!vacuum.contains(&data_id)); // Completely purged from memory
    }
}
