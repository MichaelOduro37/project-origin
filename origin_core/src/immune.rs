use rand::RngExt;
use std::collections::HashMap;

/// Phase 4a: SwarmAnomalyVoter — Byzantine-resilient consensus voting layer
/// Implements threshold voting: decisions require >66% (2/3) quorum for anomaly consensus
pub struct SwarmAnomalyVoter {
    pub node_id: String,
    pub local_confidence: f64,
    pub neighbor_scores: HashMap<String, f64>,
    pub byzantine_threshold: f64, // Default: 0.66 (2/3 majority)
    pub gossip_timeout_ms: u64,
}

impl SwarmAnomalyVoter {
    pub fn new(node_id: String, byzantine_threshold: f64) -> Self {
        SwarmAnomalyVoter {
            node_id,
            local_confidence: 0.0,
            neighbor_scores: HashMap::new(),
            byzantine_threshold,
            gossip_timeout_ms: 200,
        }
    }

    /// Local anomaly detection (wrapped NSA detector output)
    pub fn detect_local_anomaly(&mut self, is_anomalous: bool, confidence: f64) -> () {
        self.local_confidence = if is_anomalous { confidence } else { 0.0 };
    }

    /// Gossip anomaly score to neighbors (simplified broadcast)
    pub fn broadcast_score_to_neighbors(&self, neighbors: &[String]) -> HashMap<String, f64> {
        let mut gossip = HashMap::new();
        for neighbor in neighbors {
            gossip.insert(neighbor.clone(), self.local_confidence);
        }
        gossip
    }

    /// Collect scores from neighbors (simulated)
    pub fn collect_neighbor_scores(&mut self, scores: HashMap<String, f64>) {
        self.neighbor_scores.extend(scores);
    }

    /// Byzantine voting: Count votes above threshold
    /// Returns true if >= 66% of nodes (including self) vote for anomaly
    pub fn count_votes_above_threshold(&self, threshold: f64) -> bool {
        let mut anomaly_votes = 0;
        let mut total_votes = 1; // Start with self
        
        if self.local_confidence >= threshold {
            anomaly_votes = 1;
        }

        for (_neighbor_id, score) in &self.neighbor_scores {
            total_votes += 1;
            if *score >= threshold {
                anomaly_votes += 1;
            }
        }

        let vote_ratio = anomaly_votes as f64 / total_votes as f64;
        vote_ratio >= self.byzantine_threshold
    }

    /// Final Byzantine consensus decision
    pub fn byzantine_vote_anomaly(&mut self, local_anomaly: bool, neighbor_scores: HashMap<String, f64>) -> bool {
        // Update internal state
        self.detect_local_anomaly(local_anomaly, if local_anomaly { 0.8 } else { 0.0 });
        self.collect_neighbor_scores(neighbor_scores);

        // Make decision: threshold = 0.5 (50% of nodes must flag anomaly)
        self.count_votes_above_threshold(0.5)
    }
}

pub struct DendriticCell {
    csm: f64,
    semi_mature: f64,
    mature: f64,
    migration_threshold: f64,
}

impl DendriticCell {
    pub fn new(threshold: f64) -> Self {
        DendriticCell {
            csm: 0.0,
            semi_mature: 0.0,
            mature: 0.0,
            migration_threshold: threshold,
        }
    }

    pub fn sample(&mut self, pamp: f64, danger: f64, safe: f64) {
        self.csm += (pamp * 2.0) + (danger * 1.0) + (safe * 2.0);
        self.semi_mature += safe * 3.0;
        self.mature += (pamp * 2.0) + (danger * 1.0) - (safe * 2.0);
    }

    pub fn is_mature(&self) -> bool {
        self.csm >= self.migration_threshold
    }

    pub fn get_context(&self) -> bool {
        self.mature > self.semi_mature
    }
}

pub struct AisImmuneSystem {
    self_space: Vec<Vec<u8>>,
    detectors: Vec<Vec<u8>>,
    r_chunk_size: usize,
    dc_pool: Vec<DendriticCell>,
    k_alpha: f64,
}

