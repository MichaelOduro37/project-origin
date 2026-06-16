// ============================================================================
// PHASE 56: SONOLUMINESCENCE (CAVITATION BURST TRANSMISSION)
// ============================================================================
// Scientific mechanism: Fluid Dynamics (Acoustic Cavitation)
//
// Sonoluminescence ("Star in a Jar") happens when a small gas bubble in a 
// liquid is subjected to intense acoustic pressure. The bubble collapses so 
// violently that it emits an instantaneous burst of light and extreme heat.
//
// In Origin, extreme network congestion creates a dense "fluid medium" where 
// packets get stuck in queues. To transmit highly critical consensus data 
// during this gridlock, the data is formed into a mathematical `CavitationBubble`.
// An acoustic pressure wave is applied, causing the bubble to violently collapse.
// This triggers a Sonoluminescent Burst, an instantaneous data transfer that 
// punches straight through the congestion, bypassing standard queue logic.
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    pub id: usize,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct CavitationBubble {
    pub critical_payload: String,
}

#[derive(Debug, PartialEq)]
pub struct SonoluminescentBurst {
    pub emitted_payload: String,
    pub burst_temperature: f64, // Mathematical intensity of the burst
}

pub struct PacketQueue {
    pub standard_fluid_queue: Vec<Packet>,
    pub cavitation_bubble: Option<CavitationBubble>,
}

impl PacketQueue {
    pub fn new() -> Self {
        Self {
            standard_fluid_queue: Vec::new(),
            cavitation_bubble: None,
        }
    }

    pub fn enqueue_standard(&mut self, packet: Packet) {
        self.standard_fluid_queue.push(packet);
    }

    pub fn form_cavitation_bubble(&mut self, critical_payload: &str) {
        self.cavitation_bubble = Some(CavitationBubble {
            critical_payload: critical_payload.to_string(),
        });
    }

    /// Applies intense acoustic pressure to the queue.
    /// If the pressure exceeds a critical threshold and a bubble exists,
    /// it violently collapses, bypassing the entire queue and emitting a burst.
    pub fn apply_acoustic_pressure(&mut self, network_stress_pa: f64) -> Option<SonoluminescentBurst> {
        let critical_pressure_threshold = 100_000.0; // Mathematical Pascal threshold

        if network_stress_pa >= critical_pressure_threshold {
            if let Some(bubble) = self.cavitation_bubble.take() {
                // The bubble violently collapses, emitting the Sonoluminescent Burst
                return Some(SonoluminescentBurst {
                    emitted_payload: bubble.critical_payload,
                    burst_temperature: network_stress_pa * 0.5, // Extreme mathematical temperature
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sonoluminescent_burst() {
        let mut queue = PacketQueue::new();
        
        // Simulate massive network congestion (queue filled with standard packets)
        for i in 0..10_000 {
            queue.enqueue_standard(Packet { id: i, payload: "Standard Traffic".to_string() });
        }

        // Form a cavitation bubble for an emergency payload
        queue.form_cavitation_bubble("EMERGENCY_ROOT_CONSENSUS_OVERRIDE");

        // Apply critical acoustic pressure
        let burst_result = queue.apply_acoustic_pressure(150_000.0);
        
        assert!(burst_result.is_some());
        let burst = burst_result.unwrap();
        
        // The critical payload bypassed all 10,000 standard packets
        assert_eq!(burst.emitted_payload, "EMERGENCY_ROOT_CONSENSUS_OVERRIDE");
        assert_eq!(burst.burst_temperature, 75_000.0);
        
        // Bubble is destroyed after collapse
        assert!(queue.cavitation_bubble.is_none());
    }

    #[test]
    fn test_insufficient_pressure_no_burst() {
        let mut queue = PacketQueue::new();
        queue.form_cavitation_bubble("CRITICAL_DATA");

        // Apply weak acoustic pressure
        let burst_result = queue.apply_acoustic_pressure(50_000.0);
        
        // No collapse, bubble remains
        assert!(burst_result.is_none());
        assert!(queue.cavitation_bubble.is_some());
    }
}
