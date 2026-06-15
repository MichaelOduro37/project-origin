// ============================================================================
// PHASE 29: INFORMATION BOTTLENECK METHOD (TELEMETRY COMPRESSION)
// ============================================================================
// Scientific mechanism: Formulated by Naftali Tishby, the Information
// Bottleneck method minimizes mutual information I(X; T) while maximizing 
// relevance I(T; Y) based on a Lagrangian multiplier beta.
//
// Origin uses this to mathematically compress massive high-dimensional 
// node telemetry arrays into tiny "bottleneck" vectors containing ONLY
// the actionable data, perfectly preserving Swarm awareness while 
// discarding megabytes of redundant noise.
// ============================================================================

pub struct IBCompressor {
    pub beta: f64, // Tradeoff parameter: High beta = favor relevance, Low beta = favor extreme compression
    pub threshold: f64,
}

impl IBCompressor {
    pub fn new(beta: f64, threshold: f64) -> Self {
        Self { beta, threshold }
    }

    /// Compresses a raw telemetry vector `x` into a bottleneck vector `t`.
    /// `relevance_y` represents the pre-computed mutual information correlation
    /// of each feature in `x` to a target state (like anomaly/failure).
    /// Returns (Compressed Vector T, Original Size, Compressed Size)
    pub fn compress_telemetry(&self, raw_x: &[f64], relevance_y: &[f64]) -> (Vec<f64>, usize, usize) {
        assert_eq!(raw_x.len(), relevance_y.len(), "Dimension mismatch between raw telemetry and relevance vector");

        let mut t = Vec::new();
        
        for i in 0..raw_x.len() {
            // L = I(X;T) - beta * I(T;Y) proxy
            // We keep the feature if its absolute relevance * beta overcomes the compression threshold penalty
            let feature_relevance = relevance_y[i].abs() * self.beta;
            
            if feature_relevance > self.threshold {
                // Feature is deemed "relevant" and passes through the bottleneck
                t.push(raw_x[i]);
            } else {
                // Feature is redundant noise; discarded (mathematically compressed to 0)
                // In a real sparse format, we wouldn't even send the 0.
            }
        }

        (t.clone(), raw_x.len(), t.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_information_bottleneck_compression() {
        let raw_telemetry = vec![0.5, 0.9, 0.1, 0.8, 0.2, 0.4, 0.99, 0.05];
        // Only features 1, 3, and 6 are highly relevant to the target state
        let relevance_y = vec![0.01, 0.95, 0.02, 0.88, 0.00, 0.10, 0.92, 0.01];

        // High Beta: favors relevance over compression
        let compressor_high_beta = IBCompressor::new(1.0, 0.5);
        let (_, orig, comp_high) = compressor_high_beta.compress_telemetry(&raw_telemetry, &relevance_y);
        
        assert_eq!(orig, 8);
        assert_eq!(comp_high, 3); // Kept the 3 highly relevant features

        // Low Beta: favors extreme compression over relevance (lossy)
        let compressor_low_beta = IBCompressor::new(0.1, 0.5);
        let (_, _, comp_low) = compressor_low_beta.compress_telemetry(&raw_telemetry, &relevance_y);
        
        assert_eq!(comp_low, 0); // Beta was too low, everything dropped
    }
}