impl AisImmuneSystem {
    pub fn new(self_space: Vec<Vec<u8>>, r_chunk_size: usize) -> Self {
        AisImmuneSystem {
            self_space,
            detectors: Vec::new(),
            r_chunk_size,
            dc_pool: Vec::new(),
            k_alpha: 0.0,
        }
    }

    fn r_chunk_match(&self, a: &[u8], b: &[u8]) -> bool {
        let min_len = a.len().min(b.len());
        if min_len < self.r_chunk_size {
            return false;
        }
        for i in 0..=(min_len - self.r_chunk_size) {
            if a[i..i + self.r_chunk_size] == b[i..i + self.r_chunk_size] {
                return true;
            }
        }
        false
    }

    pub fn train_detectors(&mut self, num_detectors: usize, packet_len: usize) {
        let mut rng = rand::rng();
        println!("[IMMUNE] Training {} local detectors in U \\ S space...", num_detectors);
        
        while self.detectors.len() < num_detectors {
            let mut candidate = vec![0u8; packet_len];
            rng.fill(&mut candidate[..]);

            let mut is_self_reactive = false;
            for self_packet in &self.self_space {
                if self.r_chunk_match(&candidate, self_packet) {
                    is_self_reactive = true;
                    break;
                }
            }

            if !is_self_reactive {
                self.detectors.push(candidate);
            }
        }
        println!("[IMMUNE] Successfully trained {} non-self detectors. Edge node immune system active.", self.detectors.len());
    }

    /// Evaluates traffic using the Deterministic Dendritic Cell Algorithm (dDCA).
    pub fn monitor_traffic(&mut self, packet_bytes: &[u8], danger_signal: f64, safe_signal: f64) -> bool {
        // Negative Selection: Acts as the PAMP (Pathogen-Associated Molecular Pattern)
        let mut pamp_signal = 0.0;
        for detector in &self.detectors {
            if self.r_chunk_match(packet_bytes, detector) {
                pamp_signal = 1.0; // Match found in Non-Self space (Anomalous)
                break;
            }
        }

        // Spawn new Dendritic Cells if pool is low
        let mut rng = rand::rng();
        while self.dc_pool.len() < 5 {
            self.dc_pool.push(DendriticCell::new(rng.random_range(10.0..20.0)));
        }

        // Sample environment and migrate mature cells
        let mut active_cells = Vec::new();
        let mut anomalous_presentations = 0.0;
        let mut total_presentations = 0.0;

        for mut cell in self.dc_pool.drain(..) {
            cell.sample(pamp_signal, danger_signal, safe_signal);
            
            if cell.is_mature() {
                total_presentations += 1.0;
                if cell.get_context() {
                    anomalous_presentations += 1.0;
                }
            } else {
                active_cells.push(cell);
            }
        }

        self.dc_pool = active_cells;

        // Calculate Mean Antigen Context (K_alpha)
        if total_presentations > 0.0 {
            self.k_alpha = anomalous_presentations / total_presentations;
        }

        // Quarantine if K_alpha indicates a sustained anomaly context
        self.k_alpha > 0.5
    }
}

// ============================================================================
// PHASE 4A: HYPERDIMENSIONAL COMPUTING (HDC) ANOMALY DETECTION
// ============================================================================

/// A 10,000-dimensional bipolar hypervector represented as a packed bit array.
/// 10000 bits / 64 bits_per_u64 = 157 u64 blocks
pub const HD_DIMENSIONS: usize = 10000;
pub const HD_BLOCKS: usize = 157;

#[derive(Clone, Debug)]
pub struct Hypervector {
    pub bits: Vec<u64>,
}

impl Hypervector {
    pub fn new() -> Self {
        Hypervector { bits: vec![0; HD_BLOCKS] }
    }

    pub fn random(seed: u64) -> Self {
        let mut bits = vec![0; HD_BLOCKS];
        let mut current_seed = seed;
        for block in &mut bits {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            current_seed.hash(&mut hasher);
            current_seed = hasher.finish();
            *block = current_seed;
        }
        Hypervector { bits }
    }

    /// Binding (XOR)
    pub fn bind(&self, other: &Hypervector) -> Hypervector {
        let mut result = Hypervector::new();
        for i in 0..HD_BLOCKS {
            result.bits[i] = self.bits[i] ^ other.bits[i];
        }
        result
    }

