// ============================================================================
// PHASE 30: THE NATIVE AI SYSTEM (SECURE FEDERATED LEARNING VIA SMPC)
// ============================================================================
// Scientific mechanism: Adi Shamir's Secret Sharing (1979) and Secure Multi-
// Party Computation. 
//
// To fulfill Prime Directive Rule 4 (Native AI System), Origin implements
// a globally decentralized Swarm intelligence. To ensure absolute privacy,
// nodes do not broadcast their AI gradients. They split gradients into 
// polynomial shares. Peers sum these shares (SMPC homomorphic addition).
// When reconstructed via Lagrange Interpolation, the Swarm obtains the 
// exact global AI update without ever seeing an individual node's data.
// ============================================================================

// A Mersenne Prime for our finite field arithmetic
const PRIME: i64 = 2147483647; // 2^31 - 1

pub struct ShamirSecretSharing {
    pub n: usize, // Total number of shares to generate
    pub k: usize, // Threshold number of shares needed to reconstruct
}

impl ShamirSecretSharing {
    pub fn new(n: usize, k: usize) -> Self {
        assert!(k <= n, "Threshold k cannot be greater than total shares n");
        Self { n, k }
    }

    /// Safely computes (a + b) mod PRIME
    fn add_mod(a: i64, b: i64) -> i64 {
        (a % PRIME + b % PRIME) % PRIME
    }

    /// Safely computes (a * b) mod PRIME
    fn mul_mod(a: i64, b: i64) -> i64 {
        ((a % PRIME) * (b % PRIME)) % PRIME
    }

    /// Computes (base^exp) mod PRIME
    fn pow_mod(base: i64, mut exp: i64) -> i64 {
        let mut res = 1;
        let mut b = base % PRIME;
        while exp > 0 {
            if exp % 2 == 1 {
                res = Self::mul_mod(res, b);
            }
            b = Self::mul_mod(b, b);
            exp /= 2;
        }
        res
    }

    /// Computes the modular inverse using Fermat's Little Theorem (since PRIME is prime)
    fn inv_mod(a: i64) -> i64 {
        // Handle negative numbers in modulo arithmetic
        let a_pos = ((a % PRIME) + PRIME) % PRIME;
        Self::pow_mod(a_pos, PRIME - 2)
    }

    /// Splits a secret into `n` shares, requiring `k` to reconstruct.
    /// Returns a vector of (x, y) coordinates.
    pub fn split_secret(&self, secret: i64) -> Vec<(i64, i64)> {
        // The polynomial: f(x) = secret + a_1*x + a_2*x^2 + ... + a_{k-1}*x^{k-1}
        let mut coefficients = vec![secret % PRIME];
        for _ in 1..self.k {
            let r = (rand::random::<u64>() % (PRIME as u64 - 1)) as i64 + 1;
            coefficients.push(r);
        }

        let mut shares = Vec::new();
        for x in 1..=(self.n as i64) {
            let mut y = 0;
            // Evaluate polynomial at x: y = f(x) mod PRIME
            for (i, &coeff) in coefficients.iter().enumerate() {
                let term = Self::mul_mod(coeff, Self::pow_mod(x, i as i64));
                y = Self::add_mod(y, term);
            }
            shares.push((x, y));
        }

        shares
    }

    /// Reconstructs the secret from at least `k` shares using Lagrange Interpolation at x=0
    pub fn reconstruct_secret(shares: &[(i64, i64)]) -> i64 {
        let mut secret = 0;

        for i in 0..shares.len() {
            let (x_i, y_i) = shares[i];
            let mut numerator = 1;
            let mut denominator = 1;

            for j in 0..shares.len() {
                if i == j { continue; }
                let (x_j, _) = shares[j];

                // numerator = numerator * (0 - x_j)
                let num_term = ((0 - x_j) % PRIME + PRIME) % PRIME;
                numerator = Self::mul_mod(numerator, num_term);

                // denominator = denominator * (x_i - x_j)
                let den_term = ((x_i - x_j) % PRIME + PRIME) % PRIME;
                denominator = Self::mul_mod(denominator, den_term);
            }

            let lagrange_basis = Self::mul_mod(numerator, Self::inv_mod(denominator));
            let term = Self::mul_mod(y_i, lagrange_basis);
            secret = Self::add_mod(secret, term);
        }

        secret
    }

    /// Homomorphic Addition: Given two sets of shares (evaluated at the same X coordinates),
    /// adds their Y coordinates. The resulting shares will reconstruct the sum of the original secrets.
    pub fn aggregate_shares(shares_a: &[(i64, i64)], shares_b: &[(i64, i64)]) -> Vec<(i64, i64)> {
        assert_eq!(shares_a.len(), shares_b.len());
        let mut aggregated = Vec::new();
        for i in 0..shares_a.len() {
            assert_eq!(shares_a[i].0, shares_b[i].0, "X coordinates must match for homomorphic addition");
            let sum_y = Self::add_mod(shares_a[i].1, shares_b[i].1);
            aggregated.push((shares_a[i].0, sum_y));
        }
        aggregated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shamir_reconstruction() {
        let sss = ShamirSecretSharing::new(5, 3); // 5 shares, need 3
        let secret = 42069;
        
        let shares = sss.split_secret(secret);
        
        // Take exactly k shares (e.g. shares 0, 2, 4)
        let subset = vec![shares[0], shares[2], shares[4]];
        let reconstructed = ShamirSecretSharing::reconstruct_secret(&subset);
        
        assert_eq!(secret, reconstructed, "Lagrange interpolation failed to reconstruct the secret");
    }

    #[test]
    fn test_smpc_homomorphic_addition() {
        let sss = ShamirSecretSharing::new(3, 2);
        
        // Node A has gradient update 100
        let secret_a = 100;
        let shares_a = sss.split_secret(secret_a);

        // Node B has gradient update 250
        let secret_b = 250;
        let shares_b = sss.split_secret(secret_b);

        // Peers aggregate the shares blindly
        let aggregated_shares = ShamirSecretSharing::aggregate_shares(&shares_a, &shares_b);

        // The Swarm reconstructs the global AI gradient
        let reconstructed_sum = ShamirSecretSharing::reconstruct_secret(&aggregated_shares);

        assert_eq!(secret_a + secret_b, reconstructed_sum, "SMPC Homomorphic addition failed");
    }
}
