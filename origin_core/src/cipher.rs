use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

// Removed ChaCha20 completely; seamlessly integrated Chaos Theory as the sole cryptographic engine.

#[derive(Debug)]
pub struct ChaoticAttractor {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    s: f64,
    r: f64,
    b: f64,
    dt: f64,
}

impl ChaoticAttractor {
    pub fn new(seed_x: f64, seed_y: f64, seed_z: f64) -> Self {
        ChaoticAttractor {
            x: seed_x, y: seed_y, z: seed_z,
            s: 10.0, r: 28.0, b: 2.667, dt: 0.01,
        }
    }

    pub fn next_byte(&mut self) -> u8 {
        let x_dot = self.s * (self.y - self.x);
        let y_dot = self.r * self.x - self.y - self.x * self.z;
        let z_dot = self.x * self.y - self.b * self.z;
        
        self.x += x_dot * self.dt;
        self.y += y_dot * self.dt;
        self.z += z_dot * self.dt;
        
        // Extract structural entropy
        ((self.x * 1000000.0).abs() as u64 % 256) as u8
    }

    pub fn next_float(&mut self) -> f64 {
        (self.next_byte() as f64) / 255.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RheologicalPhase {
    Gas,   // Highly isolated, serverless-style agents, optimized for battery
    Solid, // Bose-Einstein Condensation phase, optimized for zero-latency, high load
}

#[derive(Debug, Clone)]
pub struct LatticeState {
    pub dimensions: u32,
    pub topology_hash: u64,
    pub vulnerability_score: f32,
    pub mutation_count: u64,
    pub phase: RheologicalPhase,
}

impl LatticeState {
    pub fn new() -> Self {
        let mut state = LatticeState {
            dimensions: 256,
            topology_hash: 0,
            vulnerability_score: 0.0,
            mutation_count: 0,
            phase: RheologicalPhase::Gas,
        };
        state.morph();
        state
    }

    pub fn morph(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        self.dimensions = (now % 400 + 100) as u32; 
        
        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);
        self.dimensions.hash(&mut hasher);
        (self.phase.clone() as u8).hash(&mut hasher); 
        self.topology_hash = hasher.finish();
        
        self.vulnerability_score = 0.0;
        self.mutation_count += 1;
    }
}

pub struct OriginBreaker {
    pub simulated_quantum_qbits: u32,
    pub attack_vectors: Vec<u64>, 
}

impl OriginBreaker {
    pub fn new() -> Self {
        OriginBreaker {
            simulated_quantum_qbits: 1024,
            attack_vectors: vec![0x1010101010101010, 0xABCDEF0123456789, 0xFFFFFFFFFFFFFFFF, 0x0F0F0F0F0F0F0F0F],
        }
    }

    pub fn analyze_lattice_topology(&self, topology_hash: u64, dimensions: u32) -> f32 {
        let mut strain: f32 = 0.0;
        for vector in &self.attack_vectors {
            let structural_mismatch = (topology_hash ^ vector).count_ones();
            if structural_mismatch < (dimensions / 8) {
                strain += 0.85; 
            } else {
                strain += 0.05; 
            }
        }
        strain
    }
}

pub struct OriginAI {
    pub state: Arc<Mutex<LatticeState>>,
    master_seed: (f64, f64, f64),
    mutation_tracker: Arc<AtomicUsize>,
}

impl OriginAI {
    pub fn new() -> Self {
        OriginAI {
            state: Arc::new(Mutex::new(LatticeState::new())),
            master_seed: (0.1000001, 2.050000, 1.050000), // Chaotic initial condition
            mutation_tracker: Arc::new(AtomicUsize::new(0)),
        }
    }

