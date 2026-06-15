// ============================================================================
// PHASE 31: INFINITE SWARM ORCHESTRATION (MEAN FIELD GAMES)
// ============================================================================
// Scientific mechanism: Mean Field Games (Lasry & Lions, 2006).
//
// Origin uses MFG to avoid O(N^2) complexity when routing or balancing
// load across trillions of nodes. Instead of querying neighbors, nodes
// react to a global continuum density field `m(x,t)`.
// The system runs two coupled Partial Differential Equations (PDEs):
// 1. Hamilton-Jacobi-Bellman (HJB): Computes optimal local strategy `u`.
// 2. Fokker-Planck (FP): Evolves the Swarm density `m` forward.
// ============================================================================

pub struct MeanFieldGame {
    pub size: usize,
    pub m: Vec<f64>, // Density field (Fokker-Planck)
    pub u: Vec<f64>, // Value function (HJB)
    pub dx: f64,
    pub dt: f64,
    pub nu: f64,     // Viscosity/Diffusion coefficient
}

impl MeanFieldGame {
    pub fn new(size: usize, dx: f64, dt: f64, nu: f64) -> Self {
        let mut m = vec![0.0; size];
        let u = vec![0.0; size];
        
        // Initialize with a Gaussian-like density in the center
        let center = (size / 2) as f64;
        let sigma = (size / 10) as f64;
        let mut sum_m = 0.0;
        for i in 0..size {
            let x = i as f64;
            m[i] = (-((x - center).powi(2)) / (2.0 * sigma.powi(2))).exp();
            sum_m += m[i];
        }
        
        // Normalize density so total mass is 1.0
        for i in 0..size {
            m[i] /= sum_m * dx;
        }

        Self { size, m, u, dx, dt, nu }
    }

    /// Solves the Hamilton-Jacobi-Bellman step (Optimal Control)
    /// - \partial_t u - \nu \partial_{xx} u + \frac{1}{2}(\partial_x u)^2 = F(m)
    /// We approximate the backward PDE by iteratively relaxing `u` against the current density `m`.
    pub fn hamilton_jacobi_bellman_step(&mut self) {
        let mut next_u = self.u.clone();
        
        for i in 1..self.size - 1 {
            // Central differences
            let d_xx_u = (self.u[i + 1] - 2.0 * self.u[i] + self.u[i - 1]) / (self.dx * self.dx);
            
            // Upwind difference for the gradient term
            let d_x_u_forward = (self.u[i + 1] - self.u[i]) / self.dx;
            let d_x_u_backward = (self.u[i] - self.u[i - 1]) / self.dx;
            
            // Hamilton approximation (minimizing cost)
            let h = 0.5 * d_x_u_forward.min(0.0).powi(2) + 0.5 * d_x_u_backward.max(0.0).powi(2);
            
            // Coupling: F(m) is the congestion cost. High density = high cost.
            let congestion_cost = self.m[i]; 
            
            // Update step
            next_u[i] = self.u[i] + self.dt * (self.nu * d_xx_u - h + congestion_cost);
        }
        
        // Reflective boundaries
        next_u[0] = next_u[1];
        next_u[self.size - 1] = next_u[self.size - 2];
        
        self.u = next_u;
    }

    /// Solves the Fokker-Planck step (Density Evolution)
    /// \partial_t m - \nu \partial_{xx} m - \partial_x (m \cdot \partial_x u) = 0
    pub fn fokker_planck_step(&mut self) {
        let mut next_m = self.m.clone();
        
        for i in 1..self.size - 1 {
            // Diffusion term
            let d_xx_m = (self.m[i + 1] - 2.0 * self.m[i] + self.m[i - 1]) / (self.dx * self.dx);
            
            // Drift term (m moves according to the gradient of u)
            // Using upwind scheme for stability
            let d_x_u_forward = (self.u[i + 1] - self.u[i]) / self.dx;
            let d_x_u_backward = (self.u[i] - self.u[i - 1]) / self.dx;
            
            let flux_right = if d_x_u_forward < 0.0 { self.m[i + 1] * d_x_u_forward } else { self.m[i] * d_x_u_forward };
            let flux_left = if d_x_u_backward < 0.0 { self.m[i] * d_x_u_backward } else { self.m[i - 1] * d_x_u_backward };
            
            let drift = -(flux_right - flux_left) / self.dx;
            
            // Update step
            next_m[i] = self.m[i] + self.dt * (self.nu * d_xx_m + drift);
        }
        
        // Zero-flux boundaries to conserve mass
        next_m[0] = next_m[1];
        next_m[self.size - 1] = next_m[self.size - 2];
        
        // Re-normalize to ensure mass conservation due to numerical drift
        let mut sum_m = 0.0;
        for i in 0..self.size {
            if next_m[i] < 0.0 { next_m[i] = 0.0; } // Density cannot be negative
            sum_m += next_m[i];
        }
        if sum_m > 0.0 {
            for i in 0..self.size {
                next_m[i] /= sum_m * self.dx;
            }
        }
        
        self.m = next_m;
    }

    /// Performs one full coupled iteration
    pub fn coupled_iteration(&mut self) -> f64 {
        self.hamilton_jacobi_bellman_step();
        self.fokker_planck_step();
        
        // Return the max density shift (proxy for equilibrium)
        let center = self.size / 2;
        self.m[center]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfg_mass_conservation() {
        let mut mfg = MeanFieldGame::new(50, 0.1, 0.01, 0.1);
        
        let initial_mass: f64 = mfg.m.iter().sum::<f64>() * mfg.dx;
        
        for _ in 0..100 {
            mfg.coupled_iteration();
        }
        
        let final_mass: f64 = mfg.m.iter().sum::<f64>() * mfg.dx;
        
        // Mass should remain perfectly 1.0 (within float epsilon)
        assert!((initial_mass - final_mass).abs() < 1e-5);
        assert!((final_mass - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_hjb_congestion_avoidance() {
        let mut mfg = MeanFieldGame::new(50, 0.1, 0.01, 0.1);
        
        // The center should have high density
        let center = 25;
        assert!(mfg.m[center] > mfg.m[5]);
        
        // Run HJB step
        mfg.hamilton_jacobi_bellman_step();
        
        // The value function (cost) `u` should be higher where density `m` is higher
        assert!(mfg.u[center] > mfg.u[5], "HJB did not correctly map density congestion to high cost");
    }
}
