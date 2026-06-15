// ============================================================================
// PHASE 13: GAUSS-BONNET TOPOLOGICAL CURVATURE REGULATION
// ============================================================================
// Scientific mechanism: The Gauss-Bonnet Theorem relates the geometry (curvature)
// of a manifold to its topology (Euler characteristic).
//
// Application: When local network congestion causes high traffic "curvature" (K),
// the node autonomously spawns temporary proxy ports ("Wormholes"). This changes 
// the network's topological shape, distributing strain and flattening the curvature.
// ============================================================================

use std::sync::{Mutex, OnceLock};

pub struct CurvatureMonitor {
    pub curvature_k: f64,
    pub threshold: f64,
    pub active_wormhole_port: Option<u16>,
}

impl CurvatureMonitor {
    pub fn new(threshold: f64) -> Self {
        Self {
            curvature_k: 0.0,
            threshold,
            active_wormhole_port: None,
        }
    }

    /// Calculate curvature based on tensegrity load.
    /// In differential geometry, curvature K is computed here via an aggregate
    /// of network strain and queue depth. We map tensegrity load directly to K.
    pub fn calculate_curvature(&mut self, tensegrity_load: f64, predicted_k: f64) -> bool {
        // Simple mapping: K grows exponentially with tensegrity load above 0.8
        if tensegrity_load > 0.8 {
            self.curvature_k += (tensegrity_load - 0.8) * 1.5;
        } else {
            self.curvature_k *= 0.9; // Decay curvature over time if load is safe
        }

        if self.curvature_k > self.threshold || predicted_k > self.threshold {
            if self.active_wormhole_port.is_none() {
                self.spawn_wormhole(predicted_k > self.threshold && self.curvature_k <= self.threshold);
                return true; // Alert triggered
            }
        } else {
            // Curvature is flat again, close wormhole
            if self.active_wormhole_port.is_some() && self.curvature_k < (self.threshold * 0.5) {
                self.close_wormhole();
            }
        }
        
        false
    }

    fn spawn_wormhole(&mut self, preemptive: bool) {
        // In a full implementation, we'd bind a real UDP socket and forward traffic.
        // For Phase 13/14, we bind a random high port to simulate the topological shift.
        if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
            if let Ok(addr) = socket.local_addr() {
                let port = addr.port();
                self.active_wormhole_port = Some(port);
                if preemptive {
                    println!("\x1b[35;1m[ESN FORECAST] PREDICTED CURVATURE EXCEEDS LIMIT. PREEMPTIVE WORMHOLE SPAWNED ON PORT {}.\x1b[0m", port);
                } else {
                    println!("\x1b[31;1m[GAUSS-BONNET] CRITICAL CURVATURE (K={:.2}) DETECTED. SPAWNING WORMHOLE ON PORT {} TO ALTER TOPOLOGY.\x1b[0m", self.curvature_k, port);
                }
            }
        }
    }

    fn close_wormhole(&mut self) {
        if let Some(port) = self.active_wormhole_port {
            println!("\x1b[32m[GAUSS-BONNET] Curvature flattened. Closing topological wormhole on port {}.\x1b[0m", port);
            self.active_wormhole_port = None;
        }
    }
}

pub fn global_curvature() -> &'static Mutex<CurvatureMonitor> {
    static CURVATURE: OnceLock<Mutex<CurvatureMonitor>> = OnceLock::new();
    CURVATURE.get_or_init(|| Mutex::new(CurvatureMonitor::new(10.0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_bonnet_curvature() {
        let mut monitor = CurvatureMonitor::new(5.0);
        
        // Low load, curvature stays 0
        monitor.calculate_curvature(0.5, 0.0);
        assert!(monitor.curvature_k < 0.1);
        assert!(monitor.active_wormhole_port.is_none());

        // High load, curvature spikes
        for _ in 0..10 {
            monitor.calculate_curvature(1.5, 0.0);
        }

        // Wormhole should be active
        assert!(monitor.curvature_k > 5.0);
        assert!(monitor.active_wormhole_port.is_some());

        // Load drops, curvature decays
        for _ in 0..20 {
            monitor.calculate_curvature(0.1, 0.0);
        }

        // Wormhole should close
        assert!(monitor.curvature_k < 2.5);
        assert!(monitor.active_wormhole_port.is_none());
    }
}