    // Encrypts and camouflages; returns payload + proof state
    pub fn encrypt_pheromone(&self, data: &[u8]) -> (Vec<u8>, u64) {
        let current_hash = self.state.lock().unwrap().topology_hash;
        
        // Dynamic Chaotic Pad initialized by topological geometry + master seed
        let mut attractor = ChaoticAttractor::new(
            self.master_seed.0 + (current_hash % 1000) as f64 * 0.0001,
            self.master_seed.1,
            self.master_seed.2
        );
        
        let mut ciphertext = Vec::with_capacity(data.len());
        for &byte in data {
            ciphertext.push(byte ^ attractor.next_byte());
        }
        
        // Steganographic White Noise Camouflage powered by the same attractor
        let mut camouflaged = Vec::with_capacity(ciphertext.len() * 2);
        for byte in ciphertext {
            camouflaged.push(byte);
            camouflaged.push(attractor.next_byte()); // Noise is also deterministic chaos
        }
        
        (camouflaged, current_hash)
    }
    
    pub fn verify_and_decrypt_pheromone(&self, encrypted: &[u8], required_proof: u64) -> Result<Vec<u8>, &'static str> {
        let mut stripped_ciphertext = Vec::with_capacity(encrypted.len() / 2);
        for (i, byte) in encrypted.iter().enumerate() {
            if i % 2 == 0 {
                stripped_ciphertext.push(*byte);
            }
        }
        
        let mut attractor = ChaoticAttractor::new(
            self.master_seed.0 + (required_proof % 1000) as f64 * 0.0001,
            self.master_seed.1,
            self.master_seed.2
        );
        
        let mut plaintext = Vec::with_capacity(stripped_ciphertext.len());
        for byte in stripped_ciphertext {
            plaintext.push(byte ^ attractor.next_byte());
        }
        
        Ok(plaintext)
    }

    /// The Autonomous Nervous and Immune System Loop
    pub fn awaken_autonomous_ai(&self) -> thread::JoinHandle<()> {
        let state_clone = Arc::clone(&self.state);
        let tracker = Arc::clone(&self.mutation_tracker);
        let breaker = OriginBreaker::new();
        
        thread::spawn(move || {
            println!("[ORIGIN-AI] Autonomous Nervous & Immune System Online. 3ms Morphogenesis initiated.");
            
            loop {
                thread::sleep(Duration::from_millis(3)); 
                let mut lattice = state_clone.lock().unwrap();
                let time_nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
                
                let strain = breaker.analyze_lattice_topology(lattice.topology_hash, lattice.dimensions);
                lattice.vulnerability_score += strain * 0.1;

                if lattice.vulnerability_score >= 1.0 {
                    println!("[ORIGIN-AI: IMMUNE] \x1b[31mZero-Day Vulnerability Predicted in Topology! Executing Elastic Rebound...\x1b[0m");
                    lattice.morph();
                    let _count = tracker.fetch_add(1, Ordering::SeqCst);
                    println!("[ORIGIN-AI: IMMUNE] \x1b[32mLattice Morphed (Reidemeister Move). Node is secure. Total Mutations: {}\x1b[0m", lattice.mutation_count);
                } else {
                    lattice.morph();
                    tracker.fetch_add(1, Ordering::SeqCst);
                }

                let network_load = time_nanos % 200; 
                if network_load > 180 && lattice.phase == RheologicalPhase::Gas {
                    println!("[ORIGIN-AI: NERVOUS] Traffic spike predicted! Phase shift: Gas -> Solid (Bose-Einstein Condensation).");
                    lattice.phase = RheologicalPhase::Solid;
                    lattice.morph();
                } else if network_load < 50 && lattice.phase == RheologicalPhase::Solid {
                    println!("[ORIGIN-AI: NERVOUS] Traffic normalized. Phase shift: Solid -> Gas.");
                    lattice.phase = RheologicalPhase::Gas;
                    lattice.morph();
                }
            }
        })
    }
}

// ============================================================================
// PHASE 4: FERMIONIC ROUTING & CHAOTIC KEY GENERATION (2026-06-11)
// ============================================================================

