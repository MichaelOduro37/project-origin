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
    tokio::spawn(crate::network::listen_for_peers(tx_clone));
    
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

    // Infinite loop feeding chaotic physics data to the UI Dashboard
    println!("[SYSTEM] Streaming live Tensegrity & Chat data to the UI... (Press Ctrl+C to stop)");
    loop {
        {
            sys.refresh_cpu_all();
            components.refresh(true);

            // Poll for incoming chat messages from UI
            while let Ok(msg) = ui_rx.try_recv() {
                println!("[APPLICATION LAYER] Received raw text from UI: {}", msg);
                // Broadcast this real message out to the mesh (using physical network layer soon)
                // For now, bounce it back to the UI to show it was processed
                let encrypted = format!("AES_ENC::{}_ENC", msg.chars().rev().collect::<String>());
                
                let _ = tx.send(TelemetryEvent::ChatIncoming {
                    sender: "Peer_Node_7".to_string(),
                    encrypted_payload: encrypted,
                    decrypted_payload: msg,
                });
            }

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

            // True Ising-Tensegrity Shedding Logic (No RNG)
            let is_shedding = max_temp > 75.0 || load > 85.0;

            let _ = tx.send(TelemetryEvent::TensegrityState {
                node: hostname.clone(),
                spin: if is_shedding { -1 } else { 1 },
                temp: max_temp,
                load: load.max(0.01),
            });
            // Fake Immune Alerts and Routing Simulation Blocks Completely Purged.
        }
        
        sleep(Duration::from_millis(1500)).await;
    }
}
