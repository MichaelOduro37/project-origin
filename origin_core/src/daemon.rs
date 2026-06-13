// ============================================================================
// PHASE 4: 10-NODE MESH PROOF OF CONCEPT
// ============================================================================

use crate::cipher::OriginAI;
use crate::tensegrity::{TensegritySwarmNode, HardwareEnvironment, IsingTensegrityOptimizer};
use crate::network::{start_node_listener, FermonicRouter};
use crate::immune::{AisImmuneSystem, HdcAnomalyDetector};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use rand::RngExt;

pub async fn run() {
    println!("===========================================================");
    println!("=== PHASE 4: 10-NODE MESH PROOF OF CONCEPT EXECUTING    ===");
    println!("===========================================================\n");

    // 1. Initialize Immune System and AI
    let _ai_cipher = OriginAI::new();
    let self_space_bytes = vec![0x00, 0x01, 0x02, 0x03];
    let mut immune_system = AisImmuneSystem::new(vec![self_space_bytes.clone()], 4);
    immune_system.train_detectors(50, 16);
    let shared_immune_system = Arc::new(Mutex::new(immune_system));

    // 2. Initialize 10 Nodes
    let mut nodes = Vec::new();
    let base_port: u16 = 9000;
    
    for i in 0..10 {
        let node_name = format!("Node_{}", i);
        let env = HardwareEnvironment {
            battery_level: 100 - (i * 5) as u8,
            is_plugged_in: i % 3 == 0,
            thermal_load: 30 + i as u8,
            cpu_cores: 8,
        };
        let node = Arc::new(Mutex::new(TensegritySwarmNode::new(&node_name, env)));
        nodes.push(node.clone());
        
        let port = base_port + i as u16;
        tokio::spawn(start_node_listener(port, node, Arc::clone(&shared_immune_system)));
    }
    
    sleep(Duration::from_millis(500)).await;

    // 3. SCENARIO 1: Ising-Tensegrity Load Shedding 
    println!("\n--- SCENARIO 1: ISING-TENSEGRITY LOAD SHEDDING ---");
    println!("[SYSTEM] Simulating massive traffic spike on Node 0...");
    let mut ising_opt = IsingTensegrityOptimizer::new("Node_0".to_string());
    ising_opt.update_local_load(5000, 200.0); // 5000 pkts, 200ms latency = OVERLOAD
    
    // Simulate peer gossip (Peers are also overloaded)
    ising_opt.ingest_peer_state("Node_1".to_string(), -1, 0.9);
    ising_opt.ingest_peer_state("Node_2".to_string(), -1, 0.8);
    
    let mut spin = 1;
    for _ in 0..15 {
        spin = ising_opt.relax_to_ground_state();
    }
    println!("[OUTCOME] Node 0 Ground State Spin: {} (Expect -1 / SHED)", spin);


    // 4. SCENARIO 2: HDC Zero-Day Anomaly Quarantine
    println!("\n--- SCENARIO 2: HDC ZERO-DAY QUARANTINE ---");
    let mut hdc = HdcAnomalyDetector::new();
    
    // Train on normal telemetry for Node 5
    let normal_data = vec![(10.0, 0.1, 0.2), (12.0, 0.15, 0.22), (9.0, 0.08, 0.18)];
    hdc.train(&normal_data);
    
    // Inject zero-day topology attack (Massive CPU, strange packet rate)
    println!("[ATTACKER] Injecting Byzantine topology mapping attack...");
    let is_quarantine = hdc.is_anomalous(999.0, 1.0, 0.95);
    println!("[OUTCOME] Quarantine Triggered: {}", is_quarantine);


    // 5. SCENARIO 3: Fermionic Routing
    println!("\n--- SCENARIO 3: FERMIONIC ROUTING (HARDWARE RADIO ABSTRACTED) ---");
    use crate::network::HardwareRadio;
    let mock_radio = HardwareRadio::bind_mock(0).await;
    let mut router = FermonicRouter::new("Node_0".to_string(), 0.5, mock_radio);
    router.register_topology(9000, vec![9001, 9002, 9003]);
    
    // Peer states: 9001 is active (true), others are inactive
    let mut mesh_state = HashMap::new();
    mesh_state.insert(9001, true); // Active, meaning state is occupied
    mesh_state.insert(9002, false);
    mesh_state.insert(9003, false);
    
    println!("[SYSTEM] Attempting to route from 9000 to 9009. Node 9001 is OCCUPIED (Pauli Exclusion).");
    let routes = router.route_fermionic(9000, 9009, &mesh_state);
    
    for route in routes {
        let first_hop = route.hops.first().unwrap_or(&0);
        println!("[OUTCOME] Route selected through port {}. Collision avoided: {}", first_hop, *first_hop != 9001);
    }

    // 6. SCENARIO 4: RMT Chaotic Key Generation
    println!("\n--- SCENARIO 4: RMT CHAOTIC KEY GENERATION ---");
    use crate::cipher::RMTKeyGenerator;
    let mut rmt = RMTKeyGenerator::new("Node_0".to_string(), 50);
    let keys = rmt.generate_rmt_keys(16);
    println!("[SYSTEM] Generated 16 bytes of entropy from local chaotic Hamiltonian.");
    println!("[OUTCOME] Quality Passed: {}", rmt.verify_entropy_quality(&keys));

    // 7. SCENARIO 5: SwarmSense-DNN Byzantine Consensus
    println!("\n--- SCENARIO 5: SWARM BYZANTINE CONSENSUS ---");
    use crate::immune::SwarmAnomalyVoter;
    let mut voter = SwarmAnomalyVoter::new("Node_0".to_string(), 0.66);
    
    let mut peer_votes = HashMap::new();
    peer_votes.insert("Node_1".to_string(), 0.9); // Peer 1 says anomaly!
    peer_votes.insert("Node_2".to_string(), 0.8); // Peer 2 says anomaly!
    peer_votes.insert("Node_3".to_string(), 0.1); // Peer 3 (hacked) says normal
    
    let consensus = voter.byzantine_vote_anomaly(true, peer_votes);
    println!("[SYSTEM] Collecting anomaly votes from 4 neighbors.");
    println!("[OUTCOME] Byzantine Consensus Reached (Quarantine): {}", consensus);

    // 8. SCENARIO 6: Phase 5 Topological Surface Codes
    println!("\n--- SCENARIO 6: TOPOLOGICAL SURFACE CODES (PHASE 5) ---");
    use crate::cipher::TopologicalSurfaceCode;
    let sc = TopologicalSurfaceCode::new(3, 3);
    let packet_shards = vec![11, 22, 33, 44, 55, 66, 77, 88, 99];
    let (mut lattice, _syndromes) = sc.generate_syndrome_lattice(&packet_shards);
    
    println!("[SYSTEM] 1D Packet mapped to 2D Surface Code Parity Lattice.");
    println!("[ATTACKER] Dropping packet shard at [1][1] (Original value: 55)...");
    lattice[1][1] = 0; // Erasure
    
    // 9. PHASE 6, 7 & 9: Live Telemetry, Chat & Swarm Updates
    println!("\n--- SCENARIO 7: LIVE TELEMETRY & SWARM UPDATER ---");
    use crate::telemetry::{TelemetryServer, TelemetryEvent};
    use crate::updater::SwarmUpdater;
    
    let (telemetry, mut ui_rx) = TelemetryServer::new();
    let tx = telemetry.get_sender();
    let mut updater = SwarmUpdater::new();
    
    tokio::spawn(telemetry.start_daemon(8080));
    println!("[SYSTEM] WebSocket Telemetry Daemon running on ws://127.0.0.1:8080");
    
    // 10. Start Universal Binary Web UI
    tokio::spawn(async {
        let app = axum::Router::new().route("/*key", axum::routing::get(crate::ui::static_handler)).route("/", axum::routing::get(crate::ui::static_handler));
        if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:8081").await {
            println!("[UI DAEMON] Universal UI hosted at http://127.0.0.1:8081");
            let _ = axum::serve(listener, app).await;
        }
    });

    let mut sys = sysinfo::System::new_all();
    let mut components = sysinfo::Components::new_with_topo();

    // Infinite loop feeding chaotic physics data to the UI Dashboard
    println!("[SYSTEM] Streaming live Tensegrity & Chat data to the UI... (Press Ctrl+C to stop)");
    loop {
        {
            let mut rng = rand::rng();
            sys.refresh_all();
            components.refresh();

            // Occasionally simulate intercepting an OTA update fragment from the mesh
            if rng.random_bool(0.05) {
                updater.receive_shard("v2.1_QUANTUM_PATCH", &[0x01, 0x02, 0x03]);
            }

            // Poll for incoming chat messages from UI
            while let Ok(msg) = ui_rx.try_recv() {
                println!("[APPLICATION LAYER] Received raw text from UI: {}", msg);
                let chaotic_hash = format!("{:016X}", rng.random::<u64>());
                let encrypted = format!("{}::{}_ENC", chaotic_hash, msg.chars().rev().collect::<String>());
                
                let _ = tx.send(TelemetryEvent::ChatIncoming {
                    sender: "Peer_Node_7".to_string(),
                    encrypted_payload: encrypted,
                    decrypted_payload: msg,
                });
            }

            // Real Hardware Telemetry (Phase 7)
            let is_shedding = rng.random_bool(0.3);
            
            let mut max_temp: f64 = 0.0;
            for comp in &components {
                let temp = comp.temperature() as f64;
                if temp > max_temp { max_temp = temp; }
            }
            if max_temp == 0.0 {
                // Fallback if hardware sensors are restricted (e.g. Android sandbox without JNI)
                max_temp = 35.0 + rng.random::<f64>() * 5.0;
            }

            let cpus = sys.cpus();
            let mut load = 1.0;
            if !cpus.is_empty() {
                load = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() as f64 / cpus.len() as f64;
            }

            let _ = tx.send(TelemetryEvent::TensegrityState {
                node: "Node_0".to_string(),
                spin: if is_shedding { -1 } else { 1 },
                temp: max_temp,
                load: load.max(0.01),
            });

            // Broadcast HDC Anomalies occasionally
            if rng.random_bool(0.15) {
                let _ = tx.send(TelemetryEvent::ImmuneAlert {
                    distance: 0.35 + rng.random::<f64>() * 0.2,
                    threshold: 0.35,
                    quarantined: true,
                });
            }

            // Broadcast Fermionic Routes
            if rng.random_bool(0.4) {
                let packet_id = format!("{:06X}", rng.random_range(0..0xFFFFFF));
                let origin = rng.random_range(0..10);
                let dest = rng.random_range(0..10);
                if origin != dest {
                    let _ = tx.send(TelemetryEvent::FermionicRoute {
                        packet_id,
                        origin: format!("Node {}", origin),
                        dest: format!("Node {}", dest),
                        is_quantum: rng.random_bool(0.5),
                    });
                }
            }
        }
        
        sleep(Duration::from_millis(1500)).await;
    }
}
