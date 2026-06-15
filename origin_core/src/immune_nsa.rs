// ============================================================================
// PHASE 24: ARTIFICIAL IMMUNE SYSTEM (NEGATIVE SELECTION ALGORITHM)
// ============================================================================
// Scientific mechanism: The biological thymus gland matures T-cells by destroying
// any T-cell that attacks the body's own "self" tissue. The surviving T-cells
// are mathematically guaranteed to only attack foreign "non-self" pathogens.
//
// Origin implements the Negative Selection Algorithm (NSA) to achieve Zero-Day
// anomaly detection without needing prior virus signatures.
// 
// 1. We define a "Self" profile (e.g., normal telemetry metrics).
// 2. The Thymus generates randomized Detectors (T-Cells).
// 3. Detectors that match the "Self" profile are censored/destroyed.
// 4. Mature detectors monitor the Swarm. If they trigger, it's a Zero-Day anomaly.
// ============================================================================

pub struct TCellDetector {
    pub id: String,
    pub profile_coordinates: Vec<f64>,
    pub detection_radius: f64,
}

pub struct Thymus {
    pub self_profile: Vec<f64>,
    pub tolerance_radius: f64,
}

impl Thymus {
    pub fn new(self_profile: Vec<f64>, tolerance_radius: f64) -> Self {
        Self {
            self_profile,
            tolerance_radius,
        }
    }

    /// Measures the Euclidean distance between two telemetry profiles
    fn distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Generates randomized T-Cells. Censors any that react to the "Self" profile.
    /// Returns a highly mature, non-self-reactive array of Zero-Day detectors.
    pub fn generate_mature_detectors(&self, num_candidates: usize) -> Vec<TCellDetector> {
        let mut mature_array = Vec::new();

        for i in 0..num_candidates {
            // Generate a random detector profile (e.g., randomly skewed metrics)
            let random_coords: Vec<f64> = self.self_profile.iter()
                .map(|_| rand::random::<f64>() * 100.0) // Assume metrics are 0-100
                .collect();

            // Check against "Self" (Negative Selection)
            let dist_to_self = Self::distance(&random_coords, &self.self_profile);
            
            if dist_to_self > self.tolerance_radius {
                // T-Cell did NOT attack self! It has matured.
                mature_array.push(TCellDetector {
                    id: format!("TCELL-{}-{}", std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos() % 1000, i),
                    profile_coordinates: random_coords,
                    detection_radius: self.tolerance_radius * 0.8, // Slightly tighter radius for detection
                });
            }
            // Else: T-Cell was self-reactive and is inherently destroyed (censored)
        }

        mature_array
    }
}

/// The immune response function deployed to the Swarm.
pub fn scan_for_anomalies(incoming_telemetry: &[f64], mature_detectors: &[TCellDetector]) -> Option<(String, f64)> {
    for detector in mature_detectors {
        let dist = Thymus::distance(incoming_telemetry, &detector.profile_coordinates);
        if dist <= detector.detection_radius {
            // A mature T-Cell reacted! By definition, this is a Non-Self anomaly.
            return Some((detector.id.clone(), dist));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_selection_algorithm() {
        // Define a "Self" node operating normally (CPU 40%, Temp 45C, RAM 50%)
        let self_profile = vec![40.0, 45.0, 50.0];
        let thymus = Thymus::new(self_profile.clone(), 20.0);

        // Generate candidate detectors
        let mature_tcells = thymus.generate_mature_detectors(1000);

        // Prove that NO mature T-Cell reacts to the "Self" profile
        let self_scan = scan_for_anomalies(&self_profile, &mature_tcells);
        assert!(self_scan.is_none(), "A mature T-Cell attacked the Self profile! Autoimmune failure.");

        // Simulate a Zero-Day exploit drastically altering telemetry (CPU 99%, Temp 85C, RAM 95%)
        let anomaly_profile = vec![99.0, 85.0, 95.0];
        let anomaly_scan = scan_for_anomalies(&anomaly_profile, &mature_tcells);
        
        // Given enough candidates (1000), it's highly probable an anomaly is caught
        assert!(anomaly_scan.is_some(), "Mature T-Cells failed to detect the Zero-Day anomaly.");
    }
}