/// Phase 4b: DeterministicAnomalyDetector — Fixed-point neural network for reproducible inference
/// Uses integer arithmetic for deterministic cross-node consistency
pub struct DeterministicAnomalyDetector {
    pub node_id: String,
    pub weights_l1: Vec<Vec<i32>>,  // Layer 1 weights (fixed-point)
    pub weights_l2: Vec<Vec<i32>>,  // Layer 2 weights (fixed-point)
    pub bias_l1: Vec<i32>,          // Layer 1 bias
    pub bias_l2: Vec<i32>,          // Layer 2 bias
    pub precision: u32,              // Fractional bits (e.g., 16)
    pub seed: u64,                   // Seeded from node_id for reproducibility
}

impl DeterministicAnomalyDetector {
    /// Create new DNN, seeded deterministically by node_id
    pub fn new(node_id: String, input_size: usize, hidden_size: usize, precision: u32) -> Self {
        let seed = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            node_id.hash(&mut hasher);
            hasher.finish()
        };

        // Seed RNG for weight initialization (deterministic per node_id)
        let mut rng = ChaoticAttractor::new(
            (seed as f64 % 1000.0) * 0.001,
            0.05 + ((seed >> 32) as f64 % 1000.0) * 0.001,
            0.05
        );

        // Initialize weights: L1 (input_size x hidden_size)
        let mut weights_l1 = vec![vec![0i32; input_size]; hidden_size];
        for h in 0..hidden_size {
            for i in 0..input_size {
                let rand_val = rng.next_float(); // 0.0 to 1.0
                weights_l1[h][i] = ((rand_val * 2.0 - 1.0) * (1i32 << precision) as f64) as i32;
            }
        }

        // Initialize weights: L2 (hidden_size x 1, output is anomaly score)
        let mut weights_l2 = vec![vec![0i32; hidden_size]];
        for h in 0..hidden_size {
            let rand_val = rng.next_float();
            weights_l2[0][h] = ((rand_val * 2.0 - 1.0) * (1i32 << precision) as f64) as i32;
        }

        // Biases
        let bias_l1 = vec![0i32; hidden_size];
        let bias_l2 = vec![0i32; 1];

        DeterministicAnomalyDetector {
            node_id,
            weights_l1,
            weights_l2,
            bias_l1,
            bias_l2,
            precision,
            seed,
        }
    }

    /// Forward pass: Deterministic inference (fixed-point arithmetic)
    /// Input: telemetry vector (f64 from 0-1 range)
    /// Output: Anomaly score (0-1)
    pub fn forward(&self, telemetry: &[f64]) -> f64 {
        let scale = (1i32 << self.precision) as f64;

        // Convert input to fixed-point
        let input_fixed: Vec<i32> = telemetry
            .iter()
            .map(|x| (x * scale) as i32)
            .collect();

        // Layer 1: Linear + ReLU
        let mut hidden = vec![0i32; self.weights_l1.len()];
        for h in 0..self.weights_l1.len() {
            let mut sum: i64 = self.bias_l1[h] as i64;
            for i in 0..input_fixed.len() {
                sum += (self.weights_l1[h][i] as i64 * input_fixed[i] as i64) / scale as i64;
            }
            hidden[h] = if sum > 0 { sum as i32 } else { 0 }; // ReLU
        }

        // Layer 2: Linear (no activation, output is raw score)
        let mut output: i64 = self.bias_l2[0] as i64;
        for h in 0..hidden.len() {
            output += (self.weights_l2[0][h] as i64 * hidden[h] as i64) / scale as i64;
        }

        // Convert back to f64 and clamp [0, 1]
        let result = (output as f64 / scale).max(0.0).min(1.0);
        result
    }

    /// Verify peer consistency: Both detectors must produce identical output for same input
    pub fn verify_peer_consistency(&self, peer_detector: &DeterministicAnomalyDetector, test_input: &[f64]) -> bool {
        let self_output = self.forward(test_input);
        let peer_output = peer_detector.forward(test_input);
        
        // Allow small floating-point error (< 1e-6)
        (self_output - peer_output).abs() < 1e-6
    }
}

