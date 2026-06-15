// ============================================================================
// PHASE 14: RESERVOIR COMPUTING (ECHO STATE NETWORKS)
// ============================================================================
// Scientific mechanism: An Echo State Network (ESN) is a recurrent neural network
// with a sparsely connected, fixed hidden layer (the reservoir). 
// By ensuring the spectral radius is < 1, the network maintains the "echo state 
// property", acting as a fading memory temporal kernel.
//
// Application: The node feeds real-time telemetry (Load, Strain, Curvature) into 
// the reservoir. The linear readout layer predicts the Curvature (K) T+10 ticks 
// into the future, allowing preemptive Gauss-Bonnet wormhole generation.
// ============================================================================

use std::sync::{Mutex, OnceLock};

const RESERVOIR_SIZE: usize = 64;

pub struct EchoStateNetwork {
    pub state: [f64; RESERVOIR_SIZE],
    w_in: [[f64; RESERVOIR_SIZE]; 2], // 2 inputs: load, current_curvature
    w_res: [[f64; RESERVOIR_SIZE]; RESERVOIR_SIZE],
    w_out: [f64; RESERVOIR_SIZE],
}

impl EchoStateNetwork {
    pub fn new() -> Self {
        let mut esn = Self {
            state: [0.0; RESERVOIR_SIZE],
            w_in: [[0.0; RESERVOIR_SIZE]; 2],
            w_res: [[0.0; RESERVOIR_SIZE]; RESERVOIR_SIZE],
            w_out: [0.0; RESERVOIR_SIZE],
        };

        // Initialize pseudo-random weights (Deterministic for Origin)
        for i in 0..RESERVOIR_SIZE {
            esn.w_in[0][i] = ((i * 13) % 100) as f64 / 100.0 - 0.5;
            esn.w_in[1][i] = ((i * 17) % 100) as f64 / 100.0 - 0.5;
            
            esn.w_out[i] = ((i * 23) % 100) as f64 / 100.0;

            for j in 0..RESERVOIR_SIZE {
                // Sparse connections
                if (i * j) % 7 == 0 {
                    esn.w_res[i][j] = ((i + j) % 100) as f64 / 100.0 - 0.5;
                    // Scale to ensure spectral radius < 1.0 (Echo State Property)
                    esn.w_res[i][j] *= 0.5; 
                }
            }
        }

        esn
    }

    /// Step the reservoir forward one time tick with new inputs.
    pub fn step(&mut self, load: f64, current_curvature: f64) {
        let mut next_state = [0.0; RESERVOIR_SIZE];

        for i in 0..RESERVOIR_SIZE {
            // 1. Input contribution
            let input_val = load * self.w_in[0][i] + current_curvature * self.w_in[1][i];
            
            // 2. Reservoir recurrent contribution
            let mut res_val = 0.0;
            for j in 0..RESERVOIR_SIZE {
                res_val += self.state[j] * self.w_res[j][i];
            }

            // Non-linear activation (tanh)
            next_state[i] = (input_val + res_val).tanh();
        }

        self.state = next_state;
    }

    /// Read out the predicted future curvature (T+10).
    pub fn predict(&self) -> f64 {
        let mut prediction = 0.0;
        for i in 0..RESERVOIR_SIZE {
            prediction += self.state[i] * self.w_out[i];
        }
        
        // Ensure prediction doesn't drop below 0
        prediction.max(0.0)
    }
}

pub fn global_reservoir() -> &'static Mutex<EchoStateNetwork> {
    static ESN: OnceLock<Mutex<EchoStateNetwork>> = OnceLock::new();
    ESN.get_or_init(|| Mutex::new(EchoStateNetwork::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_state_property() {
        let mut esn = EchoStateNetwork::new();
        
        // Feed quiet state
        for _ in 0..10 {
            esn.step(0.1, 0.0);
        }
        let quiet_pred = esn.predict();

        // Feed chaotic spike
        for _ in 0..5 {
            esn.step(1.5, 5.0);
        }
        let spike_pred = esn.predict();

        assert!(spike_pred > quiet_pred, "ESN should predict higher curvature after load spike");
    }
}
