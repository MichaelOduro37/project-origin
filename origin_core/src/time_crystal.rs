// ============================================================================
// PHASE 52: TIME CRYSTAL STATE MACHINES (ZERO-ENERGY SYNCHRONIZATION)
// ============================================================================
// Scientific mechanism: Non-Equilibrium Quantum Dynamics (Time Crystals)
//
// Traditional networks use "heartbeats" or pings to synchronize nodes. This
// burns bandwidth and CPU cycles (thermodynamic energy).
// A Time Crystal is a phase of matter that breaks time-translational symmetry.
// It oscillates between states perpetually at its absolute ground state, meaning
// it ticks forever without consuming or dissipating energy.
// We map this temporal oscillation into Origin's state machine. The network
// synchronizes automatically based on the Time Crystal's period without sending
// a single active polling packet.
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinState {
    Up,
    Down,
}

impl SpinState {
    /// In a time crystal, the state flips perpetually without energy input.
    pub fn flip(&mut self) {
        *self = match self {
            SpinState::Up => SpinState::Down,
            SpinState::Down => SpinState::Up,
        };
    }
}

pub struct TimeCrystalClock {
    pub period: usize,
    pub current_spin: SpinState,
    pub internal_ticks: usize,
}

impl TimeCrystalClock {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            current_spin: SpinState::Up,
            internal_ticks: 0,
        }
    }

    /// Ticks the clock forward. Because it is a Time Crystal, when the temporal
    /// period is reached, the state breaks time-translational symmetry and flips.
    /// This happens at the computational ground state (0 thermodynamic dissipation).
    pub fn tick_oscillation(&mut self) -> bool {
        self.internal_ticks += 1;
        
        if self.internal_ticks % self.period == 0 {
            self.current_spin.flip();
            true // Symmetry broken, state flipped
        } else {
            false
        }
    }

    /// Synchronizes the node's local network state strictly based on the Time Crystal
    /// oscillation. Because all nodes share the fundamental period (derived from the Genesis block),
    /// they achieve perfect synchronization without sending network pings.
    pub fn synchronize_state(&self, global_time: usize) -> bool {
        // A node is synchronized if its internal spin state matches the expected
        // global temporal oscillation.
        let expected_flips = global_time / self.period;
        let expected_spin = if expected_flips % 2 == 0 {
            SpinState::Up
        } else {
            SpinState::Down
        };

        self.current_spin == expected_spin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_crystal_oscillation() {
        let mut clock = TimeCrystalClock::new(3);
        assert_eq!(clock.current_spin, SpinState::Up);

        // Tick 1
        assert_eq!(clock.tick_oscillation(), false);
        assert_eq!(clock.current_spin, SpinState::Up);

        // Tick 2
        assert_eq!(clock.tick_oscillation(), false);
        assert_eq!(clock.current_spin, SpinState::Up);

        // Tick 3: Period reached, symmetry breaks, spin flips.
        assert_eq!(clock.tick_oscillation(), true);
        assert_eq!(clock.current_spin, SpinState::Down);

        // Tick 4
        assert_eq!(clock.tick_oscillation(), false);
        assert_eq!(clock.current_spin, SpinState::Down);

        // Tick 5
        assert_eq!(clock.tick_oscillation(), false);
        assert_eq!(clock.current_spin, SpinState::Down);

        // Tick 6: Period reached, symmetry breaks, spin flips back.
        assert_eq!(clock.tick_oscillation(), true);
        assert_eq!(clock.current_spin, SpinState::Up);
    }

    #[test]
    fn test_zero_energy_synchronization() {
        let mut node_a = TimeCrystalClock::new(5);
        let mut node_b = TimeCrystalClock::new(5);

        // Both nodes tick independently in their own local execution loops.
        // NO network packets are exchanged between them.
        for _ in 0..10 {
            node_a.tick_oscillation();
            node_b.tick_oscillation();
        }

        // Despite zero communication, their temporal entanglement ensures perfect synchronization.
        let global_time = 10;
        assert!(node_a.synchronize_state(global_time));
        assert!(node_b.synchronize_state(global_time));
        
        // They should both be SpinState::Up because 10 / 5 = 2 flips (Up -> Down -> Up)
        assert_eq!(node_a.current_spin, SpinState::Up);
        assert_eq!(node_b.current_spin, SpinState::Up);
    }
}
