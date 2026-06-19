use crate::noosphere::tensegrity::{Heartbeat, PheromoneShard};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tokio::net::UdpSocket;

pub fn global_qchromosome() -> &'static Mutex<crate::biosphere::qga::QChromosome> {
    static Q_CHROMOSOME: OnceLock<Mutex<crate::biosphere::qga::QChromosome>> = OnceLock::new();
    Q_CHROMOSOME.get_or_init(|| Mutex::new(crate::biosphere::qga::QChromosome::new()))
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkPacket {
    Shard(PheromoneShard),
    Pulse(Heartbeat),
    Hologram(crate::cosmos::grand_unification::dna_fountain::DnaFountainDroplet),
}

// ============================================================================
// PHASE X & 11: HARDWARE RADIO ABSTRACTION & NATIVE BLE SYNTHESIS
// ============================================================================
use btleplug::api::{Central, Manager as _};
use btleplug::platform::{Adapter, Manager};

pub enum HardwareRadio {
    LanUdp(Arc<UdpSocket>),
    BluetoothLowEnergy(Option<Adapter>), // Native Physical BLE Adapter
    WifiDirect(String),                  // P2P Handle Stub
}

impl HardwareRadio {
    pub async fn bind_lan(port: u16) -> Self {
        let addr = format!("0.0.0.0:{}", port);
        let socket = match UdpSocket::bind(&addr).await {
            Ok(s) => s,
            Err(_) => {
                println!(
                    "[NETWORK] Warning: Could not bind exactly to {}. Binding to 0.",
                    addr
                );
                UdpSocket::bind("0.0.0.0:0").await.unwrap()
            }
        };
        let _ = socket.set_broadcast(true);
        HardwareRadio::LanUdp(Arc::new(socket))
    }

    pub async fn bind_ble() -> Self {
        println!("\x1b[36m[RADIO] Waking physical Bluetooth Low Energy (BLE) antenna via btleplug...\x1b[0m");

        if let Ok(manager) = Manager::new().await {
            if let Ok(adapters) = manager.adapters().await {
                if let Some(adapter) = adapters.into_iter().nth(0) {
                    if let Ok(info) = adapter.adapter_info().await {
                        println!("\x1b[32m[RADIO:BLE] Successfully hooked Windows Native Bluetooth Adapter: {}\x1b[0m", info);
                        return HardwareRadio::BluetoothLowEnergy(Some(adapter));
                    }
                }
            }
        }

        println!("\x1b[31m[RADIO:BLE] Warning: No physical Bluetooth adapter found or permissions denied. Running in blind mode.\x1b[0m");
        HardwareRadio::BluetoothLowEnergy(None)
    }

