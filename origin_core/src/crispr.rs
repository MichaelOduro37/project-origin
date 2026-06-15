// ============================================================================
// PHASE 11: CRISPR-CAS9 ADAPTIVE SWARM IMMUNITY
// ============================================================================
// Biological mechanism: CRISPR-Cas9 is an adaptive immune system in bacteria.
// It stores fragments of viral DNA in a CRISPR array. When the virus attacks
// again, Cas9 uses the guide RNA (sgRNA) to slice the viral DNA.
//
// Application: The Origin Swarm extracts byte signatures from malicious
// payloads, shares them via sgRNA broadcasts, and slices matching packets
// at the socket layer before they consume CPU cycles.
// ============================================================================

use std::sync::{Mutex, OnceLock};
use std::collections::HashSet;

pub struct CRISPRArray {
    /// The "Spacers": digital signatures (e.g., base64 string or byte sequence hashes) 
    /// of known malicious payloads.
    spacers: HashSet<String>,
}

impl CRISPRArray {
    pub fn new() -> Self {
        Self {
            spacers: HashSet::new(),
        }
    }

    /// Integrate a new viral signature into the CRISPR array (memory)
    pub fn add_spacer(&mut self, signature: String) -> bool {
        self.spacers.insert(signature) // Returns true if it was a new signature
    }

    /// Check if the incoming payload contains any known viral signature
    pub fn scan_payload(&self, payload: &str) -> Option<String> {
        for spacer in &self.spacers {
            if payload.contains(spacer) {
                return Some(spacer.clone());
            }
        }
        None
    }
    
    pub fn get_all_spacers(&self) -> Vec<String> {
        self.spacers.iter().cloned().collect()
    }
}

pub fn global_crispr() -> &'static Mutex<CRISPRArray> {
    static CRISPR: OnceLock<Mutex<CRISPRArray>> = OnceLock::new();
    CRISPR.get_or_init(|| Mutex::new(CRISPRArray::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crispr_array_cleavage() {
        let mut array = CRISPRArray::new();
        array.add_spacer("MALICIOUS_HACK_V1".to_string());
        
        let safe_payload = "ORIGIN_CHAT:NodeX:Hello World!";
        let bad_payload = "ORIGIN_HOLO:NodeX:MALICIOUS_HACK_V1_EXPLOIT";
        
        assert_eq!(array.scan_payload(safe_payload), None);
        assert_eq!(array.scan_payload(bad_payload), Some("MALICIOUS_HACK_V1".to_string()));
    }
}
