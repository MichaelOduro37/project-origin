// ============================================================================
// PHASE 6: BIOMIMETIC SPIKING NEUROMORPHIC SYNCHRONIZATION (SNN)
// ============================================================================

use std::sync::{Mutex, OnceLock};

pub fn global_snn() -> &'static Mutex<LIFNeuron> {
    static SNN: OnceLock<Mutex<LIFNeuron>> = OnceLock::new();
    SNN.get_or_init(|| Mutex::new(LIFNeuron::new()))
}

/// Leaky Integrate-and-Fire (LIF) Neuron Model
/// Simulates biological membrane voltage to govern device sleep cycles.
#[derive(Debug, Clone, Copy)]
pub struct LIFNeuron {
    pub membrane_potential: f64,
    pub resting_potential: f64,
    pub threshold: f64,
    pub leak_rate: f64,
    pub last_fire_time: u128,
}

impl LIFNeuron {
    pub fn new() -> Self {
        LIFNeuron {
            membrane_potential: -70.0,  // Millivolts (mV) biological standard
            resting_potential: -70.0,
            threshold: -55.0,           // Action potential threshold
            leak_rate: 0.1,             // mV leaked per decay tick
            last_fire_time: 0,
        }
    }

    /// Receives incoming stimuli (e.g., network traffic).
    /// Returns `true` if an Action Potential (spike) was triggered.
    pub fn integrate(&mut self, stimulus: f64) -> bool {
        self.membrane_potential += stimulus;
        
        if self.membrane_potential >= self.threshold {
            self.fire()
        } else {
            false
        }
    }

    /// Biological leakage back to resting potential. Call this every clock tick.
    pub fn decay(&mut self) {
        if self.membrane_potential > self.resting_potential {
            self.membrane_potential -= self.leak_rate;
            if self.membrane_potential < self.resting_potential {
                self.membrane_potential = self.resting_potential;
            }
        }
    }

    /// Internal action potential trigger
    fn fire(&mut self) -> bool {
        self.membrane_potential = self.resting_potential; // Reset after spike
        self.last_fire_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        true
    }

    /// Converts the current biological state into a physical CPU/Radio polling interval.
    /// - Resting: Deep sleep (e.g., 5000ms) to save battery.
    /// - Excited/Recently fired: High performance (e.g., 50ms).
    pub fn get_polling_interval(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        // If fired within the last 5 seconds, keep the network wide awake (50ms)
        if now.saturating_sub(self.last_fire_time) < 5000 {
            return 50; 
        }

        // Map voltage [-70.0 to -55.0] to sleep interval [5000ms to 500ms]
        let range = self.threshold - self.resting_potential; // 15.0
        let excitation = (self.membrane_potential - self.resting_potential) / range; // 0.0 to 1.0
        
        let max_sleep = 5000.0;
        let min_sleep = 500.0;
        
        let sleep_ms = max_sleep - (excitation * (max_sleep - min_sleep));
        sleep_ms.max(50.0) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lif_integration_and_leak() {
        let mut neuron = LIFNeuron::new();
        assert_eq!(neuron.membrane_potential, -70.0);

        // Stimulus doesn't cross threshold
        let fired = neuron.integrate(10.0);
        assert_eq!(fired, false);
        assert_eq!(neuron.membrane_potential, -60.0);

        // Leak back
        neuron.decay();
        assert_eq!(neuron.membrane_potential, -60.1);

        // Stimulus crosses threshold
        let fired2 = neuron.integrate(10.0);
        assert_eq!(fired2, true);
        assert_eq!(neuron.membrane_potential, -70.0); // Resets after firing
    }
}
