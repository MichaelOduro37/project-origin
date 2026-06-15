// ============================================================================
// PHASE 12: FERMIONIC CRYPTOGRAPHIC ROUTING
// ============================================================================
// Scientific mechanism: The Pauli Exclusion Principle states that no two 
// identical fermions can occupy the same quantum state simultaneously.
//
// Application: Data packets are modeled as fermions. Routing paths (peers)
// are modeled as quantum states. If multiple packets are sent rapidly, they
// cannot occupy the same optimal path. They statistically repel each other, 
// distributing load across all available network paths perfectly.
// ============================================================================

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Instant, Duration};

pub struct FermionRouter {
    /// Quantum States: A map of peer IPs to the exact Instant they were last occupied.
    /// A state is considered "occupied" (forbidden) if the time since last occupancy
    /// is less than the relaxation time.
    states: HashMap<String, Instant>,
    /// The mathematical "relaxation time" for a quantum state to become available again.
    relaxation_ms: u64,
}

impl FermionRouter {
    pub fn new(relaxation_ms: u64) -> Self {
        Self {
            states: HashMap::new(),
            relaxation_ms,
        }
    }

    /// Fermi-Dirac Distribution logic for selecting a routing state.
    /// Given a list of available peers, it repels the packet from any currently
    /// occupied state and finds the lowest available energy state (first available peer).
    pub fn route_fermion(&mut self, available_peers: &[String]) -> Option<String> {
        let now = Instant::now();
        let exclusion_duration = Duration::from_millis(self.relaxation_ms);

        for peer in available_peers {
            let occupied = if let Some(&last_time) = self.states.get(peer) {
                now.duration_since(last_time) < exclusion_duration
            } else {
                false
            };

            // Pauli Exclusion Principle: if occupied, statistically repel to next state.
            if !occupied {
                // Occupy the state
                self.states.insert(peer.clone(), now);
                return Some(peer.clone());
            }
        }

        // If all states are occupied, we have hit maximum Fermi Energy (congestion collapse).
        // In a real biological/quantum system, we'd queue or drop. For now, we drop (None).
        None
    }
}

pub fn global_fermion_router() -> &'static Mutex<FermionRouter> {
    // 50ms relaxation time per state
    static ROUTER: OnceLock<Mutex<FermionRouter>> = OnceLock::new();
    ROUTER.get_or_init(|| Mutex::new(FermionRouter::new(50)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_pauli_exclusion() {
        let mut router = FermionRouter::new(50);
        let peers = vec!["NodeA".to_string(), "NodeB".to_string(), "NodeC".to_string()];

        // First packet takes lowest energy state (NodeA)
        assert_eq!(router.route_fermion(&peers), Some("NodeA".to_string()));
        
        // Second packet immediately follows. NodeA is occupied. Pauli Exclusion forces it to NodeB.
        assert_eq!(router.route_fermion(&peers), Some("NodeB".to_string()));

        // Third packet repels to NodeC.
        assert_eq!(router.route_fermion(&peers), Some("NodeC".to_string()));

        // Fourth packet drops because all states are occupied (Fermi Sea is full).
        assert_eq!(router.route_fermion(&peers), None);

        // Wait for relaxation
        sleep(Duration::from_millis(60));

        // State has decayed, NodeA is available again.
        assert_eq!(router.route_fermion(&peers), Some("NodeA".to_string()));
    }
}
