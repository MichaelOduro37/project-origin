// ============================================================================
// PHASE 28: CONSTRUCTAL LAW ROUTING OPTIMIZATION
// ============================================================================
// Scientific mechanism: Discovered by Adrian Bejan, the Constructal Law
// states that macroscopic flow systems evolve configurations that provide
// easier access to currents.
//
// Origin uses this to physically morph the network topology. High-traffic 
// channels dynamically "thicken" (become high-capacity arterial trunks), 
// and low-flow channels "thin" into capillaries. This organically scales
// the Swarm into a highly efficient vascular system.
// ============================================================================

#[derive(Debug, Clone)]
pub struct FlowChannel {
    pub id: String,
    pub capacity: f64,    // Physical bandwidth/priority allocation
    pub flow_volume: f64, // Recent traffic volume (packets/sec)
}

impl FlowChannel {
    pub fn new(id: &str, capacity: f64) -> Self {
        Self {
            id: id.to_string(),
            capacity,
            flow_volume: 0.0,
        }
    }

    /// Calculate flow resistance. Higher capacity = lower resistance.
    pub fn resistance(&self) -> f64 {
        if self.capacity <= 0.0 {
            f64::MAX
        } else {
            1.0 / self.capacity
        }
    }

    /// Morphs the physical channel based on the Constructal gradient.
    /// Returns the capacity delta.
    pub fn morph_constructal(&mut self) -> f64 {
        // Constructal adaptation rate
        let alpha = 0.1;
        // Natural decay rate for unused channels
        let decay = 0.02;

        let target_capacity = self.flow_volume * 1.5; // Ideal vascular thickness for this flow
        let old_capacity = self.capacity;

        if target_capacity > self.capacity {
            // High flow: Thicken the channel (arterial growth)
            self.capacity += (target_capacity - self.capacity) * alpha;
        } else {
            // Low flow: Thin the channel (capillary decay)
            self.capacity -= self.capacity * decay;
        }

        // Enforce absolute minimum capillary size
        if self.capacity < 1.0 {
            self.capacity = 1.0;
        }

        self.capacity - old_capacity
    }
}

pub struct ConstructalEngine {
    pub channels: Vec<FlowChannel>,
}

impl ConstructalEngine {
    pub fn new(channels: Vec<FlowChannel>) -> Self {
        Self { channels }
    }

    /// Evolves the entire network topology according to the Constructal Law.
    /// Returns a list of channels that evolved into massive Arterial Trunks.
    pub fn optimize_vascular_flow(&mut self) -> Vec<(String, f64)> {
        let mut new_trunks = Vec::new();
        let trunk_threshold = 100.0;

        for channel in &mut self.channels {
            let was_trunk = channel.capacity >= trunk_threshold;
            let delta = channel.morph_constructal();
            let is_trunk = channel.capacity >= trunk_threshold;

            // Trigger an event if it just crossed the threshold into a massive trunk
            // OR if it's already a trunk and grew significantly
            if (is_trunk && !was_trunk) || (is_trunk && delta > 10.0) {
                new_trunks.push((channel.id.clone(), channel.capacity));
            }
        }

        new_trunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructal_evolution() {
        let mut c1 = FlowChannel::new("link_A", 10.0);
        let mut c2 = FlowChannel::new("link_B", 50.0);

        // c1 has massive flow, c2 has zero flow
        c1.flow_volume = 200.0;
        c2.flow_volume = 0.0;

        let mut engine = ConstructalEngine::new(vec![c1, c2]);
        
        // Evolve multiple generations
        for _ in 0..10 {
            engine.optimize_vascular_flow();
        }

        // c1 should have grown significantly (thickened into a trunk)
        assert!(engine.channels[0].capacity > 30.0, "Constructal Law failed to thicken high-flow channel");
        // c2 should have decayed (thinned into a capillary)
        assert!(engine.channels[1].capacity < 50.0, "Constructal Law failed to thin low-flow channel");
        
        // Resistance of the trunk should be much lower than the capillary
        assert!(engine.channels[0].resistance() < engine.channels[1].resistance(), "Arterial trunk resistance must be lower than capillary");
    }
}
