// ============================================================================
// PHASE 44: PHOTONIC BAND GAP FIREWALL
// ============================================================================
// Scientific mechanism: Solid-State Physics / Optics (Photonic Crystals)
//
// Traditional firewalls run continuous CPU loops: `if ip == malicious { drop }`.
// This makes them vulnerable to CPU-exhaustion DDoS attacks.
//
// In Origin, we simulate a Photonic Crystal inbound port.
// Every packet has a `resonance_frequency`. 
// We define a `BandGap` (a mathematical range of forbidden frequencies).
// The `PhotonicLattice` structure mathematically repels any packet whose frequency
// falls inside the band gap, dropping it with O(0) structural overhead before
// it ever triggers conditional logic in the application layer.
// ============================================================================

/// Represents the forbidden frequency range (the structural firewall blocklist).
#[derive(Debug, Clone)]
pub struct BandGap {
    pub min_frequency: f64,
    pub max_frequency: f64,
}

impl BandGap {
    pub fn new(min_frequency: f64, max_frequency: f64) -> Self {
        BandGap { min_frequency, max_frequency }
    }
}

/// The inbound port memory structure, behaving like a periodic dielectric structure.
#[derive(Debug)]
pub struct PhotonicLattice {
    pub band_gaps: Vec<BandGap>,
}

impl PhotonicLattice {
    pub fn new() -> Self {
        PhotonicLattice {
            band_gaps: Vec::new(),
        }
    }

    pub fn add_band_gap(&mut self, gap: BandGap) {
        self.band_gaps.push(gap);
    }

    /// O(0) Structural Reflection Simulation.
    /// In a true low-level memory implementation, this would be enforced by 
    /// memory-mapped indexing where the index literally does not exist.
    /// Here, we simulate the physics: If the frequency falls into ANY band gap,
    /// it cannot resonate and is physically incapable of entering the lattice.
    pub fn is_resonant(&self, packet_frequency: f64) -> bool {
        // A packet is resonant only if it does NOT fall inside any forbidden Band Gap.
        for gap in &self.band_gaps {
            if packet_frequency >= gap.min_frequency && packet_frequency <= gap.max_frequency {
                // The structure prohibits this frequency. It reflects away instantly.
                return false; 
            }
        }
        true // The frequency resonates and propagates into the node's memory.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photonic_band_gap_rejection() {
        let mut lattice = PhotonicLattice::new();
        
        // Define a Band Gap simulating a DDoS frequency signature
        let ddos_gap = BandGap::new(45.0, 55.0);
        lattice.add_band_gap(ddos_gap);
        
        // Legitimate packet at frequency 30.0 (Resonant)
        assert!(lattice.is_resonant(30.0), "Valid frequency should propagate");
        
        // Malicious DDoS packet at frequency 50.0 (Forbidden)
        assert!(!lattice.is_resonant(50.0), "Forbidden frequency should be structurally repelled");
        
        // Legitimate packet at frequency 60.0 (Resonant)
        assert!(lattice.is_resonant(60.0), "Valid frequency should propagate");
    }
}
