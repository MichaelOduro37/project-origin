// ============================================================================
// PHASE 48: MINKOWSKI SPACETIME (CAUSAL BFT)
// ============================================================================
// Scientific mechanism: Einstein's Special Relativity (Light Cones)
//
// Traditional blockchains use global consensus to prevent double-spends and
// enforce causal ordering. This is fundamentally unscalable.
// Origin eliminates global consensus. Every transaction is a SpacetimeEvent.
// We calculate the Minkowski invariant locally:
// ds^2 = -c^2(dt)^2 + (dx)^2 + (dy)^2 + (dz)^2
// If ds^2 > 0 (spacelike interval), the events are causally disconnected.
// Any attempt to inject a double-spend outside the causal Light Cone is
// instantaneously rejected mathematically as an impossible paradox.
// ============================================================================

use std::fmt;

/// Represents a topological and temporal coordinate in the network.
#[derive(Debug, Clone, Copy)]
pub struct SpacetimeEvent {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub t: f64, // Temporal axis (timestamp)
}

#[derive(Debug)]
pub enum ParadoxError {
    SpacelikeSeparation(f64),
}

impl fmt::Display for ParadoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParadoxError::SpacelikeSeparation(ds_squared) => {
                write!(f, "Causal Paradox! ds^2 = {}. Events are spacelike separated.", ds_squared)
            }
        }
    }
}

/// Calculates the Minkowski Spacetime Interval (ds^2) between two events.
/// `speed_of_light`: The maximum theoretical propagation speed of the subnet.
pub fn calculate_spacetime_interval(event_a: &SpacetimeEvent, event_b: &SpacetimeEvent, speed_of_light: f64) -> f64 {
    let dt = event_b.t - event_a.t;
    let dx = event_b.x - event_a.x;
    let dy = event_b.y - event_a.y;
    let dz = event_b.z - event_a.z;

    let c_squared = speed_of_light * speed_of_light;
    
    // ds^2 = -c^2(dt)^2 + (dx)^2 + (dy)^2 + (dz)^2
    let temporal_component = -c_squared * (dt * dt);
    let spatial_component = (dx * dx) + (dy * dy) + (dz * dz);

    temporal_component + spatial_component
}

/// Evaluates if event_b could causally follow event_a.
/// Reject if they exist outside each other's light cones (ds^2 > 0).
pub fn verify_causality(event_a: &SpacetimeEvent, event_b: &SpacetimeEvent, speed_of_light: f64) -> Result<(), ParadoxError> {
    let ds_squared = calculate_spacetime_interval(event_a, event_b, speed_of_light);
    
    // In Minkowski spacetime:
    // ds^2 < 0 : Timelike (causally connected, standard propagation)
    // ds^2 == 0 : Lightlike (connected perfectly at the speed of light)
    // ds^2 > 0 : Spacelike (causally disconnected)
    
    // We allow a tiny floating-point epsilon above 0 to account for clock drift
    if ds_squared > 0.0001 {
        Err(ParadoxError::SpacelikeSeparation(ds_squared))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_causal_chain() {
        let speed_of_light = 10.0; // Network units per second

        let event_a = SpacetimeEvent { x: 0.0, y: 0.0, z: 0.0, t: 1.0 };
        // Event B happens 2 seconds later, and only 5 units away.
        // It had plenty of time to propagate.
        let event_b = SpacetimeEvent { x: 5.0, y: 0.0, z: 0.0, t: 3.0 };

        let result = verify_causality(&event_a, &event_b, speed_of_light);
        assert!(result.is_ok(), "Timelike event should be valid.");
    }

    #[test]
    fn test_causal_paradox_rejection() {
        let speed_of_light = 10.0; // Network units per second

        let event_a = SpacetimeEvent { x: 0.0, y: 0.0, z: 0.0, t: 1.0 };
        // Malicious node tries to inject Event B instantly (0.1 seconds later)
        // at a location 50 units away (which would require speed = 500).
        let event_b = SpacetimeEvent { x: 50.0, y: 0.0, z: 0.0, t: 1.1 };

        let result = verify_causality(&event_a, &event_b, speed_of_light);
        assert!(result.is_err(), "Spacelike event must be rejected!");
        
        if let Err(ParadoxError::SpacelikeSeparation(ds_squared)) = result {
            assert!(ds_squared > 0.0);
        }
    }
}
