// ============================================================================
// ORIGIN DAEMON CORE: HARDWARE POLLING AND TELEMETRY
// ============================================================================

pub async fn run() {
    println!("===========================================================");
    println!("=== ORIGIN DAEMON RUNNING: LIVE PHYSICAL MODE           ===");
    println!("===========================================================\n");

    use crate::telemetry::TelemetryServer;
    use crate::updater::SwarmUpdater;

    let (telemetry, mut ui_rx) = TelemetryServer::new();
    let tx = telemetry.get_sender();
    let tx_clone = tx.clone();
    let _updater = SwarmUpdater::new();

    tokio::spawn(telemetry.start_daemon(9944));
    println!("[SYSTEM] WebSocket Telemetry Daemon running on ws://0.0.0.0:9944");

    // Start Phase 9 LAN Discovery
    let hostname = sysinfo::System::host_name().unwrap_or_else(|| "Origin_Node".to_string());
    tokio::spawn(crate::network::start_discovery_beacon(
        hostname.clone(),
        9944,
    ));
    tokio::spawn(crate::network::listen_for_peers(tx_clone, hostname.clone()));

    // =========================================================================
    // CNTP: Chemotactic NAT Traversal Protocol Bootstrap
    // Generate persistent node key + run Layer 1 self-discovery
    // =========================================================================
    let cntp_node_key: [u8; 32] = {
        use sha2::{Sha256, Digest};
        let key_path = std::path::PathBuf::from("origin_node_key.bin");
        if let Ok(saved) = std::fs::read(&key_path) {
            if saved.len() == 32 {
                let mut key = [0u8; 32];
                key.copy_from_slice(&saved);
                println!("[CNTP] Loaded persistent node key from disk.");
                key
            } else {
                let mut hasher = Sha256::new();
                hasher.update(hostname.as_bytes());
                hasher.update(&rand::random::<[u8; 16]>());
                let result = hasher.finalize();
                let mut key = [0u8; 32];
                key.copy_from_slice(&result);
                let _ = std::fs::write(&key_path, &key);
                println!("[CNTP] Generated new persistent node key.");
                key
            }
        } else {
            let mut hasher = Sha256::new();
            hasher.update(hostname.as_bytes());
            hasher.update(&rand::random::<[u8; 16]>());
            let result = hasher.finalize();
            let mut key = [0u8; 32];
            key.copy_from_slice(&result);
            let _ = std::fs::write(&key_path, &key);
            println!("[CNTP] Generated new persistent node key.");
            key
        }
    };

    let node_key_hex = hex::encode(cntp_node_key);
    println!("[CNTP] Node Key: {}", node_key_hex);

    // Send node key to UI (with delay so WS connects first)
    let tx_cntp = tx.clone();
    let key_hex_for_ui = node_key_hex.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let _ = tx_cntp.send(crate::telemetry::TelemetryEvent::CntpNodeKey {
            key: key_hex_for_ui,
        });
    });

    // Run Layer 1: Chemotactic Self-Discovery
    let tx_cntp2 = tx.clone();
    let key_hex_for_discovery = node_key_hex.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        if let Some(identity) = crate::cntp::chemotactic_self_discover().await {
            let ip_str = format!(
                "{}.{}.{}.{}",
                identity.public_ip[0], identity.public_ip[1],
                identity.public_ip[2], identity.public_ip[3]
            );
            let nat_str = format!("{:?}", identity.nat_type);
            println!(
                "[CNTP] Full Node Key with IP: {}@{}",
                key_hex_for_discovery, ip_str
            );
            let _ = tx_cntp2.send(crate::telemetry::TelemetryEvent::CntpSelfDiscovered {
                public_ip: ip_str.clone(),
                nat_type: nat_str,
            });
            // Update the displayed key to include IP
            let _ = tx_cntp2.send(crate::telemetry::TelemetryEvent::CntpNodeKey {
                key: format!("{}@{}", key_hex_for_discovery, ip_str),
            });
        }
    });

    // 10. Start Universal Binary Web UI
    tokio::spawn(async {
        let app = axum::Router::new()
            .route("/*key", axum::routing::get(crate::ui::static_handler))
            .route("/", axum::routing::get(crate::ui::static_handler));
        if let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:8081").await {
            println!("[UI DAEMON] Universal UI hosted at http://0.0.0.0:8081");
            let _ = axum::serve(listener, app).await;
        }
    });

    let _sys = sysinfo::System::new_all();
    let _components = sysinfo::Components::new_with_refreshed_list();

    let hostname_clone = hostname.clone();
    let tx_ui = tx.clone();
    let cntp_key_for_handler = cntp_node_key;
    tokio::spawn(async move {
        loop {
            while let Ok(cmd) = ui_rx.try_recv() {
                match cmd {
                    crate::telemetry::UiCommand::Chat { message } => {
                        println!("[APPLICATION LAYER] Received chat from UI: {}", message);
                        tokio::spawn(crate::network::broadcast_chat(
                            hostname_clone.clone(),
                            message,
                        ));
                    }
                    crate::telemetry::UiCommand::Upload {
                        file_id,
                        base64_data,
                    } => {
                        println!(
                            "[HOLO] User uploaded file {} for Holographic Projection",
                            file_id
                        );
                        use base64::{engine::general_purpose, Engine as _};
                        if let Ok(bytes) = general_purpose::STANDARD.decode(&base64_data) {
                            // Phase 15: RMT Chaotic Key Generation
                            let seed = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_nanos()
                                .to_le_bytes();
                            let chaotic_key = crate::logos::advanced_mathematics::rmt::ChaoticHamiltonian::generate_key(&seed);

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
                            let shards = crate::cosmos::grand_unification::hologram::disentangle(
                                &file_id,
                                &encrypted_bytes,
                                8,
                            );
                            let tx_local = tx_ui.clone();
                            tokio::spawn(crate::network::broadcast_hologram(
                                tx_local, file_id, shards,
                            ));
                        }
                    }
                    crate::telemetry::UiCommand::HologramRequest { file_id } => {
                        tokio::spawn(crate::network::request_hologram(file_id));
                    }
                    crate::telemetry::UiCommand::CntpConnect { peer_key } => {
                        println!("[CNTP] UI initiated cross-network punch to: {}", peer_key);
                        let my_key = cntp_key_for_handler;
                        let tx_cntp_cmd = tx_ui.clone();
                        tokio::spawn(async move {
                            // Parse peer key: "<hex_pubkey>@<ip>" or just "<hex_pubkey>"
                            let (peer_pubkey_hex, peer_ip_str) = if peer_key.contains('@') {
                                let parts: Vec<&str> = peer_key.split('@').collect();
                                (parts[0].to_string(), Some(parts[1].to_string()))
                            } else {
                                (peer_key.clone(), None)
                            };

                            let peer_pubkey_bytes = match hex::decode(&peer_pubkey_hex) {
                                Ok(b) if b.len() == 32 => {
                                    let mut arr = [0u8; 32];
                                    arr.copy_from_slice(&b);
                                    arr
                                }
                                _ => {
                                    let _ = tx_cntp_cmd.send(
                                        crate::telemetry::TelemetryEvent::CntpConnectionFailed {
                                            reason: "Invalid key. Use 64-char hex or hex@IP format.".into(),
                                        },
                                    );
                                    return;
                                }
                            };

                            let peer_ip: Option<[u8; 4]> = peer_ip_str.and_then(|s| {
                                let parts: Vec<&str> = s.split('.').collect();
                                if parts.len() == 4 {
                                    Some([
                                        parts[0].parse().ok()?,
                                        parts[1].parse().ok()?,
                                        parts[2].parse().ok()?,
                                        parts[3].parse().ok()?,
                                    ])
                                } else {
                                    None
                                }
                            });

                            let peer_identity = crate::cntp::PeerIdentity {
                                public_key: peer_pubkey_bytes,
                                known_public_ip: peer_ip,
                            };

                            let (event_tx, mut event_rx) =
                                tokio::sync::mpsc::unbounded_channel();

                            // Forward CNTP events to UI telemetry
                            let tx_fwd = tx_cntp_cmd.clone();
                            tokio::spawn(async move {
                                while let Some(event) = event_rx.recv().await {
                                    match event {
                                        crate::cntp::CntpEvent::SelfDiscovered(id) => {
                                            let _ = tx_fwd.send(crate::telemetry::TelemetryEvent::CntpSelfDiscovered {
                                                public_ip: format!("{}.{}.{}.{}", id.public_ip[0], id.public_ip[1], id.public_ip[2], id.public_ip[3]),
                                                nat_type: format!("{:?}", id.nat_type),
                                            });
                                        }
                                        crate::cntp::CntpEvent::PunchAttempt { layer, ports_tried } => {
                                            let _ = tx_fwd.send(crate::telemetry::TelemetryEvent::CntpPunchAttempt { layer, ports_tried });
                                        }
                                        crate::cntp::CntpEvent::PeerConnected { peer_addr } => {
                                            let _ = tx_fwd.send(crate::telemetry::TelemetryEvent::CntpPeerConnected {
                                                peer_addr: peer_addr.to_string(),
                                            });
                                        }
                                        crate::cntp::CntpEvent::ConnectionFailed { reason } => {
                                            let _ = tx_fwd.send(crate::telemetry::TelemetryEvent::CntpConnectionFailed { reason });
                                        }
                                    }
                                }
                            });

                            let result = crate::cntp::connect_to_peer(
                                &my_key,
                                &peer_identity,
                                event_tx,
                            ).await;

                            if let Some((_socket, addr)) = result {
                                println!("\x1b[32;1m[CNTP] ✓ DIRECT P2P TUNNEL LIVE: {}\x1b[0m", addr);
                            }
                        });
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    });

    // Keep the daemon alive listening for real network events
    println!("[SYSTEM] Node operating in purely physical mode. Waiting for real network events...");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
