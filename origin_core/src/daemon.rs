// ============================================================================
// ORIGIN DAEMON CORE: HARDWARE POLLING AND TELEMETRY
// ============================================================================

use tokio::time::{sleep, Duration};
pub async fn run() {
    println!("===========================================================");
    println!("=== ORIGIN DAEMON RUNNING: LIVE PHYSICAL MODE           ===");
    println!("===========================================================\n");

    use crate::telemetry::{TelemetryServer, TelemetryEvent};
    use crate::updater::SwarmUpdater;
    
    let (telemetry, mut ui_rx) = TelemetryServer::new();
    let tx = telemetry.get_sender();
    let tx_clone = tx.clone();
    let _updater = SwarmUpdater::new();
    
    tokio::spawn(telemetry.start_daemon(9944));
    println!("[SYSTEM] WebSocket Telemetry Daemon running on ws://0.0.0.0:9944");

    // Start Phase 9 LAN Discovery
    let hostname = sysinfo::System::host_name().unwrap_or_else(|| "Origin_Node".to_string());
    tokio::spawn(crate::network::start_discovery_beacon(hostname.clone(), 9944));
    tokio::spawn(crate::network::listen_for_peers(tx_clone, hostname.clone()));
    
    // 10. Start Universal Binary Web UI
    tokio::spawn(async {
        let app = axum::Router::new()
            .route("/*key", axum::routing::get(crate::ui::static_handler))
            .route("/", axum::routing::get(crate::ui::static_handler));
        if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:8081").await {
            println!("[UI DAEMON] Universal UI hosted at http://127.0.0.1:8081");
            let _ = axum::serve(listener, app).await;
        }
    });

    let mut sys = sysinfo::System::new_all();
    let mut components = sysinfo::Components::new_with_refreshed_list();

    let hostname_clone = hostname.clone();
    let tx_ui = tx.clone();
    tokio::spawn(async move {
        loop {
            while let Ok(cmd) = ui_rx.try_recv() {
                match cmd {
                    crate::telemetry::UiCommand::Chat { message } => {
                        println!("[APPLICATION LAYER] Received chat from UI: {}", message);
                        tokio::spawn(crate::network::broadcast_chat(hostname_clone.clone(), message));
                    },
                    crate::telemetry::UiCommand::Upload { file_id, base64_data } => {
                        println!("[HOLO] User uploaded file {} for Holographic Projection", file_id);
                        use base64::{engine::general_purpose, Engine as _};
                        if let Ok(bytes) = general_purpose::STANDARD.decode(&base64_data) {
                            
                            // Phase 15: RMT Chaotic Key Generation
                            let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos().to_le_bytes();
                            let chaotic_key = crate::rmt::ChaoticHamiltonian::generate_key(&seed);
                            
                            let mut encrypted_bytes = Vec::with_capacity(bytes.len());
                            for (i, byte) in bytes.iter().enumerate() {
                                encrypted_bytes.push(byte ^ chaotic_key[i % 32]);
                            }

                            println!("\x1b[35;1m[RMT] Extracted GOE eigenvalue spacings to generate 256-bit Chaotic Key for {}\x1b[0m", file_id);
                            let _ = tx_ui.send(crate::telemetry::TelemetryEvent::RMTKeyGenerated {
                                matrix_size: 32,
                                entropy_bits: 256,
                            });

                            // Phase 8: Disentangle encrypted file into 8 holographic shards
                            let shards = crate::hologram::disentangle(&file_id, &encrypted_bytes, 8);
                            let tx_local = tx_ui.clone();
                            tokio::spawn(crate::network::broadcast_hologram(tx_local, file_id, shards));
                        }
                    },
                    crate::telemetry::UiCommand::HologramRequest { file_id } => {
                        tokio::spawn(crate::network::request_hologram(file_id));
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    });

    // Phase 23: Track history for Lyapunov Exponent Calculation
    let mut load_history: std::collections::VecDeque<f64> = std::collections::VecDeque::with_capacity(10);
    for _ in 0..10 { load_history.push_back(50.0); }

    // Infinite loop feeding chaotic physics data to the UI Dashboard
    println!("[SYSTEM] Streaming live Tensegrity telemetry to the UI... (Press Ctrl+C to stop)");
    loop {
        {
            sys.refresh_cpu_all();
            components.refresh(true);

            let mut max_temp: f64 = 0.0;
            for comp in &components {
                let temp = comp.temperature().unwrap_or(0.0) as f64;
                if temp > max_temp { max_temp = temp; }
            }
            
            let cpus = sys.cpus();
            let mut load = 1.0;
            if !cpus.is_empty() {
                load = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() as f64 / cpus.len() as f64;
            }

            // Fallback: Windows often restricts raw thermal sensor access. 
            if max_temp == 0.0 {
                max_temp = 38.0 + (load * 0.45); // e.g. 38C idle, up to ~83C at 100% load
            }

            // Phase 21: Causal Inference & Do-Calculus Intervention
            // Instead of blind heuristic shedding (is_shedding = load > 85), the Swarm mathematically 
            // simulates P(GlobalHealth | do(shed_load)).
            let base_neighbor_curvature = if rand::random::<f64>() < 0.5 { 0.0 } else { 35.0 }; // Simulate stressed vs unstressed neighbors
            
            let mut is_shedding = false;
            let heuristic_wants_to_shed = max_temp > 75.0 || load > 85.0;

            if heuristic_wants_to_shed {
                let (should_shed_causally, predicted_benefit) = crate::causal_inference::CausalEngine::should_intervene(load, base_neighbor_curvature);
                
                is_shedding = should_shed_causally;
                
                let _ = tx.send(TelemetryEvent::CausalIntervention {
                    action: "ShedLoad".into(),
                    predicted_benefit,
                    executed: is_shedding,
                });
            }

            let _ = tx.send(TelemetryEvent::TensegrityState {
                node: hostname.clone(),
                spin: if is_shedding { -1 } else { 1 },
                temp: max_temp,
                load: load.max(0.01),
            });
            
            // Phase 6: Broadcast Neuromorphic State to UI
            let (membrane_potential, threshold, sleep_interval_ms) = {
                let snn = crate::snn::global_snn().lock().unwrap();
                (snn.membrane_potential, snn.threshold, snn.get_polling_interval())
            };
            let _ = tx.send(TelemetryEvent::SNNState {
                membrane_potential,
                threshold,
                sleep_interval_ms,
            });

            // Phase 13/14: Reservoir Computing and Curvature Regulation
            let (is_alert, k, predicted_k, wormhole_port) = {
                let mut esn = crate::reservoir::global_reservoir().lock().unwrap();
                let mut monitor = crate::curvature::global_curvature().lock().unwrap();
                
                let normalized_load = load / 100.0;
                
                // Step ESN
                esn.step(normalized_load, monitor.curvature_k);
                let predicted_k = esn.predict();

                let alert = monitor.calculate_curvature(normalized_load, predicted_k);
                (alert, monitor.curvature_k, predicted_k, monitor.active_wormhole_port)
            };

            // Always broadcast Curvature state
            let _ = tx.send(TelemetryEvent::CurvatureAlert {
                curvature_k: k,
                predicted_k,
                wormhole_port,
            });

            // Phase 18: VCG Spot Market Auction
            // We simulate a spot compute auction periodically
            if rand::random::<f64>() < 0.3 {
                let auction = crate::vcg_auction::VCGAuction::new(64); // 64 Cores available
                let mut bids = Vec::new();
                for i in 0..5 {
                    let req = (((rand::random::<f64>() * 100.0) as usize) % 16) + 4;
                    // Valuation roughly $2/core, but randomized
                    let val = (req as f64) * 2.0 + (rand::random::<f64>() * 5.0);
                    bids.push(crate::vcg_auction::Bid {
                        agent_id: format!("Agent-{}-{}", hostname, i),
                        resources_requested: req,
                        valuation: val,
                    });
                }
                let winners = auction.resolve(&bids);
                let _ = tx.send(TelemetryEvent::VCGAuctionSettled {
                    winners,
                    total_capacity: 64,
                });
            }

            // Phase 19: Homotopy Type Theory & Proof-Carrying Data
            if rand::random::<f64>() < 0.15 {
                let file_id = format!("quantum_archive_{}", (rand::random::<f64>() * 1000.0) as usize);
                let plan = crate::proof_carrying_data::ShardMigrationPlan {
                    file_id: file_id.clone(),
                    source_nodes: vec!["Origin-Alpha".into()],
                    target_nodes: vec![hostname.clone()],
                };

                // The Sender creates a mathematical proof that the migration preserves the 8-shard replication invariant
                let artifact = crate::proof_carrying_data::HoTTVerifier::create_migration_artifact(plan, 8);

                // The Daemon strictly verifies the zero-trust mathematical proof in O(1) time before accepting the topology shift
                let is_valid = crate::proof_carrying_data::HoTTVerifier::verify_migration(&artifact).is_ok();
                
                let _ = tx.send(TelemetryEvent::ProofVerified {
                    file_id,
                    is_valid,
                });
            }

            // Phase 20: Sparse Representations & Compressed Sensing
            if rand::random::<f64>() < 0.2 {
                // Collect a high-dimensional dense state vector (e.g., 100 features from the node)
                let input_dim = 100;
                let output_dim = 10; // Compress by 10x!
                
                let mut dense_state = vec![0.0; input_dim];
                for i in 0..input_dim {
                    // Simulate collecting thermodynamics, SN potentials, network loads, etc.
                    dense_state[i] = (i as f64).sin() * max_temp + (load * 0.1);
                }

                // Generate Measurement Matrix (in a real system, seed is shared globally)
                let phi = crate::compressed_sensing::MeasurementMatrix::new(input_dim, output_dim, 999);
                
                // Compress the 100D state into a 10D sketch
                let sketch = phi.compress(&dense_state);

                let _ = tx.send(TelemetryEvent::CompressedTelemetrySnapshot {
                    snapshot: crate::compressed_sensing::CompressedTelemetrySnapshot {
                        original_dim: input_dim,
                        compressed_dim: output_dim,
                        sketch,
                    }
                });
            }

            // Phase 22: Category Theory (Compositionality & Interfaces)
            if rand::random::<f64>() < 0.15 {
                let mut cat = crate::category_theory::SchemaCategory::new();
                cat.add_morphism("whisper_adapter", "AudioCell", "TextSchema");
                cat.add_morphism("bert_adapter", "TextSchema", "EmbeddingCell");
                cat.add_morphism("compression_adapter", "EmbeddingCell", "SparseSketch");
                
                // Simulate an attempt to bind an Audio micro-cell to a SparseSketch ML index
                let composition = cat.compose("AudioCell", "SparseSketch");
                
                let (path_str, is_valid) = match composition {
                    Some(morphisms) => {
                        let names: Vec<String> = morphisms.iter().map(|m| m.name.clone()).collect();
                        (names.join(" ∘ "), true)
                    },
                    None => ("None".to_string(), false),
                };

                let _ = tx.send(TelemetryEvent::CategoricalComposition {
                    cell_a: "AudioCell".into(),
                    cell_b: "SparseSketch".into(),
                    morphism_path: path_str,
                    is_valid,
                });
            }

            // Phase 23: Complexity Synchronization as Distributed Control
            load_history.pop_front();
            load_history.push_back(load);
            
            // Only broadcast complexity sync periodically
            if rand::random::<f64>() < 0.3 {
                let history_slice: Vec<f64> = load_history.iter().copied().collect();
                let local_lyapunov = crate::complexity_sync::ComplexityEngine::calculate_lyapunov_exponent(&history_slice);
                
                // Simulate a network consensus target (e.g., Swarm wants slight stability)
                let network_target = -0.5;
                
                let action = crate::complexity_sync::ComplexityEngine::synchronize(local_lyapunov, network_target, load);
                
                let action_str = match action {
                    crate::complexity_sync::LoadAction::PullLoad(amt) => format!("Pulling {:.1}% Load", amt),
                    crate::complexity_sync::LoadAction::ShedLoad(amt) => format!("Shedding {:.1}% Load", amt),
                    crate::complexity_sync::LoadAction::Stable => "Stable Equilibrium".to_string(),
                };

                let _ = tx.send(TelemetryEvent::ComplexitySync {
                    lyapunov_exponent: local_lyapunov,
                    target: network_target,
                    action: action_str,
                });
            }

        }
        
        sleep(Duration::from_millis(1500)).await;
    }
}
