// ============================================================================
// PHASE 33: CONTINUOUS LEADER ELECTION (TURING PATTERNS)
// ============================================================================
// Scientific mechanism: Reaction-Diffusion Turing Patterns (Alan Turing, 1952)
//
// Origin abandons $O(N^2)$ voting protocols (Paxos/Raft) for continuous 
// leader election. The network simulates two interacting chemicals over a 
// Graph Laplacian. An Activator (U) and an Inhibitor (V) diffuse across 
// network links. Because the Inhibitor diffuses much faster than the 
// Activator, the homogeneous state becomes unstable (Turing Instability). 
// The network spontaneously breaks symmetry, causing localized peaks ("spots") 
// of Activator concentration. Nodes located at these peaks automatically 
// become Swarm Anchors (temporary leaders/validators).
// ============================================================================

pub struct TuringPatternSystem {
    pub num_nodes: usize,
    pub u: Vec<f64>, // Activator concentration
    pub v: Vec<f64>, // Inhibitor concentration
    pub adjacency_list: Vec<Vec<usize>>,
    pub d_u: f64,    // Diffusion rate of Activator
    pub d_v: f64,    // Diffusion rate of Inhibitor
    pub dt: f64,     // Time step
}

impl TuringPatternSystem {
    pub fn new(num_nodes: usize, d_u: f64, d_v: f64, dt: f64) -> Self {
        // Initialize with slight random noise around 0.0 to trigger instability
        let mut u = Vec::with_capacity(num_nodes);
        let mut v = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            u.push((rand::random::<f64>() - 0.5) * 0.1);
            v.push((rand::random::<f64>() - 0.5) * 0.1);
        }

        Self {
            num_nodes,
            u,
            v,
            adjacency_list: vec![Vec::new(); num_nodes],
            d_u,
            d_v,
            dt,
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.adjacency_list[a].push(b);
        self.adjacency_list[b].push(a);
    }

    /// Advances the Reaction-Diffusion PDE over the Graph Laplacian
    pub fn step(&mut self) {
        let mut next_u = self.u.clone();
        let mut next_v = self.v.clone();

        for i in 0..self.num_nodes {
            // Compute Discrete Graph Laplacian for diffusion
            let mut laplacian_u = 0.0;
            let mut laplacian_v = 0.0;
            
            for &neighbor in &self.adjacency_list[i] {
                laplacian_u += self.u[neighbor] - self.u[i];
                laplacian_v += self.v[neighbor] - self.v[i];
            }

            // Chemical Kinetics (Generic Activator-Inhibitor)
            // Activator promotes itself but is suppressed by Inhibitor
            let reaction_u = self.u[i] - self.u[i].powi(3) - self.v[i];
            
            // Inhibitor is promoted by Activator and decays naturally
            let reaction_v = 0.3 * self.u[i] - 0.2 * self.v[i];

            // PDE Update
            next_u[i] = self.u[i] + self.dt * (reaction_u + self.d_u * laplacian_u);
            next_v[i] = self.v[i] + self.dt * (reaction_v + self.d_v * laplacian_v);
        }

        self.u = next_u;
        self.v = next_v;
    }

    /// Check if any nodes have exceeded the Activator threshold, becoming Anchors
    pub fn get_anchors(&self, threshold: f64) -> Vec<(usize, f64)> {
        let mut anchors = Vec::new();
        for i in 0..self.num_nodes {
            if self.u[i] > threshold {
                anchors.push((i, self.u[i]));
            }
        }
        anchors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turing_symmetry_breaking_leader_election() {
        // Create a 50-node ring topology network
        let num_nodes = 50;
        // D_v MUST be significantly larger than D_u for Turing Instability
        let mut turing = TuringPatternSystem::new(num_nodes, 0.01, 0.2, 0.1);
        
        for i in 0..num_nodes {
            turing.add_edge(i, (i + 1) % num_nodes);
        }

        // Run the PDE solver until symmetry breaks and spots form
        for _ in 0..150 {
            turing.step();
        }

        let anchors = turing.get_anchors(0.8);
        
        // Mathematically, the uniform noise MUST have differentiated into distinct
        // high-concentration "spots". These nodes are our elected Leaders.
        assert!(!anchors.is_empty(), "Turing instability failed to elect any Anchors");
        assert!(anchors.len() < num_nodes / 2, "Turing spots should be sparse, not global");
    }
}
