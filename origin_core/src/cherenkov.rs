// ============================================================================
// PHASE 54: CHERENKOV RADIATION (RELATIVISTIC ANOMALY DETECTION)
// ============================================================================
// Scientific mechanism: Particle Physics (Electromagnetic Shockwaves)
//
// Cherenkov Radiation is an optical sonic boom. When a high-energy particle
// travels through a dielectric medium faster than the local phase velocity of
// light, it emits a violent electromagnetic shockwave (a blue glow).
//
// In Origin, networks are vulnerable to velocity-based attacks (DDoS, HFT bots)
// that spam packets faster than the protocol is designed to process.
// We define a strict `NETWORK_PHASE_VELOCITY_LIMIT` based on Minkowski causality.
// If anomalous traffic exceeds this speed, it generates a "Cherenkov Shockwave",
// instantly flagging and rejecting the superluminal packets.
// ============================================================================

/// The maximum allowable transmission rate (packets/second or equivalent units)
/// representing the "speed of light" through the local Origin medium.
pub const NETWORK_PHASE_VELOCITY_LIMIT: f64 = 10_000.0;

#[derive(Debug, PartialEq)]
pub struct CherenkovShockwave {
    pub anomaly_signature: String,
    pub excess_velocity: f64,
}

pub struct CherenkovDetector {
    pub local_medium_density: f64, // Can be used to dynamically adjust the phase velocity limit
}

impl CherenkovDetector {
    pub fn new() -> Self {
        Self {
            local_medium_density: 1.0, // Standard vacuum equivalent
        }
    }

    /// Evaluates incoming packet velocity against the relativistic limit.
    /// If the velocity exceeds the phase limit, a Cherenkov Shockwave is emitted.
    pub fn detect_superluminal_anomaly(&self, packet_velocity: f64) -> Result<(), CherenkovShockwave> {
        // The phase velocity limit adjusted by the local medium density
        let local_speed_of_light = NETWORK_PHASE_VELOCITY_LIMIT / self.local_medium_density;

        if packet_velocity > local_speed_of_light {
            let excess_velocity = packet_velocity - local_speed_of_light;
            
            return Err(CherenkovShockwave {
                anomaly_signature: format!("SUPERLUMINAL_BREACH_DETECTED_EXCESS_{:.2}", excess_velocity),
                excess_velocity,
            });
        }

        // Velocity is subluminal; traffic is legitimate.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subluminal_legitimate_traffic() {
        let detector = CherenkovDetector::new();
        // Legitimate traffic: 5,000 velocity units (Below the 10,000 limit)
        let result = detector.detect_superluminal_anomaly(5000.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_superluminal_ddos_shockwave() {
        let detector = CherenkovDetector::new();
        // DDoS Attack: 15,000 velocity units (Breaks the 10,000 limit)
        let result = detector.detect_superluminal_anomaly(15000.0);
        
        assert!(result.is_err());
        let shockwave = result.unwrap_err();
        assert_eq!(shockwave.excess_velocity, 5000.0);
        assert!(shockwave.anomaly_signature.contains("SUPERLUMINAL_BREACH"));
    }
}