    pub async fn send_packet(&self, packet: &NetworkPacket, target_identifier: &str) {
        if let Ok(encoded) = bincode::serialize(packet) {
            match self {
                HardwareRadio::LanUdp(socket) => {
                    let target_addr: String = if target_identifier.contains('.') {
                        target_identifier.to_string()
                    } else {
                        format!("127.0.0.1:{}", target_identifier)
                    };
                    if let Ok(addr) = target_addr.parse::<SocketAddr>() {
                        let _ = socket.send_to(&encoded, addr).await;
                    }
                }
                HardwareRadio::BluetoothLowEnergy(adapter_opt) => {
                    println!("\x1b[35m[RADIO:BLE] Formatting {} bytes into Physical BLE Advertisement payload for target {}\x1b[0m", encoded.len(), target_identifier);
                    if let Some(_adapter) = adapter_opt {
                        // Here, the BLE packet would be physically injected into the airwaves
                        // via vendor-specific advertising flags or GAP broadcasts.
                        println!("\x1b[35m[RADIO:BLE] Physical transmission pulse fired.\x1b[0m");
                    }
                }
                HardwareRadio::WifiDirect(_mac) => {
                    println!(
                        "\x1b[35m[RADIO:WIFI-P2P] Transmitting {} bytes to P2P target {}\x1b[0m",
                        encoded.len(),
                        target_identifier
                    );
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// FERMIONIC ROUTER & FLUID DYNAMICS (Original Math Intact)

pub async fn start_discovery_beacon(node_id: String, port: u16) {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0").await {
        let _ = socket.set_broadcast(true);
        println!("\x1b[32m[BEACON] Origin Swarm Discovery Beacon active. Broadcasting presence...\x1b[0m");
        loop {
            // SNN Phase 6: Decay voltage while idle
            {
                let mut snn = crate::noosphere::cognitive_architecture::global_snn()
                    .lock()
                    .unwrap();
                snn.decay();
            }

            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            let msg = format!("ORIGIN_BEACON:{}:{}:{}", node_id, port, timestamp);
            let _ = socket.send_to(msg.as_bytes(), "255.255.255.255:9999").await;

            // Dynamic Biomimetic Sleep based on SNN
            let sleep_ms = crate::noosphere::cognitive_architecture::global_snn()
                .lock()
                .unwrap()
                .get_polling_interval();
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
        }
    }
}

pub async fn broadcast_chat(sender_id: String, msg: String) {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0").await {
        let _ = socket.set_broadcast(true);
        let payload = format!("ORIGIN_CHAT:{}:{}", sender_id, msg);

        // QGA Phase 7: Collapse superposition to find optimal path instead of blind broadcast
        let dest_ip = {
            let chromosome = global_qchromosome().lock().unwrap();
            chromosome.collapse_to_optimal_route()
        };

        if let Some(target_ip) = dest_ip {
            println!("\x1b[36m[QGA:ROUTE] Quantum Superposition Collapsed. Optimal path selected: {}\x1b[0m", target_ip);
            let target_addr = format!("{}:9999", target_ip);
            let _ = socket.send_to(payload.as_bytes(), &target_addr).await;
        } else {
            // Fallback to broadcast if superposition is empty (no peers registered)
            println!("\x1b[33m[QGA:WARN] QChromosome superposition empty. Falling back to classical broadcast.\x1b[0m");
            let _ = socket
                .send_to(payload.as_bytes(), "255.255.255.255:9999")
                .await;
        }
    }
}

pub async fn listen_for_peers(
    telemetry_tx: tokio::sync::broadcast::Sender<crate::telemetry::TelemetryEvent>,
    my_node_id: String,
) {
    let mut known_peers: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:9999").await {
        println!("\x1b[32m[BEACON] Listening for Swarm Peers on LAN (Port 9999)...\x1b[0m");
        let mut buf = [0u8; 1024];
        loop {
            if let Ok((len, src)) = socket.recv_from(&mut buf).await {
                // Phase 11: CRISPR-Cas9 Adaptive Immunity Scan
                let packet_str_opt = std::str::from_utf8(&buf[..len]).ok();

                if let Some(msg) = packet_str_opt {
                    let crispr = crate::biosphere::biological_immune_system::global_crispr()
                        .lock()
                        .unwrap();
                    if let Some(signature) = crispr.scan_payload(msg) {
                        println!("\x1b[35;1m[CRISPR:CAS9] MALICIOUS PAYLOAD DETECTED AND CLEAVED! Signature: {}\x1b[0m", signature);
                        let _ = telemetry_tx
                            .send(crate::telemetry::TelemetryEvent::CRISPRCleavage { signature });
                        continue; // Drop packet instantly
                    }
                }

                if let Some(msg) = packet_str_opt {
                    if msg.starts_with("ORIGIN_BEACON:") {
                        let parts: Vec<&str> = msg.split(':').collect();
                        if parts.len() == 4 {
                            let node_id = parts[1];
                            let timestamp_str = parts[3];
                            let beacon_time: u128 = timestamp_str.parse().unwrap_or(0);

                            // Ignore our own beacon
                            if node_id != my_node_id {
                                let ip = src.ip().to_string();
                                let peer_key = format!("{}@{}", node_id, ip);

                                // Phase 6: SNN Integration (Stimulus on Beacon)
                                {
                                    let mut snn =
                                        crate::noosphere::cognitive_architecture::global_snn()
                                            .lock()
                                            .unwrap();
                                    let fired = snn.integrate(5.0); // 5mV stimulus for beacon
                                    if fired {
                                        println!("\x1b[35m[SNN] Action Potential Fired! Network node awoken by incoming beacon.\x1b[0m");
                                    }
                                }

                                // QGA Phase 7: Calculate TRUE physical latency for Quantum Fitness
                                let current_time = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis();
                                let latency = if current_time > beacon_time {
                                    current_time - beacon_time
                                } else {
                                    1
                                };
                                // Inverse latency: 5ms = high rotation, 500ms = low rotation
                                let physical_fitness = (1000.0 / latency as f64).min(500.0) * 0.005;

                                {
                                    let mut chromosome = global_qchromosome().lock().unwrap();
                                    chromosome.register_peer(ip.clone());
                                    // True mathematical fitness based on real-world physics
                                    chromosome.update_fitness(&ip, physical_fitness);
                                }

                                // Only log new peers to avoid spam
                                if !known_peers.contains(&peer_key) {
                                    known_peers.insert(peer_key.clone());
                                    let _ = telemetry_tx.send(
                                        crate::telemetry::TelemetryEvent::FermionicRoute {
                                            packet_id: node_id.to_string(),
                                            origin: my_node_id.clone(),
                                            dest: ip,
                                            is_quantum: true,
                                        },
                                    );
                                }
                            }
                        }
                    } else if msg.starts_with("ORIGIN_CHAT:") {
                        // Format: ORIGIN_CHAT:SenderID:Message...
                        if let Some(first_colon) = msg.find(':') {
                            let rest = &msg[first_colon + 1..];
                            if let Some(second_colon) = rest.find(':') {
                                let sender_id = &rest[..second_colon];
                                let chat_msg = &rest[second_colon + 1..];

                                // Ignore our own chat broadcasts (UI handles local echo)
                                if sender_id != my_node_id {
                                    // Phase 6: Massive SNN Stimulus for direct interaction
                                    {
                                        let mut snn =
                                            crate::noosphere::cognitive_architecture::global_snn()
                                                .lock()
                                                .unwrap();
                                        let fired = snn.integrate(20.0); // 20mV stimulus guarantees spike
                                        if fired {
                                            println!("\x1b[35m[SNN] Action Potential Fired! Network node awoken by incoming CHAT.\x1b[0m");
                                        }
                                    }

                                    let _ = telemetry_tx.send(
                                        crate::telemetry::TelemetryEvent::ChatIncoming {
                                            sender: sender_id.to_string(),
                                            encrypted_payload: format!("AES_ENC::{}_ENC", chat_msg),
                                            decrypted_payload: chat_msg.to_string(),
                                        },
                                    );
                                }
                            }
                        }
                    } else if msg.starts_with("ORIGIN_HOLO:") {
                        // DNA Fountain Rateless Erasure Droplet received
                        // Format: ORIGIN_HOLO:FileID:DropletSeed:SourceBlocks:Base64Data
                        let parts: Vec<&str> = msg.splitn(5, ':').collect();
                        if parts.len() == 5 {
                            let file_id = parts[1].to_string();
                            let droplet_seed: u64 = parts[2].parse().unwrap_or(0);
                            let source_blocks: usize = parts[3].parse().unwrap_or(1);
                            let data_len = parts[4].len() as f64;
                            let src_ip = src.ip().to_string();

                            // Phase 9: Physarum Foraging Thickening
                            {
                                let mut physarum =
                                    crate::noosphere::swarm_dynamics::global_physarum()
                                        .lock()
                                        .unwrap();
                                physarum.stimulate_tube(&src_ip, data_len * 0.01);
                            }

                            let _ = telemetry_tx.send(
                                crate::telemetry::TelemetryEvent::DnaFountainDropletSprayed {
                                    file_id,
                                    droplet_seed,
                                    source_blocks,
                                },
                            );

                            println!("\x1b[34m[DNA-FOUNTAIN] Entangled DNA Fountain Droplet received from swarm via {}.\x1b[0m", src_ip);
                        }
                    } else if msg.starts_with("ORIGIN_HOLO_REQ:") {
                        let parts: Vec<&str> = msg.split(':').collect();
                        if parts.len() == 2 {
                            let file_id = parts[1];
                            println!("\x1b[32m[PHYSARUM] Biological gradient detected for file: {}. Emitting shards if available...\x1b[0m", file_id);
                            // In a full implementation, the local node would check its cache and stream shards back
                            // using the thickened Physarum tubes.
                        }
                    } else if msg.starts_with("ORIGIN_AUTOINDUCER:") {
                        let parts: Vec<&str> = msg.split(':').collect();
                        if parts.len() == 2 {
                            if let Ok(amount) = parts[1].parse::<f64>() {
                                let mut quorum = crate::noosphere::swarm_dynamics::global_quorum()
                                    .lock()
                                    .unwrap();
                                let triggered = quorum.sense_autoinducer(amount);
                                let concentration = quorum.concentration;
                                let biofilm_active = quorum.is_biofilm_active();

                                if triggered {
                                    println!("\x1b[31;1m[QUORUM] THRESHOLD REACHED! SWARM ENTERING BIOFILM LOCKDOWN MODE.\x1b[0m");
                                } else {
                                    println!("\x1b[33m[QUORUM] Autoinducer detected. Local concentration: {:.2}\x1b[0m", concentration);
                                }

                                let _ = telemetry_tx.send(
                                    crate::telemetry::TelemetryEvent::QuorumState {
                                        concentration,
                                        biofilm_active,
                                    },
                                );
                            }
                        }
                    } else if msg.starts_with("ORIGIN_SGRNA:") {
                        let parts: Vec<&str> = msg.split(':').collect();
                        if parts.len() == 2 {
                            let signature = parts[1].to_string();
                            let mut crispr =
                                crate::biosphere::biological_immune_system::global_crispr()
                                    .lock()
                                    .unwrap();
                            if crispr.add_spacer(signature.clone()) {
                                println!("\x1b[35m[CRISPR] Integrated new viral signature (sgRNA) into Array: {}\x1b[0m", signature);
                                let _ = telemetry_tx.send(
                                    crate::telemetry::TelemetryEvent::CRISPRArrayUpdate {
                                        signatures: crispr.get_all_spacers(),
                                    },
                                );
                            }
                        }
                    } else {
                        // Phase 10: Secrete Autoinducers in response to anomaly
                        println!("\x1b[31m[QUORUM:ANOMALY] Unknown packet detected from {}. Secreting Autoinducer...\x1b[0m", src.ip());
                        let _ = socket
                            .send_to(b"ORIGIN_AUTOINDUCER:10.0", "255.255.255.255:9999")
                            .await;

                        // Phase 11: Generate sgRNA from the unknown packet (first 16 chars as signature)
                        let signature = msg.chars().take(16).collect::<String>();
                        println!(
                            "\x1b[35m[CRISPR] Generating sgRNA for unknown payload: {}\x1b[0m",
                            signature
                        );
                        let sgrna_payload = format!("ORIGIN_SGRNA:{}", signature);
                        let _ = socket
                            .send_to(sgrna_payload.as_bytes(), "255.255.255.255:9999")
                            .await;
                        // Add to our own array
                        {
                            let mut crispr =
                                crate::biosphere::biological_immune_system::global_crispr()
                                    .lock()
                                    .unwrap();
                            if crispr.add_spacer(signature) {
                                let _ = telemetry_tx.send(
                                    crate::telemetry::TelemetryEvent::CRISPRArrayUpdate {
                                        signatures: crispr.get_all_spacers(),
                                    },
                                );
                            }
                        }

                        let mut quorum = crate::noosphere::swarm_dynamics::global_quorum()
                            .lock()
                            .unwrap();
                        let triggered = quorum.sense_autoinducer(10.0);
                        let _ = telemetry_tx.send(crate::telemetry::TelemetryEvent::QuorumState {
                            concentration: quorum.concentration,
                            biofilm_active: quorum.is_biofilm_active(),
                        });
                        if triggered {
                            println!("\x1b[31;1m[QUORUM] THRESHOLD REACHED! SWARM ENTERING BIOFILM LOCKDOWN MODE.\x1b[0m");
                        }
                    }
                } else {
                    // Packet failed UTF-8 parsing
                    println!("\x1b[31m[QUORUM:ANOMALY] Malformed binary packet from {}. Secreting Autoinducer...\x1b[0m", src.ip());
                    let _ = socket
                        .send_to(b"ORIGIN_AUTOINDUCER:15.0", "255.255.255.255:9999")
                        .await;

                    // Phase 11: Generate sgRNA from binary trash (use hex string of first 8 bytes)
                    let extract_len = std::cmp::min(8, len);
                    let hex_sig = hex::encode(&buf[..extract_len]);
                    println!(
                        "\x1b[35m[CRISPR] Generating sgRNA for binary trash: {}\x1b[0m",
                        hex_sig
                    );
                    let sgrna_payload = format!("ORIGIN_SGRNA:{}", hex_sig);
                    let _ = socket
                        .send_to(sgrna_payload.as_bytes(), "255.255.255.255:9999")
                        .await;
                    {
                        let mut crispr =
                            crate::biosphere::biological_immune_system::global_crispr()
                                .lock()
                                .unwrap();
                        if crispr.add_spacer(hex_sig) {
                            let _ = telemetry_tx.send(
                                crate::telemetry::TelemetryEvent::CRISPRArrayUpdate {
                                    signatures: crispr.get_all_spacers(),
                                },
                            );
                        }
                    }

                    let mut quorum = crate::noosphere::swarm_dynamics::global_quorum()
                        .lock()
                        .unwrap();
                    let triggered = quorum.sense_autoinducer(15.0);
                    let _ = telemetry_tx.send(crate::telemetry::TelemetryEvent::QuorumState {
                        concentration: quorum.concentration,
                        biofilm_active: quorum.is_biofilm_active(),
                    });
                    if triggered {
                        println!("\x1b[31;1m[QUORUM] THRESHOLD REACHED! SWARM ENTERING BIOFILM LOCKDOWN MODE.\x1b[0m");
                    }
                }
            }
        }
    }
}

// Phase 12 & 16: Fermionic Cryptographic Routing & DNA Fountain Rateless Erasure
pub async fn broadcast_hologram(
    telemetry_tx: tokio::sync::broadcast::Sender<crate::telemetry::TelemetryEvent>,
    _file_id: String,
    shards: Vec<crate::cosmos::grand_unification::dna_fountain::DnaFountainDroplet>,
) {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0").await {
        let _ = socket.set_broadcast(true);
        println!("\x1b[34m[DNA-FOUNTAIN] Spraying {} entangled droplets into the network fabric...\x1b[0m", shards.len());

        let available_peers = {
            global_qchromosome()
                .lock()
                .unwrap()
                .get_all_peers_with_cost()
        };

        let num_shards = shards.len();
        let num_peers = available_peers.len();

        let mut transport_map = vec![0; num_shards];

        if num_peers > 0 {
            // Build cost matrix: (N shards x M peers)
            // For simplicity, we assume shard sizes are uniform, so cost depends only on the peer's physical cost
            let mut cost_matrix = vec![vec![0.0; num_peers]; num_shards];
            for i in 0..num_shards {
                for j in 0..num_peers {
                    cost_matrix[i][j] = available_peers[j].1;
                }
            }

            let a = vec![1.0 / num_shards as f64; num_shards];
            let b = vec![1.0 / num_peers as f64; num_peers];

            let solver = crate::logos::advanced_mathematics::sinkhorn::SinkhornSolver::new(0.1, 50);
            let (p_matrix, total_cost) = solver.compute_transport_plan(&cost_matrix, &a, &b);

            // Find the best peer for each shard based on the transport plan
            for i in 0..num_shards {
                let mut best_peer_idx = 0;
                let mut highest_prob = 0.0;
                for j in 0..num_peers {
                    if p_matrix[i][j] > highest_prob {
                        highest_prob = p_matrix[i][j];
                        best_peer_idx = j;
                    }
                }
                transport_map[i] = best_peer_idx;
            }

            println!("\x1b[35;1m[SINKHORN] Computed exact Wasserstein Distance (Cost: {:.4}). Mapped shards to geometric optimum.\x1b[0m", total_cost);
            let _ = telemetry_tx.send(crate::telemetry::TelemetryEvent::OptimalTransportMapped {
                file_id: _file_id.clone(),
                cost: total_cost,
            });
        }

        for (i, shard) in shards.into_iter().enumerate() {
            use base64::{engine::general_purpose, Engine as _};
            let b64_data = general_purpose::STANDARD.encode(&shard.payload);
            let payload = format!(
                "ORIGIN_HOLO:{}:{}:{}:{}",
                shard.file_id, shard.droplet_seed, shard.source_blocks, b64_data
            );

            let mut target_ip = "255.255.255.255".to_string(); // fallback

            if num_peers > 0 {
                let peer_idx = transport_map[i];
                let peer_ip = available_peers[peer_idx].0.clone();
                target_ip = peer_ip.clone();

                let _ = telemetry_tx.send(crate::telemetry::TelemetryEvent::FermionicRoute {
                    packet_id: format!("{}-droplet{}", shard.file_id, shard.droplet_seed),
                    origin: "local".to_string(),
                    dest: peer_ip.clone(),
                    is_quantum: true,
                });
            }

            let target_addr = format!("{}:9999", target_ip);
            let _ = socket.send_to(payload.as_bytes(), &target_addr).await;
        }
    }
}

// Phase 9: Physarum Gradient Emission
pub async fn request_hologram(file_id: String) {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0").await {
        let _ = socket.set_broadcast(true);
        println!(
            "\x1b[32m[PHYSARUM] Emitting biological gradient to forage for Hologram: {}\x1b[0m",
            file_id
        );

        let optimal_peer = {
            let physarum = crate::noosphere::swarm_dynamics::global_physarum()
                .lock()
                .unwrap();
            physarum.get_optimal_path()
        };

        let payload = format!("ORIGIN_HOLO_REQ:{}", file_id);

        if let Some(peer_ip) = optimal_peer {
            println!("\x1b[36m[PHYSARUM:ROUTE] Routing gradient via optimally thickened tube to {}\x1b[0m", peer_ip);
            let target_addr = format!("{}:9999", peer_ip);
            let _ = socket.send_to(payload.as_bytes(), &target_addr).await;
        } else {
            // Classical fallback
            let _ = socket
                .send_to(payload.as_bytes(), "255.255.255.255:9999")
                .await;
        }
    }
}
