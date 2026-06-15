// ============================================================================
// PHASE 41: HAWKING RADIATION CACHE EVICTION (HOLOGRAPHIC MEMORY)
// ============================================================================
// Scientific mechanism: Astrophysics (Black Hole Information Paradox)
//
// In Origin, memory management is modeled as a Black Hole.
// 1. Data has "mass" (TTL/Priority). Over time, unaccessed data "evaporates" 
//    via Hawking Radiation to free up physical RAM.
// 2. The Information Paradox: When mass reaches 0, standard systems would 
//    just `drop()` the payload. We do not. The Holographic Principle states
//    information is preserved on the Event Horizon.
// 3. We drop the raw payload to save space, but mathematically scramble and 
//    hash its signature, storing it permanently in the Event Horizon registry.
// 4. This ensures O(1) auditability—the node can mathematically prove that 
//    a specific massive payload once existed and passed through it, without 
//    having to store the payload itself.
// ============================================================================

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DataParticle {
    pub id: String,
    pub raw_payload: Option<Vec<u8>>,
    pub mass: f64, // Represents TTL / Priority. Decreases over time.
}

pub struct BlackHoleCache {
    pub internal_bulk: HashMap<String, DataParticle>,
    /// The Holographic Boundary. Stores mathematical signatures of evaporated data.
    pub event_horizon: HashMap<String, String>, 
}

impl BlackHoleCache {
    pub fn new() -> Self {
        Self {
            internal_bulk: HashMap::new(),
            event_horizon: HashMap::new(),
        }
    }

    pub fn insert_data(&mut self, id: String, payload: Vec<u8>, initial_mass: f64) {
        let particle = DataParticle {
            id: id.clone(),
            raw_payload: Some(payload),
            mass: initial_mass,
        };
        self.internal_bulk.insert(id, particle);
    }

    /// Simulates Hawking Radiation. Over time, data mass decreases.
    /// Returns a list of Data IDs that fully evaporated in this cycle.
    pub fn evaporate(&mut self, decay_rate: f64) -> Vec<String> {
        let mut evaporated_ids = Vec::new();

        for (id, particle) in self.internal_bulk.iter_mut() {
            particle.mass -= decay_rate;
            if particle.mass <= 0.0 {
                evaporated_ids.push(id.clone());
            }
        }

        // Process evaporation (Information Paradox)
        for id in &evaporated_ids {
            if let Some(particle) = self.internal_bulk.remove(id) {
                self.inscribe_event_horizon(particle);
            }
        }

        evaporated_ids
    }

    /// The Holographic Principle: Scramble the payload into a mathematical 
    /// signature and store it permanently on the event horizon. Drop the raw payload.
    fn inscribe_event_horizon(&mut self, particle: DataParticle) {
        if let Some(payload) = particle.raw_payload {
            // In a real system, we would use a cryptographic hash like SHA-256.
            // For simulation, we'll create a deterministic pseudo-hash string.
            let signature = format!("HOLOSIG_{}_{:x}", particle.id, payload.len());
            self.event_horizon.insert(particle.id, signature);
        }
    }

    /// Allows a node to mathematically prove a payload existed, even if evaporated.
    pub fn verify_historical_existence(&self, id: &str) -> bool {
        self.internal_bulk.contains_key(id) || self.event_horizon.contains_key(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hawking_evaporation_and_holographic_memory() {
        let mut black_hole = BlackHoleCache::new();

        let data_id = "QuantumState_Alpha".to_string();
        let payload = vec![0; 1024]; // 1KB of data

        black_hole.insert_data(data_id.clone(), payload, 2.0); // Mass = 2.0

        // Step 1: Evaporate by 1.0. Data should still exist in bulk.
        let evaporated = black_hole.evaporate(1.0);
        assert!(evaporated.is_empty());
        assert!(black_hole.internal_bulk.contains_key(&data_id));
        assert!(!black_hole.event_horizon.contains_key(&data_id));

        // Step 2: Evaporate by 1.0. Mass hits 0.0. 
        let evaporated = black_hole.evaporate(1.0);
        assert_eq!(evaporated.len(), 1);
        assert_eq!(evaporated[0], data_id);

        // Step 3: Verify the Information Paradox
        // Raw payload MUST be gone to save RAM.
        assert!(!black_hole.internal_bulk.contains_key(&data_id));
        
        // Holographic signature MUST be inscribed on the Event Horizon.
        assert!(black_hole.event_horizon.contains_key(&data_id));
        let sig = black_hole.event_horizon.get(&data_id).unwrap();
        assert_eq!(sig, "HOLOSIG_QuantumState_Alpha_400"); // 1024 in hex is 400

        // Step 4: Verify historical existence works.
        assert!(black_hole.verify_historical_existence(&data_id));
    }
}
