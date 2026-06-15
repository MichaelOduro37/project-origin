// ============================================================================
// PHASE 20: SPARSE REPRESENTATIONS & COMPRESSED SENSING
// ============================================================================
// Scientific mechanism: As the Swarm generates immense arrays of telemetry 
// (thermodynamics, curvature, SNN potentials), bandwidth becomes the limit.
// 
// According to Compressed Sensing (CS) and the Johnson-Lindenstrauss lemma,
// if we multiply a high-dimensional state vector by a chaotic, pseudo-random
// Gaussian measurement matrix (which satisfies the Restricted Isometry Property),
// we compress it into a tiny "sketch".
//
// Crucially, the Euclidean distance between any two states is mathematically 
// preserved in their compressed sketches. Thus, the Swarm can analyze anomalies 
// without ever decompressing the data!
// ============================================================================

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompressedTelemetrySnapshot {
    pub original_dim: usize,
    pub compressed_dim: usize,
    pub sketch: Vec<f64>,
}

pub struct MeasurementMatrix {
    pub input_dim: usize,
    pub output_dim: usize,
    // Flattened M x N matrix. (Row-major: row * input_dim + col)
    matrix: Vec<f64>, 
}

impl MeasurementMatrix {
    /// Creates a deterministic pseudo-random Gaussian measurement matrix.
    /// In a real global Swarm, nodes agree on a shared seed to generate identical matrices.
    pub fn new(input_dim: usize, output_dim: usize, seed: u64) -> Self {
        let mut matrix = Vec::with_capacity(input_dim * output_dim);
        
        // Simple deterministic pseudo-random generator (LCG) for Gaussian approximation
        // Box-Muller transform
        let mut state = seed;
        let mut next_rand = || -> f64 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let u1 = ((state >> 32) as f64) / (u32::MAX as f64);
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let u2 = ((state >> 32) as f64) / (u32::MAX as f64);
            
            // Box-Muller to get standard normal N(0, 1)
            (-2.0 * u1.max(1e-10).ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
        };

        // Populate the matrix with Gaussian values scaled by 1/sqrt(M)
        // This scaling ensures the Johnson-Lindenstrauss norm preservation.
        let scale = 1.0 / (output_dim as f64).sqrt();
        for _ in 0..(input_dim * output_dim) {
            matrix.push(next_rand() * scale);
        }

        Self {
            input_dim,
            output_dim,
            matrix,
        }
    }

    /// Compresses a high-dimensional signal into a sparse sketch via Matrix Multiplication.
    /// y = Phi * x
    pub fn compress(&self, signal: &[f64]) -> Vec<f64> {
        assert_eq!(signal.len(), self.input_dim, "Signal dimension must match input_dim");
        
        let mut sketch = vec![0.0; self.output_dim];
        
        for i in 0..self.output_dim {
            let mut sum = 0.0;
            for j in 0..self.input_dim {
                sum += self.matrix[i * self.input_dim + j] * signal[j];
            }
            sketch[i] = sum;
        }
        
        sketch
    }

    /// Calculates the true Euclidean distance between two dense signals.
    pub fn true_distance(a: &[f64], b: &[f64]) -> f64 {
        assert_eq!(a.len(), b.len());
        let mut sum = 0.0;
        for i in 0..a.len() {
            let diff = a[i] - b[i];
            sum += diff * diff;
        }
        sum.sqrt()
    }

    /// Calculates the approximate distance directly in the compressed domain.
    /// Due to the J-L Lemma, this tightly bounds the true_distance.
    pub fn compressed_distance(sketch_a: &[f64], sketch_b: &[f64]) -> f64 {
        assert_eq!(sketch_a.len(), sketch_b.len());
        let mut sum = 0.0;
        for i in 0..sketch_a.len() {
            let diff = sketch_a[i] - sketch_b[i];
            sum += diff * diff;
        }
        sum.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_johnson_lindenstrauss_preservation() {
        let input_dim = 1000;
        let output_dim = 50; // Compressing by 20x!
        
        let phi = MeasurementMatrix::new(input_dim, output_dim, 42);

        // Generate two dense signals
        let mut sig_a = vec![0.0; input_dim];
        let mut sig_b = vec![0.0; input_dim];

        for i in 0..input_dim {
            sig_a[i] = (i as f64).sin();
            sig_b[i] = (i as f64).cos() * 0.5;
        }

        // True distance
        let true_dist = MeasurementMatrix::true_distance(&sig_a, &sig_b);

        // Compress!
        let sketch_a = phi.compress(&sig_a);
        let sketch_b = phi.compress(&sig_b);

        // Compressed distance
        let comp_dist = MeasurementMatrix::compressed_distance(&sketch_a, &sketch_b);

        // The J-L Lemma guarantees comp_dist is approximately equal to true_dist
        let error_margin = (true_dist - comp_dist).abs() / true_dist;
        
        // In highly chaotic systems, an error < 15% is extremely good for a 20x compression ratio.
        assert!(error_margin < 0.15, "Error margin {} is too high. J-L lemma failed.", error_margin);
    }
}
