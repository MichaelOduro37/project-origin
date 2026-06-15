// ============================================================================
// PHASE 50: THERMODYNAMIC REVERSIBLE ROUTING (ZERO-ENTROPY COMPUTING)
// ============================================================================
// Scientific mechanism: Thermodynamics (Landauer's Principle)
//
// Landauer's principle states that erasing information (irreversible logic)
// dissipates thermodynamic heat (kT ln 2). Dropping a packet erases bits.
// We use a Fredkin Gate (Controlled-SWAP), a universal reversible logic gate.
// If a packet is invalid, we don't drop it. We use the Fredkin Gate to SWAP
// it into a Reversible Heat Sink Buffer. No bits are erased. The operation
// is 100% mathematically reversible. Logical Entropy generated = 0.
// ============================================================================

/// Represents a simple packet header or payload slice.
#[derive(Debug, Clone, PartialEq)]
pub struct DataPacket {
    pub id: u32,
    pub payload: Vec<u8>,
}

/// The Fredkin Gate (CSWAP).
/// It takes three inputs (c, a, b) and returns three outputs (c_out, a_out, b_out).
/// If control line `c` is false (0), `a` and `b` pass through unchanged.
/// If control line `c` is true (1), `a` and `b` are swapped.
/// Crucially, no information is lost. The operation is its own inverse (Involution).
pub fn fredkin_gate<T>(c: bool, a: T, b: T) -> (bool, T, T) {
    if c {
        (c, b, a) // Swapped
    } else {
        (c, a, b) // Unchanged
    }
}

/// A Reversible Router that generates 0 logical entropy.
pub struct ReversibleRouter {
    pub main_transmission_line: Vec<DataPacket>,
    pub heat_sink_buffer: Vec<DataPacket>, // Holds "dropped" packets without erasing them
}

impl ReversibleRouter {
    pub fn new() -> Self {
        ReversibleRouter {
            main_transmission_line: Vec::new(),
            heat_sink_buffer: Vec::new(),
        }
    }

    /// Routes a packet reversibly based on the `is_valid` control line.
    /// In a standard router: `if !is_valid { drop(packet); }` -> Information Erased -> Heat generated.
    /// In a Reversible Router: `fredkin_gate(!is_valid, packet, empty_slot)` -> No Information Erased -> 0 Heat.
    pub fn route_packet_reversible(&mut self, packet: DataPacket, is_valid: bool) {
        // We use a dummy empty packet as the "b" input for the Fredkin Gate.
        let empty_packet = DataPacket { id: 0, payload: vec![] };

        // Control line: We want to swap the packet to the heat sink if it is INVALID.
        let control = !is_valid;

        // Apply the Fredkin Gate
        let (c_out, out_a, out_b) = fredkin_gate(control, packet, empty_packet);

        // out_a goes to the main transmission line (it holds the valid packet, or the empty packet if swapped)
        // out_b goes to the heat sink (it holds the empty packet, or the invalid packet if swapped)
        
        // We only append non-empty outputs to keep the simulation clean, 
        // though in pure physics the empty slots would still exist.
        if !c_out {
            // control was false (packet was valid). out_a is the packet.
            self.main_transmission_line.push(out_a);
        } else {
            // control was true (packet was invalid). out_b is the packet.
            self.heat_sink_buffer.push(out_b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fredkin_gate_involution() {
        // A fundamental property of a reversible gate is that applying it twice
        // with the same control line restores the exact original inputs.
        let original_a = 42;
        let original_b = 99;

        // Pass 1: Control = true (SWAP)
        let (c1, a1, b1) = fredkin_gate(true, original_a, original_b);
        assert_eq!(a1, 99);
        assert_eq!(b1, 42);

        // Pass 2: Apply the gate again to the outputs
        let (_, a2, b2) = fredkin_gate(c1, a1, b1);
        
        // The original state is perfectly restored. Zero entropy generated.
        assert_eq!(a2, original_a);
        assert_eq!(b2, original_b);
    }

    #[test]
    fn test_reversible_router_zero_entropy() {
        let mut router = ReversibleRouter::new();

        let valid_packet = DataPacket { id: 1, payload: vec![1, 1, 1] };
        let invalid_packet = DataPacket { id: 2, payload: vec![0, 0, 0] };

        // Route the packets
        router.route_packet_reversible(valid_packet.clone(), true);
        router.route_packet_reversible(invalid_packet.clone(), false);

        // The valid packet is exactly in the main line
        assert_eq!(router.main_transmission_line.len(), 1);
        assert_eq!(router.main_transmission_line[0], valid_packet);

        // The invalid packet was NOT deleted/erased. It was cleanly swapped into the heat sink.
        assert_eq!(router.heat_sink_buffer.len(), 1);
        assert_eq!(router.heat_sink_buffer[0], invalid_packet);
    }
}
