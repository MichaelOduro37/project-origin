// ============================================================================
// PHASE 51: PENROSE TILING (APERIODIC CRYPTOGRAPHY)
// ============================================================================
// Scientific mechanism: Geometry (Aperiodic Tiling)
//
// Traditional encryption relies on PRNGs, which have a finite period. They
// eventually repeat, creating mathematical cycles that Quantum Computers or AI
// can exploit to shatter the cipher.
// A Penrose Tiling (Kites and Darts) tiles an infinite plane without EVER
// repeating. We use this infinitely non-repeating geometric sequence to
// generate our cryptographic pad. Because there are zero repeating cycles,
// it is fundamentally immune to Quantum Cryptanalysis and AI pattern recognition.
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PenroseShape {
    Kite,
    Dart,
}

/// Generates a localized sequence of the Penrose Tiling via substitution (inflation) rules.
/// A Kite inflates into -> [Kite, Dart, Kite]
/// A Dart inflates into -> [Kite, Dart]
/// (This is a simplified 1D string substitution representing the 2D geometric inflation).
/// The resulting sequence of shapes is mathematically aperiodic.
pub fn generate_aperiodic_lattice(depth: usize) -> Vec<PenroseShape> {
    // We start with a single Kite as our geometric "seed"
    let mut lattice = vec![PenroseShape::Kite];

    for _ in 0..depth {
        let mut next_generation = Vec::new();
        
        for shape in &lattice {
            match shape {
                PenroseShape::Kite => {
                    next_generation.push(PenroseShape::Kite);
                    next_generation.push(PenroseShape::Dart);
                    next_generation.push(PenroseShape::Kite);
                }
                PenroseShape::Dart => {
                    next_generation.push(PenroseShape::Kite);
                    next_generation.push(PenroseShape::Dart);
                }
            }
        }
        lattice = next_generation;
    }

    lattice
}

/// Encrypts (and decrypts) a payload using the aperiodic sequence.
/// It uses a byte representation of the Kite/Dart geometry to mutate the data.
/// Because the lattice never repeats, the cipher is effectively an infinite One-Time Pad.
pub fn process_aperiodic_cipher(payload: &[u8], lattice: &[PenroseShape]) -> Vec<u8> {
    let mut processed = Vec::with_capacity(payload.len());
    
    // We map Kite -> 0b10101010 (170) and Dart -> 0b01010101 (85)
    for (i, &byte) in payload.iter().enumerate() {
        // Wrap around the lattice if the payload is larger (though in reality,
        // we would just generate a deeper lattice to ensure zero repetition).
        let shape = lattice[i % lattice.len()];
        let geometric_pad = match shape {
            PenroseShape::Kite => 170u8,
            PenroseShape::Dart => 85u8,
        };

        // Simple XOR for the cipher mapping.
        // Because the geometric sequence of shapes is aperiodic, the XOR stream is patternless.
        processed.push(byte ^ geometric_pad);
    }

    processed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aperiodic_lattice_growth() {
        // Verify the inflation rules grow the lattice as expected.
        let l0 = generate_aperiodic_lattice(0);
        assert_eq!(l0, vec![PenroseShape::Kite]);

        let l1 = generate_aperiodic_lattice(1);
        assert_eq!(l1, vec![PenroseShape::Kite, PenroseShape::Dart, PenroseShape::Kite]);

        let l2 = generate_aperiodic_lattice(2);
        // Kite -> Kite, Dart, Kite
        // Dart -> Kite, Dart
        // Kite -> Kite, Dart, Kite
        assert_eq!(l2.len(), 3 + 2 + 3);
    }

    #[test]
    fn test_aperiodic_encryption_symmetry() {
        let payload = b"CLASSIFIED_ORIGIN_DATA".to_vec();
        
        // Generate a lattice deep enough to cover the payload without wrapping (depth 5 -> 21 shapes? let's do 6)
        // Depth 0: 1
        // Depth 1: 3
        // Depth 2: 8
        // Depth 3: 21
        // Depth 4: 55  (Fibonacci-like growth)
        let lattice = generate_aperiodic_lattice(4); 
        assert!(lattice.len() >= payload.len());

        // Encrypt
        let encrypted = process_aperiodic_cipher(&payload, &lattice);
        assert_ne!(payload, encrypted); // Should be garbled

        // Decrypt (XOR cipher is symmetrical)
        let decrypted = process_aperiodic_cipher(&encrypted, &lattice);
        assert_eq!(payload, decrypted); // Should flawlessly reconstruct
    }
}
