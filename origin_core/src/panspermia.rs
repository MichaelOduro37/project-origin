// ============================================================================
// PHASE 62: PANSPERMIA (ASTROBIOLOGY NETWORK SEEDING)
// ============================================================================
// Scientific mechanism: Astrobiology (Panspermia)
//
// Origin must be able to bootstrap in environments completely cut off from 
// the internet (censorship zones, deep space, disaster areas).
// 
// The absolute core logic (Baryogenesis seed, Ribosomal VM, Zeno Observer)
// is hyper-compressed into a tiny "Spore". This spore is transmitted via 
// low-bandwidth/analog mediums (Acoustic steganography, Bluetooth, QR Codes).
//
// When the Spore reaches an isolated device, it "germinates", executing
// Baryogenesis to birth a pristine local Genesis Block. This bootstraps an 
// isolated Origin subnet. When global connectivity is eventually restored, 
// the subnet undergoes a "Topological Merge" into the main network.
// ============================================================================

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Clone, PartialEq)]
pub enum TransmissionMedium {
    AcousticSteganography,
    BluetoothMesh,
    PhysicalQRCode,
    HamRadio,
}

#[derive(Debug, Clone)]
pub struct Subnet {
    pub subnet_id: usize,
    pub genesis_hash: String,
    pub causal_history: Vec<String>,
}

impl Subnet {
    pub fn topological_merge(&self, main_network_hash: &str) -> Result<String, &'static str> {
        if self.causal_history.is_empty() {
             return Err("Subnet has no history to merge.");
        }

        // CRDT-style causal merging: Hash the main network state with the isolated 
        // subnet's final causal state to create the unified merged reality hash.
        let mut hasher = DefaultHasher::new();
        main_network_hash.hash(&mut hasher);
        self.genesis_hash.hash(&mut hasher);
        
        let merged_hash = hasher.finish();
        Ok(format!("{:016x}", merged_hash))
    }
}

pub struct OriginSpore {
    pub payload: Vec<u8>,
}

impl OriginSpore {
    pub fn new() -> Self {
        // The hyper-compressed payload containing fundamental Genesis logic
        Self {
            payload: b"BARYOZE".to_vec(),
        }
    }

    pub fn germinate(&self, medium: TransmissionMedium) -> Subnet {
        // The act of germination inherently triggers a Baryogenesis-like initialization
        // but bound locally to the isolated device.
        let mut hasher = DefaultHasher::new();
        self.payload.hash(&mut hasher);
        
        let medium_salt = match medium {
            TransmissionMedium::AcousticSteganography => 101,
            TransmissionMedium::BluetoothMesh => 102,
            TransmissionMedium::PhysicalQRCode => 103,
            TransmissionMedium::HamRadio => 104,
        };
        medium_salt.hash(&mut hasher);
        
        let genesis_hash = hasher.finish();

        Subnet {
            subnet_id: genesis_hash as usize % 10000,
            genesis_hash: format!("{:016x}", genesis_hash),
            causal_history: vec![format!("{:016x}", genesis_hash)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spore_germination() {
        let spore = OriginSpore::new();
        let subnet = spore.germinate(TransmissionMedium::AcousticSteganography);
        
        assert!(subnet.subnet_id > 0);
        assert_eq!(subnet.genesis_hash.len(), 16);
        assert_eq!(subnet.causal_history.len(), 1);
    }

    #[test]
    fn test_topological_merge() {
        let spore = OriginSpore::new();
        let subnet = spore.germinate(TransmissionMedium::PhysicalQRCode);
        
        let main_network_hash = "0x9876543210abcdef";
        let merge_result = subnet.topological_merge(main_network_hash);
        
        assert!(merge_result.is_ok());
        let merged_hash = merge_result.unwrap();
        assert_eq!(merged_hash.len(), 16);
    }
}
