#[cfg(test)]
mod phase4_tests {
    use crate::immune::{Hypervector, HdcAnomalyDetector};
    use crate::tensegrity::IsingTensegrityOptimizer;

    #[test]
    fn test_hdc_binding_and_distance() {
        let v1 = Hypervector::random(42);
        let v2 = Hypervector::random(99);
        
        let dist = v1.hamming_distance(&v2);
        // Distance between two random 10,000 D bipolar vectors should be ~0.5 (orthogonality)
        assert!(dist > 0.45 && dist < 0.55);

        let v3 = v1.bind(&v2);
        // Binding shouldn't collapse the vector
        let dist_bind = v3.hamming_distance(&v1);
        assert!(dist_bind > 0.45 && dist_bind < 0.55);
    }

    #[test]
    fn test_hdc_anomaly_detection() {
        let mut detector = HdcAnomalyDetector::new();
        // Normal traffic profile
        let normal_samples = vec![
            (10.0, 0.1, 0.2),
            (11.0, 0.1, 0.2),
            (10.5, 0.15, 0.2),
        ];
        detector.train(&normal_samples);

        // Test normal
        assert!(!detector.is_anomalous(10.2, 0.12, 0.2));

        // Test anomaly (high CPU, low packet rate)
        assert!(detector.is_anomalous(1.0, 0.99, 0.8));
    }

    #[test]
    fn test_ising_tensegrity_relaxation() {
        let mut optimizer = IsingTensegrityOptimizer::new("TestNode".to_string());
        
        // Setup high local load (h_i will push spin to -1)
        optimizer.update_local_load(1000, 100.0);
        
        // Peer is shedding (-1), tension is high (0.8)
        optimizer.ingest_peer_state("Peer1".to_string(), -1, 0.8);

        // Force deterministic low temp behavior
        optimizer.temperature = 0.001; 
        
        let mut new_spin = 1;
        // Run a few relaxation steps
        for _ in 0..10 {
            new_spin = optimizer.relax_to_ground_state();
        }
        
        // With extreme load, it should shed load (-1)
        assert_eq!(new_spin, -1);
    }

    #[test]
    fn test_surface_code_healing() {
        use crate::cipher::TopologicalSurfaceCode;

        let sc = TopologicalSurfaceCode::new(3, 3);
        
        // Create 9 dummy data shards
        let data = vec![101, 202, 303, 404, 505, 606, 707, 808, 909];
        
        let (mut lattice, syndromes) = sc.generate_syndrome_lattice(&data);
        
        // Verify lattice structure
        assert_eq!(lattice[1][1], 505);
        
        // Simulate a network erasure (dropped packet chunk)
        lattice[1][1] = 0;
        
        // Attempt localized topological heal using MWPM logic
        let healed_val = sc.mwpm_local_heal(1, 1, &mut lattice, &syndromes).unwrap();
        
        // Mathematically verify reconstruction
        assert_eq!(healed_val, 505);
        assert_eq!(lattice[1][1], 505);
    }
}
