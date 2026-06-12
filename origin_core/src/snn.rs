//! Biomimetic Spiking Neural Network (SNN) Integration
//! Implements a Leaky Integrate-and-Fire (LIF) model for predictive routing.
//! Rather than keeping mobile nodes awake constantly, nodes accumulate "packet pressure" (voltage).
//! Once a threshold is reached, an action potential is fired, waking the node predictively.

pub struct IntegrateAndFireNode {
    pub node_id: String,
    voltage: f64,
    threshold: f64,
    leak_rate: f64,
    is_awake: bool,
    last_tick: u64,
}

impl IntegrateAndFireNode {
    pub fn new(node_id: String, threshold: f64, leak_rate: f64) -> Self {
        Self {
            node_id,
            voltage: 0.0,
            threshold,
            leak_rate,
            is_awake: false,
            last_tick: 0,
        }
    }

    /// Simulate the passage of time (decay)
    pub fn tick_decay(&mut self, current_tick: u64) {
        let elapsed = current_tick.saturating_sub(self.last_tick);
        if elapsed > 0 {
            // Leak voltage over time
            self.voltage *= (1.0 - self.leak_rate).powi(elapsed as i32);
            if self.voltage < 0.001 {
                self.voltage = 0.0;
            }
            self.last_tick = current_tick;
        }
    }

    /// Accumulate packet pressure from nearby topology
    pub fn accumulate_pressure(&mut self, pressure: f64) -> bool {
        self.voltage += pressure;
        
        // Action Potential Threshold Crossed
        if self.voltage >= self.threshold {
            self.is_awake = true;
            self.voltage = 0.0; // Reset after spike (Refractory period abstracted)
            true
        } else {
            false
        }
    }

    pub fn is_primed(&self) -> bool {
        self.is_awake
    }

    pub fn sleep(&mut self) {
        self.is_awake = false;
        self.voltage = 0.0;
    }
}
