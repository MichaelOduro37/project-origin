// ============================================================================
// PHASE 4: 10-NODE MESH PROOF OF CONCEPT
// ============================================================================

use origin_core::cipher::OriginAI;
use origin_core::tensegrity::{TensegritySwarmNode, HardwareEnvironment, IsingTensegrityOptimizer};
use origin_core::network::{start_node_listener, FermonicRouter};
use origin_core::immune::{AisImmuneSystem, HdcAnomalyDetector};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use rand::RngExt;
use rust_embed::RustEmbed;
use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::{StatusCode, header, Uri},
};

#[derive(RustEmbed)]
#[folder = "../origin_ui/dist"]
struct WebAssets;

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/');
    if path.is_empty() {
        path = "index.html";
    }
    
    match WebAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                [(header::CONTENT_TYPE, mime.as_ref())],
                content.data,
            ).into_response()
        }
        None => {
            if let Some(index) = WebAssets::get("index.html") {
                (
                    [(header::CONTENT_TYPE, "text/html")],
                    index.data,
                ).into_response()
            } else {
                (StatusCode::NOT_FOUND, "404 Not Found").into_response()
            }
        }
    }
}

#[tokio::main]
async fn main() {
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
    println!("\n--- SCENARIO 3: FERMIONIC ROUTING (PAULI EXCLUSION) ---");
    use origin_core::network::HardwareRadio;
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
    use origin_core::cipher::RMTKeyGenerator;
    let mut rmt = RMTKeyGenerator::new("Node_0".to_string(), 50);
    let keys = rmt.generate_rmt_keys(16);
    println!("[SYSTEM] Generated 16 bytes of entropy from local chaotic Hamiltonian.");
    println!("[OUTCOME] Quality Passed: {}", rmt.verify_entropy_quality(&keys));

    // 7. SCENARIO 5: SwarmSense-DNN Byzantine Consensus
    println!("\n--- SCENARIO 5: SWARM BYZANTINE CONSENSUS ---");
    use origin_core::immune::SwarmAnomalyVoter;
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
    use origin_core::cipher::TopologicalSurfaceCode;
    let sc = TopologicalSurfaceCode::new(3, 3);
    let packet_shards = vec![11, 22, 33, 44, 55, 66, 77, 88, 99];
    let (mut lattice, _syndromes) = sc.generate_syndrome_lattice(&packet_shards);
    
    println!("[SYSTEM] 1D Packet mapped to 2D Surface Code Parity Lattice.");
    println!("[ATTACKER] Dropping packet shard at [1][1] (Original value: 55)...");
    lattice[1][1] = 0; // Erasure
    
    // 9. PHASE 6 & 7: Live Telemetry & Application Layer Daemon
    println!("\n--- SCENARIO 7: LIVE TELEMETRY & CHAT SERVER ---");
    use origin_core::telemetry::{TelemetryServer, TelemetryEvent};
    let (telemetry, mut ui_rx) = TelemetryServer::new();
    let tx = telemetry.get_sender();
    
    tokio::spawn(telemetry.start_daemon(8080));
    println!("[SYSTEM] WebSocket Telemetry Daemon running on ws://127.0.0.1:8080");
    
    // 10. Start Universal Binary Web UI
    tokio::spawn(async {
        let app = Router::new().route("/*key", get(static_handler)).route("/", get(static_handler));
        if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:8081").await {
            println!("[UI DAEMON] Universal UI hosted at http://127.0.0.1:8081");
            
            // Open the browser for the user automatically!
            if let Err(e) = webbrowser::open("http://127.0.0.1:8081") {
                println!("[UI DAEMON] Failed to open browser: {}", e);
            }
            
            let _ = axum::serve(listener, app).await;
        }
    });

    // Infinite loop feeding chaotic physics data to the UI Dashboard
    println!("[SYSTEM] Streaming live Tensegrity & Chat data to the UI... (Press Ctrl+C to stop)");
    let mut rng = rand::rng();
    
    
    loop {
        // Poll for incoming chat messages from UI
        while let Ok(msg) = ui_rx.try_recv() {
            println!("[APPLICATION LAYER] Received raw text from UI: {}", msg);
            // Simulate Chaotic Encryption & Swarm Hop
            let chaotic_hash = format!("{:016X}", rng.random::<u64>());
            let encrypted = format!("{}::{}_ENC", chaotic_hash, msg.chars().rev().collect::<String>());
            
            // Broadcast back out as if it traversed the mesh and healed
            let _ = tx.send(TelemetryEvent::ChatIncoming {
                sender: "Peer_Node_7".to_string(),
                encrypted_payload: encrypted,
                decrypted_payload: msg,
            });
        }
        // Broadcast Tensegrity State
        let is_shedding = rng.random_bool(0.3);
        let _ = tx.send(TelemetryEvent::TensegrityState {
            node: "Node_0".to_string(),
            spin: if is_shedding { -1 } else { 1 },
            temp: 40.0 + rng.random::<f64>() * 5.0,
            load: 1.0,
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
        
        sleep(Duration::from_millis(1500)).await;
    }
}