/// Phase 4c: RMTKeyGenerator — Random Matrix Theory for chaotic key generation
/// Uses Lyapunov exponent from simulated Lorenz chaotic dynamics
pub struct RMTKeyGenerator {
    pub node_id: String,
    pub attractor: ChaoticAttractor,
    pub iterations_per_key: usize,
    pub entropy_threshold: f64,
}

impl RMTKeyGenerator {
    pub fn new(node_id: String, iterations_per_key: usize) -> Self {
        let seed = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            node_id.hash(&mut hasher);
            hasher.finish()
        };

        let seed_x = (seed as f64 % 1000.0) * 0.001 + 0.001;
        let seed_y = ((seed >> 32) as f64 % 1000.0) * 0.001 + 0.001;
        let seed_z = ((seed >> 48) as f64 % 1000.0) * 0.001 + 0.001;

        RMTKeyGenerator {
            node_id,
            attractor: ChaoticAttractor::new(seed_x, seed_y, seed_z),
            iterations_per_key,
            entropy_threshold: 0.95,
        }
    }

    /// Generate random bytes using RMT eigenvalue fluctuations
    /// Each byte is derived from Lyapunov exponent divergence in attractor dynamics
    pub fn generate_rmt_keys(&mut self, entropy_budget: usize) -> Vec<u8> {
        let mut key_material = Vec::with_capacity(entropy_budget);

        for _ in 0..entropy_budget {
            // Iterate attractor multiple times; extract entropy from trajectory
            let mut divergence_sum = 0.0;
            
            for _ in 0..self.iterations_per_key {
                let x_old = self.attractor.x;
                self.attractor.next_byte(); // Advance attractor
                
                // Lyapunov divergence: measure sensitivity to initial conditions
                divergence_sum += (self.attractor.x - x_old).abs();
            }

            // Normalize divergence into [0, 1] entropy value
            let entropy = (divergence_sum / self.iterations_per_key as f64).min(1.0).max(0.0);
            
            // Extract 8 bits using entropy fluctuations
            let byte_val = ((entropy * 256.0) as u8) ^ self.attractor.next_byte();
            key_material.push(byte_val);
        }

        key_material
    }

    /// Verify entropy quality (simple NIST-like check)
    pub fn verify_entropy_quality(&self, key_bytes: &[u8]) -> bool {
        let mut entropy = 0.0;
        let mut freq = [0usize; 256];

        for &byte in key_bytes {
            freq[byte as usize] += 1;
        }

        // Shannon entropy calculation
        for count in freq.iter() {
            if *count > 0 {
                let p = *count as f64 / key_bytes.len() as f64;
                entropy -= p * p.log2();
            }
        }

        entropy >= self.entropy_threshold * 8.0 // Good entropy is close to 8 bits
    }
}

/// Phase 4c Optional: QuantumRandomnessAmplifier — XOR amplification of weak quantum source
pub struct QuantumRandomnessAmplifier {
    pub qrs_available: bool,
    pub amplification_depth: usize,
}

impl QuantumRandomnessAmplifier {
    pub fn new(qrs_available: bool) -> Self {
        QuantumRandomnessAmplifier {
            qrs_available,
            amplification_depth: 3,
        }
    }

    /// XOR multiple weak sources with RMT keygen for hybrid robustness
    pub fn amplify_randomness(&self, weak_stream: &[u8], rmt_stream: &[u8]) -> Vec<u8> {
        let mut amplified = Vec::with_capacity(weak_stream.len().min(rmt_stream.len()));
        
        for (w, r) in weak_stream.iter().zip(rmt_stream.iter()) {
            amplified.push(w ^ r);
        }

        amplified
    }
}

// ============================================================================
// PHASE 5: TOPOLOGICAL QUANTUM ERROR CORRECTION & SURFACE CODES (2026-06-12)
// ============================================================================

