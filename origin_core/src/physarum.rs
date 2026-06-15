// ============================================================================
// PHASE 9: PHYSARUM POLYCEPHALUM (SLIME MOLD) FORAGING MODEL
// ============================================================================

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Represents the biological attractant gradient emitted by a node 
/// hunting for scattered Holographic shards.
#[derive(Debug, Clone)]
pub struct SlimeGradient {
    pub file_id: String,
    pub intensity: f64,
}

/// Represents a "tube" (network path) to a peer. 
/// In Physarum biology, tubes thicken when they carry flow (data), 
/// and decay when they don't, naturally isolating the shortest path.
#[derive(Debug, Clone)]
pub struct PhysarumTube {
    pub peer_ip: String,
    pub thickness: f64, // Conductivity / Bandwidth
}

pub struct PhysarumNetwork {
    pub tubes: HashMap<String, PhysarumTube>,
    pub decay_rate: f64, // Gamma in the Physarum equation
}

impl PhysarumNetwork {
    pub fn new() -> Self {
        Self {
            tubes: HashMap::new(),
            decay_rate: 0.05, // 5% decay per tick
        }
    }

    /// Update the thickness of a tube based on the flux (data) it carried.
    /// Equation: dD/dt = |Q| - gamma * D
    pub fn stimulate_tube(&mut self, peer_ip: &str, flux: f64) {
        let tube = self.tubes.entry(peer_ip.to_string()).or_insert(PhysarumTube {
            peer_ip: peer_ip.to_string(),
            thickness: 1.0, // Initial base thickness
        });

        // Biological thickening
        tube.thickness += flux;
        
        // Prevent infinite thickening
        if tube.thickness > 100.0 {
            tube.thickness = 100.0;
        }
    }

    /// Decay all tubes. Tubes with no flow will wither away, pruning the network.
    pub fn decay_all(&mut self) {
        for tube in self.tubes.values_mut() {
            tube.thickness -= self.decay_rate * tube.thickness;
            if tube.thickness < 0.1 {
                tube.thickness = 0.1; // Baseline capillary
            }
        }
    }

    /// Get the optimal peer (thickest tube) to route a shard request through.
    pub fn get_optimal_path(&self) -> Option<String> {
        let mut best_peer = None;
        let mut max_thickness = 0.0;

        for (ip, tube) in &self.tubes {
            if tube.thickness > max_thickness {
                max_thickness = tube.thickness;
                best_peer = Some(ip.clone());
            }
        }
        best_peer
    }
}

pub fn global_physarum() -> &'static Mutex<PhysarumNetwork> {
    static PHYSARUM: OnceLock<Mutex<PhysarumNetwork>> = OnceLock::new();
    PHYSARUM.get_or_init(|| Mutex::new(PhysarumNetwork::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physarum_thickening_and_decay() {
        let mut network = PhysarumNetwork::new();
        
        // Two peers: A and B
        network.stimulate_tube("PeerA", 10.0);
        network.stimulate_tube("PeerB", 2.0);
        
        assert!(network.tubes.get("PeerA").unwrap().thickness > network.tubes.get("PeerB").unwrap().thickness);
        
        // Optimal path should be PeerA
        assert_eq!(network.get_optimal_path(), Some("PeerA".to_string()));
        
        // Decay over time
        let initial_a = network.tubes.get("PeerA").unwrap().thickness;
        network.decay_all();
        let decayed_a = network.tubes.get("PeerA").unwrap().thickness;
        
        assert!(decayed_a < initial_a);
    }
}
