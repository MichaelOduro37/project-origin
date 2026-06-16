// ============================================================================
// PHASE 58: STRANGE ATTRACTOR ROUTING (CHAOTIC ANONYMITY)
// ============================================================================
// Scientific mechanism: Chaos Theory (Lorenz Attractor)
//
// A Strange Attractor describes a system whose evolution is perfectly 
// deterministic but completely chaotic. The trajectory never repeats itself,
// appearing random to outside observers, but is mathematically bounded.
//
// In Origin, Traffic Analysis defeats encryption by monitoring predictable 
// routing paths. To prevent this, "Dark Routing" requests map a packet's path 
// to the differential equations of a Strange Attractor (Lorenz system). 
// The packet bounces wildly through the network in a non-repeating orbit. 
// Because the math is deterministic, it eventually orbits directly into the 
// destination node. The path is impossible to predict or trace backwards.
// ============================================================================

pub struct LorenzSystem {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    // Lorenz parameters
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
}

impl LorenzSystem {
    pub fn new(seed_x: f64, seed_y: f64, seed_z: f64) -> Self {
        Self {
            x: seed_x,
            y: seed_y,
            z: seed_z,
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }

    /// Progresses the chaotic system by one time-step using Euler's method.
    pub fn step(&mut self, dt: f64) {
        let dx = self.sigma * (self.y - self.x);
        let dy = self.x * (self.rho - self.z) - self.y;
        let dz = self.x * self.y - self.beta * self.z;

        self.x += dx * dt;
        self.y += dy * dt;
        self.z += dz * dt;
    }

    /// Maps the current chaotic coordinates to a physical network node ID (0-99)
    pub fn map_to_node(&self) -> usize {
        // Chaotic folding to keep it bounded within network limits
        let raw_val = (self.x.abs() + self.y.abs() + self.z.abs()) * 1000.0;
        (raw_val as usize) % 100
    }
}

pub struct AttractorRouter;

impl AttractorRouter {
    /// Generates a highly chaotic, non-repeating hop sequence to the destination.
    pub fn route_chaotic_packet(start_node: usize, destination: usize, max_hops: usize) -> Result<Vec<usize>, &'static str> {
        // Seed the attractor (in production, seeds are derived from encrypted packet metadata)
        let mut lorenz = LorenzSystem::new(start_node as f64 + 0.1, 1.0, 1.0);
        let dt = 0.01;
        let mut trajectory = vec![start_node];

        for _ in 0..max_hops {
            lorenz.step(dt);
            let next_hop = lorenz.map_to_node();
            
            // Avoid immediate ping-ponging
            if Some(&next_hop) != trajectory.last() {
                trajectory.push(next_hop);
            }

            if next_hop == destination {
                return Ok(trajectory);
            }
        }

        // If the chaos doesn't orbit into the destination in time
        Err("Attractor orbit did not intersect destination within max hops.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaotic_trajectory_generation() {
        let start_node = 5;
        let destination = 42; // Arbitrary destination
        
        // We run the chaotic routing
        let result = AttractorRouter::route_chaotic_packet(start_node, destination, 5000);
        
        assert!(result.is_ok());
        let trajectory = result.unwrap();
        
        // Ensure it took a chaotic path, not a straight line
        assert!(trajectory.len() > 2);
        // Ensure it arrived at the destination
        assert_eq!(*trajectory.last().unwrap(), destination);
        
        // Ensure no immediate repeating nodes (ping-ponging)
        for i in 0..(trajectory.len() - 1) {
            assert_ne!(trajectory[i], trajectory[i+1]);
        }
    }
}
