// ============================================================================
// PHASE 49: NO-CLONING THEOREM (QUANTUM EAVESDROPPING DETECTION)
// ============================================================================
// Scientific mechanism: Quantum Mechanics (No-Cloning Theorem)
//
// Traditional encryption (TLS) hides data, but hackers/ISPs can secretly
// copy the encrypted packets ("Harvest Now, Decrypt Later").
// We map keys into simulated Quantum Qubits (polarization states).
// The No-Cloning theorem states it is impossible to perfectly copy a state.
// If an eavesdropper "measures" or copies the packet, the wave function
// collapses irreversibly. The receiver checks the basis and detects a massive
// error rate, mathematically proving the channel is compromised.
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantumBasis {
    Rectilinear, // |0>, |1>  (represented as +, -)
    Diagonal,    // |+>, |->  (represented as / , \)
}

#[derive(Debug, Clone, Copy)]
pub struct Qubit {
    pub bit_value: u8, // 0 or 1
    pub basis: QuantumBasis,
}

impl Qubit {
    pub fn new(bit_value: u8, basis: QuantumBasis) -> Self {
        Qubit { bit_value, basis }
    }
}

/// Simulates an Eavesdropper (Eve) or a Receiver (Bob) "measuring" the qubit.
/// If they guess the wrong measurement basis, the quantum wave function collapses,
/// and the resulting bit value is randomized, destroying the original data.
pub fn measure_state(qubit: &Qubit, measurement_basis: QuantumBasis) -> u8 {
    if qubit.basis == measurement_basis {
        // Correct basis: wave function is preserved, exact bit is read.
        qubit.bit_value
    } else {
        // Wrong basis: wave function COLLAPSES. The data is randomized (50/50 chance).
        rand::random::<u8>() % 2
    }
}

/// Simulates a Deep Packet Inspection (DPI) attack.
/// The hacker tries to copy the packets in transit.
pub fn eavesdrop_attack(qubits: &mut Vec<Qubit>) {
    for qubit in qubits.iter_mut() {
        // The hacker doesn't know the polarization basis, so they guess randomly.
        let random_basis = if rand::random::<bool>() {
            QuantumBasis::Rectilinear
        } else {
            QuantumBasis::Diagonal
        };

        // The act of inspecting the packet forces a measurement, collapsing the wave function.
        let collapsed_bit = measure_state(qubit, random_basis);
        
        // The qubit is now permanently altered to the hacker's measurement state.
        qubit.bit_value = collapsed_bit;
        qubit.basis = random_basis;
    }
}

#[derive(Debug)]
pub enum WiretapError {
    EavesdropperDetected(f64), // Contains the error rate
}

/// The receiver verifies the quantum coherence of the connection.
/// Returns Ok if the channel is secure, or Err if a wiretap collapsed the wave functions.
pub fn verify_coherence(sent_bases: &[QuantumBasis], received_qubits: &[Qubit], original_bits: &[u8]) -> Result<(), WiretapError> {
    let mut matching_bases = 0;
    let mut errors = 0;

    for i in 0..received_qubits.len() {
        // Only evaluate qubits where the sender and receiver used the exact same basis.
        // Theoretically, if no one interfered, 100% of these should match the original bit.
        if sent_bases[i] == received_qubits[i].basis {
            matching_bases += 1;
            
            if received_qubits[i].bit_value != original_bits[i] {
                errors += 1;
            }
        }
    }

    if matching_bases == 0 {
        return Ok(()); // Edge case, avoid divide by zero
    }

    let error_rate = (errors as f64) / (matching_bases as f64);

    // If an eavesdropper collapsed the wave functions, the error rate on matching
    // bases will spike to roughly 25%. We use 10% as a safe detection threshold.
    if error_rate > 0.10 {
        Err(WiretapError::EavesdropperDetected(error_rate))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_quantum_channel() {
        let size = 1000;
        let mut qubits = Vec::new();
        let mut sent_bases = Vec::new();
        let mut original_bits = Vec::new();

        for _ in 0..size {
            let bit = rand::random::<u8>() % 2;
            let basis = if rand::random::<bool>() { QuantumBasis::Rectilinear } else { QuantumBasis::Diagonal };
            
            qubits.push(Qubit::new(bit, basis));
            sent_bases.push(basis);
            original_bits.push(bit);
        }

        // Bob receives them without Eve eavesdropping.
        // Bob measures them with his own random bases.
        let mut bobs_qubits = Vec::new();
        for qubit in qubits {
            let bobs_basis = if rand::random::<bool>() { QuantumBasis::Rectilinear } else { QuantumBasis::Diagonal };
            let bobs_bit = measure_state(&qubit, bobs_basis);
            bobs_qubits.push(Qubit::new(bobs_bit, bobs_basis));
        }

        // Verify: Error rate should be 0.0 because no one collapsed the states in transit.
        let result = verify_coherence(&sent_bases, &bobs_qubits, &original_bits);
        assert!(result.is_ok(), "Channel should be secure!");
    }

    #[test]
    fn test_eavesdropper_detection() {
        let size = 1000;
        let mut qubits = Vec::new();
        let mut sent_bases = Vec::new();
        let mut original_bits = Vec::new();

        for _ in 0..size {
            let bit = rand::random::<u8>() % 2;
            let basis = if rand::random::<bool>() { QuantumBasis::Rectilinear } else { QuantumBasis::Diagonal };
            
            qubits.push(Qubit::new(bit, basis));
            sent_bases.push(basis);
            original_bits.push(bit);
        }

        // EVE INTERCEPTS AND INSPECTS THE PACKETS (Deep Packet Inspection)
        eavesdrop_attack(&mut qubits);

        // Bob receives the altered qubits.
        let mut bobs_qubits = Vec::new();
        for qubit in qubits {
            let bobs_basis = if rand::random::<bool>() { QuantumBasis::Rectilinear } else { QuantumBasis::Diagonal };
            let bobs_bit = measure_state(&qubit, bobs_basis);
            bobs_qubits.push(Qubit::new(bobs_bit, bobs_basis));
        }

        // Verify: The wave function collapse caused by Eve should spike the error rate.
        let result = verify_coherence(&sent_bases, &bobs_qubits, &original_bits);
        assert!(result.is_err(), "Eavesdropper must be detected!");
        
        if let Err(WiretapError::EavesdropperDetected(error_rate)) = result {
            assert!(error_rate > 0.15, "Error rate should be roughly 25%");
        }
    }
}