    /// Permutation (1-bit cyclic shift)
    pub fn permute(&self) -> Hypervector {
        let mut result = Hypervector::new();
        let mut carry = self.bits[HD_BLOCKS - 1] >> 63;
        for i in 0..HD_BLOCKS {
            let next_carry = self.bits[i] >> 63;
            result.bits[i] = (self.bits[i] << 1) | carry;
            carry = next_carry;
        }
        result
    }

    /// Superposition (Majority Rule)
    pub fn superpose(vectors: &[Hypervector]) -> Hypervector {
        let mut result = Hypervector::new();
        if vectors.is_empty() {
            return result;
        }
        for i in 0..HD_BLOCKS {
            for bit_idx in 0..64 {
                let mut ones = 0;
                for v in vectors {
                    if (v.bits[i] & (1 << bit_idx)) != 0 {
                        ones += 1;
                    }
                }
                if ones > vectors.len() / 2 {
                    result.bits[i] |= 1 << bit_idx;
                }
            }
        }
        result
    }

    pub fn hamming_distance(&self, other: &Hypervector) -> f64 {
        let mut dist = 0;
        for i in 0..HD_BLOCKS {
            dist += (self.bits[i] ^ other.bits[i]).count_ones();
        }
        // Account for exact dimension count instead of full blocks
        let actual_dist = dist as f64 * (HD_DIMENSIONS as f64 / (HD_BLOCKS * 64) as f64);
        actual_dist / HD_DIMENSIONS as f64
    }
}

pub struct HdcAnomalyDetector {
    pub self_baseline: Hypervector,
    pub feature_base_vectors: HashMap<String, Hypervector>,
    pub threshold: f64,
}

impl HdcAnomalyDetector {
    pub fn new() -> Self {
        HdcAnomalyDetector {
            self_baseline: Hypervector::new(),
            feature_base_vectors: HashMap::new(),
            threshold: 0.35, // 35% bit difference triggers anomaly
        }
    }

    fn get_feature_vector(&mut self, feature: &str) -> Hypervector {
        if let Some(v) = self.feature_base_vectors.get(feature) {
            v.clone()
        } else {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            feature.hash(&mut hasher);
            let seed = hasher.finish();
            let new_v = Hypervector::random(seed);
            self.feature_base_vectors.insert(feature.to_string(), new_v.clone());
            new_v
        }
    }

    pub fn encode_telemetry(&mut self, packet_rate: f64, cpu_load: f64, memory_usage: f64) -> Hypervector {
        let rate_bin = (packet_rate / 100.0).min(100.0) as u32;
        let cpu_bin = (cpu_load * 10.0).min(10.0) as u32;
        let mem_bin = (memory_usage * 10.0).min(10.0) as u32;

        let v_rate = self.get_feature_vector("packet_rate").bind(&self.get_feature_vector(&format!("val_{}", rate_bin)));
        let v_cpu = self.get_feature_vector("cpu_load").bind(&self.get_feature_vector(&format!("val_{}", cpu_bin)));
        let v_mem = self.get_feature_vector("mem_usage").bind(&self.get_feature_vector(&format!("val_{}", mem_bin)));

        Hypervector::superpose(&[v_rate, v_cpu, v_mem])
    }

    pub fn train(&mut self, normal_samples: &[(f64, f64, f64)]) {
        let mut encoded_samples = Vec::new();
        for (rate, cpu, mem) in normal_samples {
            encoded_samples.push(self.encode_telemetry(*rate, *cpu, *mem));
        }
        self.self_baseline = Hypervector::superpose(&encoded_samples);
        println!("[HDC IMMUNE] Training complete. Generated Self Baseline Hypervector from {} samples.", normal_samples.len());
    }

    pub fn is_anomalous(&mut self, packet_rate: f64, cpu_load: f64, memory_usage: f64) -> bool {
        let live_state = self.encode_telemetry(packet_rate, cpu_load, memory_usage);
        let distance = live_state.hamming_distance(&self.self_baseline);
        if distance > self.threshold {
            println!("\x1b[31m[HDC IMMUNE] ANOMALY DETECTED! State distance: {:.4} (Threshold: {})\x1b[0m", distance, self.threshold);
            true
        } else {
            false
        }
    }
}
