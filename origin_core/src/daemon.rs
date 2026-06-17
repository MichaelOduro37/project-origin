// ============================================================================
// ORIGIN DAEMON CORE: HARDWARE POLLING AND TELEMETRY
// ============================================================================

use tokio::time::sleep;
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
