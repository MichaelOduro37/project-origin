// ============================================================================
// PHASE 17: NETWORK CODING & SLEPIAN-WOLF (CODED TELEMETRY)
// ============================================================================
// Scientific mechanism: Slepian-Wolf Theorem allows distributed correlated
// sources to compress data down to their joint entropy without communicating.
// We simulate this algebraically via XOR-based delta-syndrome coding and 
// Run-Length Encoding (RLE).
//
// By XORing highly correlated telemetry packets (like Tensegrity states), 
// the syndrome becomes overwhelmingly sparse (mostly zeroes). RLE then 
// compresses this sparse matrix to achieve massive theoretical bandwidth savings.
// ============================================================================

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodedTelemetryBatch {
    pub baseline_payload: Vec<u8>,
    pub coded_syndromes: Vec<Vec<u8>>, // RLE compressed XOR deltas
    pub original_sizes: Vec<usize>,
    pub total_uncompressed_bytes: usize,
    pub total_compressed_bytes: usize,
}

pub struct SlepianWolfEncoder;

impl SlepianWolfEncoder {
    /// Encodes a batch of raw string payloads into a CodedTelemetryBatch.
    pub fn encode_batch(payloads: &[String]) -> Option<CodedTelemetryBatch> {
        if payloads.is_empty() {
            return None;
        }

        let baseline = payloads[0].as_bytes().to_vec();
        let mut coded_syndromes = Vec::new();
        let mut original_sizes = Vec::new();
        
        let mut total_uncompressed = baseline.len();
        original_sizes.push(baseline.len());

        let mut prev_payload = baseline.clone();

        for i in 1..payloads.len() {
            let current_payload = payloads[i].as_bytes();
            total_uncompressed += current_payload.len();
            original_sizes.push(current_payload.len());

            // 1. XOR Syndrome Generation
            let max_len = std::cmp::max(prev_payload.len(), current_payload.len());
            let mut syndrome = vec![0u8; max_len];
            
            for j in 0..max_len {
                let b1 = if j < current_payload.len() { current_payload[j] } else { 0 };
                let b2 = if j < prev_payload.len() { prev_payload[j] } else { 0 };
                syndrome[j] = b1 ^ b2;
            }

            // 2. Run-Length Encoding (RLE) to compress the sparse matrix
            let compressed = Self::rle_compress(&syndrome);
            coded_syndromes.push(compressed);
            
            prev_payload = current_payload.to_vec();
        }

        let mut total_compressed = baseline.len();
        for syn in &coded_syndromes {
            total_compressed += syn.len();
        }

        Some(CodedTelemetryBatch {
            baseline_payload: baseline,
            coded_syndromes,
            original_sizes,
            total_uncompressed_bytes: total_uncompressed,
            total_compressed_bytes: total_compressed,
        })
    }

    /// Super simple Run-Length Encoder for zero-heavy streams
    fn rle_compress(data: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();
        let mut i = 0;
        while i < data.len() {
            let current = data[i];
            let mut count = 1;
            while i + 1 < data.len() && data[i + 1] == current && count < 255 {
                count += 1;
                i += 1;
            }
            compressed.push(count as u8);
            compressed.push(current);
            i += 1;
        }
        compressed
    }

    /// Super simple Run-Length Decoder
    pub fn rle_decode(data: &[u8]) -> Vec<u8> {
        let mut decompressed = Vec::new();
        let mut i = 0;
        while i + 1 < data.len() {
            let count = data[i];
            let val = data[i + 1];
            for _ in 0..count {
                decompressed.push(val);
            }
            i += 2;
        }
        decompressed
    }

    /// Reconstructs the original payloads from the encoded batch
    pub fn decode_batch(batch: &CodedTelemetryBatch) -> Vec<String> {
        let mut decoded = Vec::new();
        let mut prev_payload = batch.baseline_payload.clone();
        
        if let Ok(s) = String::from_utf8(prev_payload.clone()) {
            decoded.push(s);
        }

        for (i, compressed_syndrome) in batch.coded_syndromes.iter().enumerate() {
            let syndrome = Self::rle_decode(compressed_syndrome);
            let orig_len = batch.original_sizes[i + 1];
            
            let mut current_payload = vec![0u8; orig_len];
            for j in 0..orig_len {
                let b2 = if j < prev_payload.len() { prev_payload[j] } else { 0 };
                let syn_byte = if j < syndrome.len() { syndrome[j] } else { 0 };
                current_payload[j] = syn_byte ^ b2;
            }
            
            if let Ok(s) = String::from_utf8(current_payload.clone()) {
                decoded.push(s);
            }
            prev_payload = current_payload;
        }

        decoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slepian_wolf_coding() {
        let payloads = vec![
            "ORIGIN_TELEMETRY: TENSEGRITY=0.98 SNN=12.4".to_string(),
            "ORIGIN_TELEMETRY: TENSEGRITY=0.98 SNN=12.5".to_string(),
            "ORIGIN_TELEMETRY: TENSEGRITY=0.99 SNN=12.5".to_string(),
        ];

        let batch = SlepianWolfEncoder::encode_batch(&payloads).unwrap();
        
        // Due to high correlation, compressed size should be significantly smaller
        // than raw bytes if we had large/more payloads.
        println!("Uncompressed: {}", batch.total_uncompressed_bytes);
        println!("Compressed: {}", batch.total_compressed_bytes);

        let decoded = SlepianWolfEncoder::decode_batch(&batch);
        
        assert_eq!(decoded.len(), 3);
        assert_eq!(decoded[0], payloads[0]);
        assert_eq!(decoded[1], payloads[1]);
        assert_eq!(decoded[2], payloads[2]);
    }
}
