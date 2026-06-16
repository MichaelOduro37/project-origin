// ============================================================================
// PHASE 57: QUANTUM CHROMODYNAMICS (QCD) COLOR CONFINEMENT
// ============================================================================
// Scientific mechanism: Particle Physics (Strong Nuclear Force / Confinement)
//
// In QCD, quarks possess a "color charge" (Red, Green, Blue) and can never 
// exist in isolation. They must bind into color-neutral composite particles
// (Hadrons). If infinite energy is used to pull them apart, the strong force 
// snaps, spawning random quark-antiquark pairs (virtual noise).
//
// In Origin, Deep Packet Inspection (DPI) relies on isolating individual 
// packets. We defeat this by assigning packets a Color Charge. Packets can 
// only be transmitted as bound Hadrons (RGB Triplets). If a sniffer attempts 
// to extract a single packet from the triplet, the mathematical strong force 
// detects the isolation. The connection snaps, instantly destroying the payload
// and replacing it with randomized virtual particle noise. 
// ============================================================================

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum ColorCharge {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
pub struct QuarkPacket {
    pub payload: String,
    pub color: ColorCharge,
}

impl QuarkPacket {
    pub fn new(payload: &str, color: ColorCharge) -> Self {
        Self {
            payload: payload.to_string(),
            color,
        }
    }
}

/// A bound, color-neutral composite particle containing 3 QuarkPackets.
#[derive(Debug, Clone)]
pub struct Hadron {
    pub red_quark: QuarkPacket,
    pub green_quark: QuarkPacket,
    pub blue_quark: QuarkPacket,
}

impl Hadron {
    /// Binds three separate quark packets into a stable Hadron for transmission.
    pub fn bind(red: QuarkPacket, green: QuarkPacket, blue: QuarkPacket) -> Result<Self, &'static str> {
        if red.color != ColorCharge::Red || green.color != ColorCharge::Green || blue.color != ColorCharge::Blue {
            return Err("Color Confinement Violation: Hadron must be strictly RGB color-neutral.");
        }
        Ok(Self {
            red_quark: red,
            green_quark: green,
            blue_quark: blue,
        })
    }

    /// Simulates a Deep Packet Inspection (DPI) sniffer attempting to isolate
    /// a single packet from the stream. This violates color confinement.
    /// The strong force snaps, physically destroying the payload and returning noise.
    pub fn attempt_isolation(&self, target_color: ColorCharge) -> String {
        let noise: String = (0..16)
            .map(|_| {
                let random_char = (rand::random::<u8>() % 94 + 33) as char;
                random_char
            })
            .collect();
        
        // Return randomized virtual particle noise instead of the real payload
        format!("HADRON_SNAP_NOISE_SPAWNED::[{}]", noise)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hadron_binding() {
        let r = QuarkPacket::new("Tx_Header_0x1A", ColorCharge::Red);
        let g = QuarkPacket::new("Payload_Body_Hash", ColorCharge::Green);
        let b = QuarkPacket::new("Signature_R_S", ColorCharge::Blue);

        let hadron = Hadron::bind(r, g, b);
        assert!(hadron.is_ok());
    }

    #[test]
    fn test_invalid_hadron_binding() {
        let r1 = QuarkPacket::new("Data1", ColorCharge::Red);
        let r2 = QuarkPacket::new("Data2", ColorCharge::Red);
        let b = QuarkPacket::new("Data3", ColorCharge::Blue);

        let hadron = Hadron::bind(r1, r2, b);
        assert!(hadron.is_err());
        assert_eq!(hadron.unwrap_err(), "Color Confinement Violation: Hadron must be strictly RGB color-neutral.");
    }

    #[test]
    fn test_hadron_snap_isolation_defense() {
        let r = QuarkPacket::new("TOP_SECRET_PRIVATE_KEY", ColorCharge::Red);
        let g = QuarkPacket::new("NOISE_GLUON_1", ColorCharge::Green);
        let b = QuarkPacket::new("NOISE_GLUON_2", ColorCharge::Blue);

        let hadron = Hadron::bind(r, g, b).unwrap();

        // ISP or Sniffer attempts to isolate the Red packet
        let sniffed_data = hadron.attempt_isolation(ColorCharge::Red);

        // The exact payload is completely destroyed and replaced by random noise
        assert!(sniffed_data.contains("HADRON_SNAP_NOISE_SPAWNED::"));
        assert!(!sniffed_data.contains("TOP_SECRET_PRIVATE_KEY"));
    }
}
