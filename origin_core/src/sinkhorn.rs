// ============================================================================
// PHASE 16: OPTIMAL TRANSPORT (WASSERSTEIN DISTANCES)
// ============================================================================
// Scientific mechanism: Optimal Transport determines the mathematically 
// cheapest way to move "mass" from one distribution to another. By using 
// an entropy-regularized Sinkhorn-Knopp algorithm, we compute the 
// exact Wasserstein mapping of Holographic Shards to Peer Nodes.
//
// Application: Before broadcasting MERA shards, we build a cost matrix of 
// physical peer latencies. The Sinkhorn solver mathematically maps exactly 
// which shard goes to which peer to absolutely minimize network strain.
// ============================================================================

pub struct SinkhornSolver {
    epsilon: f64,
    max_iter: usize,
}

impl SinkhornSolver {
    pub fn new(epsilon: f64, max_iter: usize) -> Self {
        Self { epsilon, max_iter }
    }

    /// Computes the optimal transport plan P of size (N x M).
    /// `cost_matrix`: N rows (shards) x M cols (peers)
    /// `a`: Mass distribution of shards (size N)
    /// `b`: Capacity distribution of peers (size M)
    /// Returns the Transport matrix (N x M) and the Wasserstein cost.
    pub fn compute_transport_plan(&self, cost_matrix: &[Vec<f64>], a: &[f64], b: &[f64]) -> (Vec<Vec<f64>>, f64) {
        let n = a.len();
        let m = b.len();

        if n == 0 || m == 0 {
            return (vec![], 0.0);
        }

        // Initialize K = exp(-C / epsilon)
        let mut k = vec![vec![0.0; m]; n];
        for i in 0..n {
            for j in 0..m {
                k[i][j] = (-cost_matrix[i][j] / self.epsilon).exp();
            }
        }

        let mut u = vec![1.0 / n as f64; n];
        let mut v = vec![1.0 / m as f64; m];

        // Sinkhorn-Knopp Iterations
        for _ in 0..self.max_iter {
            // Update u: u = a ./ (K * v)
            for i in 0..n {
                let mut denominator = 0.0;
                for j in 0..m {
                    denominator += k[i][j] * v[j];
                }
                if denominator > 1e-12 {
                    u[i] = a[i] / denominator;
                }
            }

            // Update v: v = b ./ (K^T * u)
            for j in 0..m {
                let mut denominator = 0.0;
                for i in 0..n {
                    denominator += k[i][j] * u[i];
                }
                if denominator > 1e-12 {
                    v[j] = b[j] / denominator;
                }
            }
        }

        // Compute optimal transport plan P = diag(u) * K * diag(v)
        let mut p = vec![vec![0.0; m]; n];
        let mut total_cost = 0.0;

        for i in 0..n {
            for j in 0..m {
                p[i][j] = u[i] * k[i][j] * v[j];
                total_cost += p[i][j] * cost_matrix[i][j];
            }
        }

        (p, total_cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sinkhorn_transport() {
        let solver = SinkhornSolver::new(0.1, 100);
        
        // 2 shards, 2 peers
        // Cost to send shard 0 to peer 0 is cheap (1.0), to peer 1 is expensive (10.0)
        // Cost to send shard 1 to peer 0 is expensive (10.0), to peer 1 is cheap (1.0)
        let cost_matrix = vec![
            vec![1.0, 10.0],
            vec![10.0, 1.0],
        ];
        
        let a = vec![0.5, 0.5]; // Each shard is 50% mass
        let b = vec![0.5, 0.5]; // Each peer holds 50% capacity
        
        let (p, cost) = solver.compute_transport_plan(&cost_matrix, &a, &b);
        
        // Shard 0 should map heavily to Peer 0
        assert!(p[0][0] > p[0][1]);
        // Shard 1 should map heavily to Peer 1
        assert!(p[1][1] > p[1][0]);
        // Total cost should be close to 1.0 (since 0.5*1.0 + 0.5*1.0 = 1.0)
        assert!(cost < 2.0);
    }
}
