// ============================================================================
// PHASE 32: SWARM GLOBAL MEMORY (SPARSE DISTRIBUTED MEMORY)
// ============================================================================
// Scientific mechanism: Sparse Distributed Memory (Pentti Kanerva, 1988).
//
// Origin uses SDM as its biologically-inspired Decentralized File System.
// Instead of storing data at exact addresses (like RAM or DHTs), Origin 
// distributes data across a massively high-dimensional boolean space. 
// A write distributes the data vector to all Origin "hard locations" within 
// a Hamming radius of the address. A read pools all hard locations within 
// the radius and reconstructs the data via a statistical majority vote.
// This guarantees that Swarm memory survives catastrophic node failure and 
// can be retrieved even if the query address is corrupted with noise.
// ============================================================================

pub const VECTOR_SIZE: usize = 256;

#[derive(Clone, Debug, PartialEq)]
pub struct BitVector {
    pub bits: Vec<bool>,
}

impl BitVector {
    pub fn new_random() -> Self {
        let mut bits = Vec::with_capacity(VECTOR_SIZE);
        for _ in 0..VECTOR_SIZE {
            bits.push(rand::random::<bool>());
        }
        Self { bits }
    }

    pub fn new_empty() -> Self {
        Self { bits: vec![false; VECTOR_SIZE] }
    }

    pub fn hamming_distance(&self, other: &BitVector) -> usize {
        let mut dist = 0;
        for i in 0..VECTOR_SIZE {
            if self.bits[i] != other.bits[i] {
                dist += 1;
            }
        }
        dist
    }

    /// Flips a given percentage of bits (noise simulation)
    pub fn apply_noise(&self, noise_ratio: f64) -> Self {
        let mut noisy = self.clone();
        for i in 0..VECTOR_SIZE {
            if rand::random::<f64>() < noise_ratio {
                noisy.bits[i] = !noisy.bits[i];
            }
        }
        noisy
    }
}

pub struct HardLocation {
    pub address: BitVector,
    pub counters: Vec<i32>,
}

impl HardLocation {
    pub fn new(address: BitVector) -> Self {
        Self {
            address,
            counters: vec![0; VECTOR_SIZE],
        }
    }
}

pub struct SparseDistributedMemory {
    pub hard_locations: Vec<HardLocation>,
    pub activation_radius: usize,
}

impl SparseDistributedMemory {
    pub fn new(num_hard_locations: usize, activation_radius: usize) -> Self {
        let mut hard_locations = Vec::with_capacity(num_hard_locations);
        for _ in 0..num_hard_locations {
            hard_locations.push(HardLocation::new(BitVector::new_random()));
        }
        Self {
            hard_locations,
            activation_radius,
        }
    }

    /// Writes data to all Swarm nodes within the Hamming radius of the address
    pub fn write(&mut self, address: &BitVector, data: &BitVector) -> usize {
        let mut nodes_activated = 0;
        for hl in self.hard_locations.iter_mut() {
            if hl.address.hamming_distance(address) <= self.activation_radius {
                nodes_activated += 1;
                for i in 0..VECTOR_SIZE {
                    if data.bits[i] {
                        hl.counters[i] += 1;
                    } else {
                        hl.counters[i] -= 1;
                    }
                }
            }
        }
        nodes_activated
    }

    /// Reads data by pooling from all Swarm nodes within the Hamming radius and taking a majority vote
    pub fn read(&self, address: &BitVector) -> (BitVector, usize) {
        let mut sums = vec![0; VECTOR_SIZE];
        let mut nodes_activated = 0;

        for hl in self.hard_locations.iter() {
            if hl.address.hamming_distance(address) <= self.activation_radius {
                nodes_activated += 1;
                for i in 0..VECTOR_SIZE {
                    sums[i] += hl.counters[i];
                }
            }
        }

        let mut reconstructed_data = BitVector::new_empty();
        for i in 0..VECTOR_SIZE {
            reconstructed_data.bits[i] = sums[i] > 0;
        }

        (reconstructed_data, nodes_activated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdm_associative_recall_with_noise() {
        // Create an SDM lattice with 1,000 "hard location" nodes
        // Using a radius that activates roughly 5-10% of nodes
        // (For 256 bits, mean distance is 128. Radius of 115 gives good activation spread)
        let mut sdm = SparseDistributedMemory::new(1000, 115);

        let target_address = BitVector::new_random();
        let target_data = BitVector::new_random();

        // Write data to the Swarm memory
        let activated_writes = sdm.write(&target_address, &target_data);
        assert!(activated_writes > 0, "No nodes activated during write!");

        // Add 10% bit-flip noise to the target address to simulate retrieval via an imperfect/corrupted cue
        let noisy_query_address = target_address.apply_noise(0.10);
        
        assert_ne!(target_address, noisy_query_address, "Noisy query should be different");

        // Read data back from the Swarm using the NOISY address
        let (reconstructed_data, activated_reads) = sdm.read(&noisy_query_address);
        assert!(activated_reads > 0, "No nodes activated during read!");

        // Mathematically, the reconstructed data should perfectly match the original data
        // due to the statistical majority vote of the overlapping activation radii.
        assert_eq!(target_data, reconstructed_data, "Associative recall failed to reconstruct original data");
    }
}