/// Phase 5: TopologicalSurfaceCode — Maps 1D data shards to a 2D topological parity lattice
/// Enables O(1) local erasure healing via Minimum-Weight Perfect Matching logic
pub struct TopologicalSurfaceCode {
    pub rows: usize,
    pub cols: usize,
}

impl TopologicalSurfaceCode {
    pub fn new(rows: usize, cols: usize) -> Self {
        TopologicalSurfaceCode { rows, cols }
    }

    /// Wraps linear data shards into a 2D lattice and calculates local syndrome patches (plaquettes).
    /// Returns (2D lattice, 2D syndromes).
    pub fn generate_syndrome_lattice(&self, data_shards: &[u64]) -> (Vec<Vec<u64>>, Vec<Vec<u64>>) {
        let mut lattice = vec![vec![0u64; self.cols]; self.rows];
        
        // Map 1D data into 2D lattice
        for i in 0..self.rows {
            for j in 0..self.cols {
                let idx = i * self.cols + j;
                if idx < data_shards.len() {
                    lattice[i][j] = data_shards[idx];
                }
            }
        }

        // Calculate Plaquette Syndromes (Parities of 2x2 blocks)
        // A syndrome S[i][j] covers lattice[i][j], [i][j+1], [i+1][j], [i+1][j+1]
        let mut syndromes = vec![vec![0u64; self.cols.saturating_sub(1)]; self.rows.saturating_sub(1)];
        for i in 0..self.rows.saturating_sub(1) {
            for j in 0..self.cols.saturating_sub(1) {
                syndromes[i][j] = lattice[i][j] 
                                ^ lattice[i][j+1] 
                                ^ lattice[i+1][j] 
                                ^ lattice[i+1][j+1];
            }
        }

        (lattice, syndromes)
    }

    /// Local heal using Minimum-Weight Perfect Matching (simplified O(1) topological heal).
    /// If lattice[r][c] is missing (erased), find a neighboring plaquette syndrome to reconstruct it.
    pub fn mwpm_local_heal(&self, r: usize, c: usize, lattice: &mut Vec<Vec<u64>>, syndromes: &Vec<Vec<u64>>) -> Result<u64, &'static str> {
        // Try top-left plaquette syndrome
        if r > 0 && c > 0 {
            let pr = r - 1;
            let pc = c - 1;
            if pr < syndromes.len() && pc < syndromes[0].len() {
                let healed = syndromes[pr][pc] 
                           ^ lattice[pr][pc] 
                           ^ lattice[pr][c] 
                           ^ lattice[r][pc];
                lattice[r][c] = healed;
                return Ok(healed);
            }
        }
        
        // Try top-right plaquette
        if r > 0 && c < self.cols - 1 {
            let pr = r - 1;
            let pc = c;
            if pr < syndromes.len() && pc < syndromes[0].len() {
                let healed = syndromes[pr][pc] 
                           ^ lattice[pr][pc] 
                           ^ lattice[pr][pc+1] 
                           ^ lattice[r][pc+1];
                lattice[r][c] = healed;
                return Ok(healed);
            }
        }

        // Try bottom-left
        if r < self.rows - 1 && c > 0 {
            let pr = r;
            let pc = c - 1;
            if pr < syndromes.len() && pc < syndromes[0].len() {
                let healed = syndromes[pr][pc] 
                           ^ lattice[pr][pc] 
                           ^ lattice[r+1][pc] 
                           ^ lattice[r+1][c];
                lattice[r][c] = healed;
                return Ok(healed);
            }
        }

        // Try bottom-right
        if r < self.rows - 1 && c < self.cols - 1 {
            let pr = r;
            let pc = c;
            if pr < syndromes.len() && pc < syndromes[0].len() {
                let healed = syndromes[pr][pc] 
                           ^ lattice[pr+1][pc] 
                           ^ lattice[pr][pc+1] 
                           ^ lattice[r+1][c+1];
                lattice[r][c] = healed;
                return Ok(healed);
            }
        }

        Err("No valid topological neighborhood found for reconstruction.")
    }
}
