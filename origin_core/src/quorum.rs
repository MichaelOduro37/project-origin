use std::time::Instant;
use std::sync::{Mutex, OnceLock};

const QUORUM_THRESHOLD: f64 = 100.0;
const DECAY_RATE: f64 = 0.5; // Concentration lost per second

pub struct QuorumSensor {
    pub concentration: f64,
    pub last_update: Instant,
    pub biofilm_mode: bool,
}

impl QuorumSensor {
    pub fn new() -> Self {
        Self {
            concentration: 0.0,
            last_update: Instant::now(),
            biofilm_mode: false,
        }
    }

    /// Update the natural decay of the autoinducer molecules in the environment.
    pub fn apply_decay(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        if elapsed > 0.0 {
            // Decay formula: C(t) = C(t-1) - gamma * dt
            self.concentration -= DECAY_RATE * elapsed;
            if self.concentration < 0.0 {
                self.concentration = 0.0;
            }
            self.last_update = now;
        }
    }

    /// Add new autoinducers to the local environment (from local generation or peer broadcasts).
    pub fn sense_autoinducer(&mut self, amount: f64) -> bool {
        self.apply_decay();
        self.concentration += amount;
        
        let was_biofilm = self.biofilm_mode;
        
        // Check threshold for quorum
        if self.concentration >= QUORUM_THRESHOLD {
            self.biofilm_mode = true;
        } else if self.concentration < QUORUM_THRESHOLD * 0.5 {
            // Hysteresis: only exit biofilm mode when concentration drops significantly
            self.biofilm_mode = false;
        }

        // Return true if biofilm state changed to true (triggered)
        self.biofilm_mode && !was_biofilm
    }

    /// Is the network currently locked down?
    pub fn is_biofilm_active(&self) -> bool {
        self.biofilm_mode
    }
}

pub fn global_quorum() -> &'static Mutex<QuorumSensor> {
    static QUORUM: OnceLock<Mutex<QuorumSensor>> = OnceLock::new();
    QUORUM.get_or_init(|| Mutex::new(QuorumSensor::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quorum_sensing_and_biofilm() {
        let mut sensor = QuorumSensor::new();
        
        assert_eq!(sensor.is_biofilm_active(), false);
        
        // Add some autoinducers, but not enough
        sensor.sense_autoinducer(50.0);
        assert_eq!(sensor.is_biofilm_active(), false);
        
        // Add enough to cross threshold
        let triggered = sensor.sense_autoinducer(60.0);
        assert_eq!(triggered, true);
        assert_eq!(sensor.is_biofilm_active(), true);
        
        // Test decay (simulated)
        sensor.concentration = 40.0; // Force drop below hysteresis threshold
        sensor.sense_autoinducer(0.0); // Trigger check
        assert_eq!(sensor.is_biofilm_active(), false);
    }
}
