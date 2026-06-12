use std::collections::{HashSet, HashMap};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PheromoneShard {
    pub message_id: String,
    pub group_id: Option<String>, // None for 1-on-1, Some for Group Chats
    pub shard_index: usize,
    pub total_shards: usize,
    pub encrypted_payload: Vec<u8>,
    pub zero_knowledge_proof: u64, // The lock that the cipher uses
}

// Represents a CRDT Ring for a Group Chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginGroup {
    pub group_id: String,
    pub members: HashSet<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum NodeTrait {
    Client,
    Relay,
    HeavyCompute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareEnvironment {
    pub battery_level: u8,    // 0-100
    pub is_plugged_in: bool,
    pub thermal_load: u8,     // 0-100
    pub cpu_cores: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub node_id: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct TensegritySwarmNode {
    pub node_id: String,
    pub pheromones: HashSet<PheromoneShard>,
    pub known_groups: HashMap<String, OriginGroup>,
    pub environment: HardwareEnvironment,
    pub active_traits: HashSet<NodeTrait>,
    pub peer_tension: HashMap<String, u64>, // node_id -> last_seen_timestamp
    pub global_tension: f32, // The mathematical strain on this node
}

impl TensegritySwarmNode {
    pub fn new(node_id: &str, environment: HardwareEnvironment) -> Self {
        let mut node = TensegritySwarmNode {
            node_id: node_id.to_string(),
            pheromones: HashSet::new(),
            known_groups: HashMap::new(),
            environment,
            active_traits: HashSet::new(),
            peer_tension: HashMap::new(),
            global_tension: 0.0,
        };
        node.quorum_sense();
        node
    }

    /// Evaluates the environment and dynamically expresses traits (Device Symbiosis)
    pub fn quorum_sense(&mut self) {
        self.active_traits.clear();
        
        // Base trait: Everyone is a client
        self.active_traits.insert(NodeTrait::Client);

        // Thermal & Battery Guardrails (Hibernation Trigger)
        if self.environment.battery_level < 30 && !self.environment.is_plugged_in || self.environment.thermal_load > 80 {
            println!("[{}] Hibernation Triggered: Conserving battery/thermals. Dropping Relay traits.", self.node_id);
            return; // Gas phase only, no mesh duties
        }

        // Parasitic Reversal (The Charger State)
        if self.environment.is_plugged_in || self.environment.battery_level > 70 {
            self.active_traits.insert(NodeTrait::Relay);
        }

        // Data Center / Heavy Node detection
        if self.environment.cpu_cores >= 16 && self.environment.is_plugged_in {
            self.active_traits.insert(NodeTrait::HeavyCompute);
            println!("[{}] Heavy Compute Expressed: Ready for Bose-Einstein Condensation.", self.node_id);
        } else if self.active_traits.contains(&NodeTrait::Relay) {
            println!("[{}] Relay Trait Expressed: Volunteering for mesh routing.", self.node_id);
        }
    }

    pub fn join_group(&mut self, group: OriginGroup) {
        self.known_groups.insert(group.group_id.clone(), group);
    }

    pub fn ingest_heartbeat(&mut self, heartbeat: Heartbeat) {
        self.peer_tension.insert(heartbeat.node_id, heartbeat.timestamp);
    }

    pub fn generate_heartbeat(&self) -> Heartbeat {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Heartbeat {
            node_id: self.node_id.clone(),
            timestamp: now,
        }
    }

    pub fn evaluate_tension_cascade(&mut self) -> Option<Vec<PheromoneShard>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut missing_peers = 0;

        for (peer_id, last_seen) in &self.peer_tension {
            // If a peer hasn't been seen in 3 seconds, consider them partitioned/dead
            if now > *last_seen && (now - last_seen) > 3 {
                missing_peers += 1;
                println!("[TENSEGRITY] Node {} detects {} has dropped offline.", self.node_id, peer_id);
            }
        }

        // Tension increases quadratically based on missing peers (Chaos Theory)
        let chaotic_strain = (missing_peers as f32).powf(2.0);
        
        // The Strange Attractor: The AI applies a damping coefficient to prevent total collapse
        // during temporary network jitter, smoothing the chaotic strain.
        let strange_attractor_damping = if chaotic_strain > 0.0 { 0.85 } else { 1.0 };
        self.global_tension = chaotic_strain * strange_attractor_damping;

        if self.global_tension >= 0.85 && self.active_traits.contains(&NodeTrait::Relay) {
            println!("[TENSEGRITY] Structural tension critical on {} (Tension: {:.2}). Executing Trophic Cascade! Replicating memory fragments...", self.node_id, self.global_tension);
            // Return local shards to be re-broadcast (Bose-Einstein Condensation)
            return Some(self.pheromones.iter().cloned().collect());
        }

        None
    }

    pub fn ingest_pheromone(&mut self, shard: PheromoneShard) -> bool {
        // If the node is not a relay and this isn't specifically addressed to them (simplified for simulation),
        // they might drop it to save battery.
        if !self.active_traits.contains(&NodeTrait::Relay) {
            // For simulation purposes, we let them ingest if they are a client, but ideally
            // a purely hibernating node won't buffer random mesh traffic.
            // We return false to indicate it refused to relay.
            return false;
        }
        
        self.pheromones.insert(shard);
        true
    }

    // Mathematical merge of CRDT state (The Gossip Protocol)
    pub fn merge(&mut self, other: &TensegritySwarmNode) {
        for shard in &other.pheromones {
            self.pheromones.insert(shard.clone());
        }
    }
    
    // Tries to reassemble any complete messages available in the local state
    // Filters for 1-on-1 or specific group messages based on the Node's context
    pub fn reassemble_messages(&self, ai_cipher: &crate::cipher::OriginAI) -> Vec<(String, Option<String>, String)> {
        let mut shard_groups: HashMap<String, Vec<&PheromoneShard>> = HashMap::new();
        
        // Group shards by message ID
        for shard in &self.pheromones {
            shard_groups.entry(shard.message_id.clone()).or_default().push(shard);
        }
        
        let mut completed_messages = Vec::new();
        
        for (msg_id, mut shards) in shard_groups {
            if shards.is_empty() { continue; }
            let total = shards[0].total_shards;
            let group_id = shards[0].group_id.clone();

            if shards.len() == total {
                // We have all shards! Sort them.
                shards.sort_by_key(|s| s.shard_index);
                
                let mut full_payload = Vec::new();
                for shard in &shards {
                    full_payload.extend_from_slice(&shard.encrypted_payload);
                }
                
                // Decrypt using the ZKP (topology hash) attached to the first shard
                let proof = shards[0].zero_knowledge_proof;
                if let Ok(decrypted_bytes) = ai_cipher.verify_and_decrypt_pheromone(&full_payload, proof) {
                    if let Ok(content) = String::from_utf8(decrypted_bytes) {
                        completed_messages.push((msg_id.clone(), group_id, content));
                    }
                }
            }
        }
        
        completed_messages
    }
}

// ============================================================================
// PHASE 4E: ISING-TENSEGRITY OPTIMIZER (Updated 2026-06-12)
// ============================================================================

/// Phase 4e: IsingTensegrityOptimizer — Energy minimization for load shedding
/// Replaces heuristic chaos-control with a quantum-inspired Hamiltonian model.
/// Nodes act as Ising spins (+1 = Accept Load, -1 = Shed Load).
pub struct IsingTensegrityOptimizer {
    pub node_id: String,
    pub spin: i8, // +1 or -1
    pub local_load: f64, // 0.0 to 1.0 (acts as external magnetic field h_i)
    pub peer_spins: HashMap<String, i8>,
    pub peer_tensions: HashMap<String, f64>, // Interaction strengths J_ij
    pub temperature: f64, // For simulated annealing
    pub rng: crate::cipher::ChaoticAttractor, // Deterministic chaos for annealing
}

impl IsingTensegrityOptimizer {
    pub fn new(node_id: String) -> Self {
        let seed = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            node_id.hash(&mut hasher);
            hasher.finish()
        };

        IsingTensegrityOptimizer {
            node_id,
            spin: 1, // Default: Accept load
            local_load: 0.0,
            peer_spins: HashMap::new(),
            peer_tensions: HashMap::new(),
            temperature: 10.0, // Initial high temperature for annealing
            rng: crate::cipher::ChaoticAttractor::new(
                (seed as f64 % 1000.0) * 0.001 + 0.001,
                ((seed >> 32) as f64 % 1000.0) * 0.001 + 0.001,
                ((seed >> 48) as f64 % 1000.0) * 0.001 + 0.001
            ),
        }
    }

    /// Update local load (acts as external magnetic field h_i)
    /// High load means h_i is negative, pushing spin towards -1 (shed)
    pub fn update_local_load(&mut self, packet_count: usize, avg_latency_ms: f64) {
        let load = (packet_count as f64 / 1000.0) + (avg_latency_ms / 100.0);
        self.local_load = load.min(1.0);
    }

    /// Receive gossip about peer spins and their relative tension (distance/bandwidth)
    pub fn ingest_peer_state(&mut self, peer_id: String, spin: i8, tension: f64) {
        self.peer_spins.insert(peer_id.clone(), spin);
        self.peer_tensions.insert(peer_id, tension);
    }

    /// Calculate the Hamiltonian Energy of a given spin state
    /// H = - \sum J_{ij} s_i s_j - h_i s_i
    pub fn calculate_energy(&self, candidate_spin: i8) -> f64 {
        let mut interaction_energy = 0.0;
        
        for (peer_id, peer_spin) in &self.peer_spins {
            if let Some(j_ij) = self.peer_tensions.get(peer_id) {
                // Ferromagnetic coupling: We want to align with peers to share load,
                // BUT if they are shedding (-1), we might need to accept (+1) to compensate.
                // In Tensegrity, tension means anti-ferromagnetic coupling for load balancing:
                // If a neighbor is shedding (-1), it increases pressure on us to accept (+1).
                // So J_ij is negative (anti-ferromagnetic)
                let anti_ferromagnetic_j = -j_ij;
                interaction_energy += anti_ferromagnetic_j * (candidate_spin as f64) * (*peer_spin as f64);
            }
        }

        // External field h_i pushes spin to -1 if local load is high, and +1 if load is low.
        // If load = 1.0 (max), h_i = -1.0. If load = 0.0 (idle), h_i = +1.0.
        let h_i = 1.0 - (2.0 * self.local_load);
        let field_energy = -h_i * (candidate_spin as f64);

        interaction_energy + field_energy
    }

    /// Relax to ground state using quantum-inspired simulated annealing
    pub fn relax_to_ground_state(&mut self) -> i8 {
        let current_energy = self.calculate_energy(self.spin);
        let flipped_spin = -self.spin;
        let candidate_energy = self.calculate_energy(flipped_spin);

        let delta_e = candidate_energy - current_energy;

        if delta_e < 0.0 {
            // Lower energy state found, accept deterministically
            self.spin = flipped_spin;
        } else {
            // Probabilistic acceptance based on temperature (Metropolis-Hastings)
            let acceptance_probability = (-delta_e / self.temperature).exp();
            let roll = self.rng.next_float();
            
            if roll < acceptance_probability {
                self.spin = flipped_spin;
            }
        }

        // Cool down the system
        self.temperature = (self.temperature * 0.95).max(0.01);
        
        let state_str = if self.spin == 1 { "ACCEPTING" } else { "SHEDDING" };
        println!("[ISING OPTIMIZER] Node {} relaxed to spin {} ({}). Local Load: {:.2}, Temp: {:.4}", 
                 self.node_id, self.spin, state_str, self.local_load, self.temperature);
                 
        self.spin
    }
}
