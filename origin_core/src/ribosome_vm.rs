// ============================================================================
// PHASE 53: RIBOSOMAL VIRTUAL MACHINE (BIOLOGICAL ASSEMBLY)
// ============================================================================
// Scientific mechanism: Molecular Biology (Ribosomal Translation)
//
// Traditional smart contracts require massive Virtual Machines (EVM, WASM)
// running on a heavy OS. 
// Origin implements the biological computing model. Logic payloads are sent
// as `mRNA Vectors` (codon triplets). The Origin node acts as a `Ribosome`.
// It reads the codons and dynamically synthesizes execution primitives
// (Amino Acids) on the fly, assembling them into a runnable "Protein".
// This provides Turing-complete execution at the hyper-efficiency of a cell.
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Codon {
    AUG, // Start Translation
    GCA, // Arithmetic: Add
    UGC, // Cryptography: Hash
    CGA, // Memory: Write State
    UAA, // Stop Translation
}

#[derive(Debug, Clone, PartialEq)]
pub enum AminoAcid {
    InitializeEnvironment,
    OpAdd,
    OpHash,
    WriteMemory,
    TerminateExecution,
}

pub struct Ribosome {
    pub execution_environment_active: bool,
}

impl Ribosome {
    pub fn new() -> Self {
        Self {
            execution_environment_active: false,
        }
    }

    /// The Ribosome reads the mRNA vector (sequence of codons) and synthesizes
    /// the corresponding executable logic primitives (Amino Acids).
    pub fn translate_and_fold(&mut self, mrna: &[Codon]) -> Result<Vec<AminoAcid>, &'static str> {
        let mut protein = Vec::new();

        for codon in mrna {
            match codon {
                Codon::AUG => {
                    self.execution_environment_active = true;
                    protein.push(AminoAcid::InitializeEnvironment);
                }
                Codon::GCA => {
                    if !self.execution_environment_active { return Err("Ribosome inactive: missing AUG start codon"); }
                    protein.push(AminoAcid::OpAdd);
                }
                Codon::UGC => {
                    if !self.execution_environment_active { return Err("Ribosome inactive: missing AUG start codon"); }
                    protein.push(AminoAcid::OpHash);
                }
                Codon::CGA => {
                    if !self.execution_environment_active { return Err("Ribosome inactive: missing AUG start codon"); }
                    protein.push(AminoAcid::WriteMemory);
                }
                Codon::UAA => {
                    if !self.execution_environment_active { return Err("Ribosome inactive: missing AUG start codon"); }
                    protein.push(AminoAcid::TerminateExecution);
                    self.execution_environment_active = false;
                    break; // Translation complete
                }
            }
        }

        Ok(protein)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ribosomal_translation_success() {
        let mut ribosome = Ribosome::new();
        
        // A valid active payload: Start -> Add -> Hash -> Write -> Stop
        let mrna_vector = vec![Codon::AUG, Codon::GCA, Codon::UGC, Codon::CGA, Codon::UAA];
        
        let protein = ribosome.translate_and_fold(&mrna_vector).unwrap();
        
        assert_eq!(protein.len(), 5);
        assert_eq!(protein[0], AminoAcid::InitializeEnvironment);
        assert_eq!(protein[4], AminoAcid::TerminateExecution);
        assert_eq!(ribosome.execution_environment_active, false); // properly terminated
    }

    #[test]
    fn test_ribosomal_translation_failure() {
        let mut ribosome = Ribosome::new();
        
        // Invalid payload: No AUG start codon
        let invalid_mrna = vec![Codon::GCA, Codon::UAA];
        
        let result = ribosome.translate_and_fold(&invalid_mrna);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Ribosome inactive: missing AUG start codon");
    }
}
