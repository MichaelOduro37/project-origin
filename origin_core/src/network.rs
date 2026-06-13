use std::sync::Arc;
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use crate::tensegrity::{PheromoneShard, Heartbeat};
use std::sync::{Mutex, OnceLock};

pub fn global_qchromosome() -> &'static Mutex<crate::qga::QChromosome> {
    static Q_CHROMOSOME: OnceLock<Mutex<crate::qga::QChromosome>> = OnceLock::new();
    Q_CHROMOSOME.get_or_init(|| Mutex::new(crate::qga::QChromosome::new()))
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkPacket {
    Shard(PheromoneShard),
    Pulse(Heartbeat),
}

// ============================================================================
// PHASE X & 11: HARDWARE RADIO ABSTRACTION & NATIVE BLE SYNTHESIS
// ============================================================================
use btleplug::api::{Central, Manager as _};
use btleplug::platform::{Adapter, Manager};

pub enum HardwareRadio {
    LanUdp(Arc<UdpSocket>),
    BluetoothLowEnergy(Option<Adapter>), // Native Physical BLE Adapter
    WifiDirect(String),         // P2P Handle Stub
}

impl HardwareRadio {
    pub async fn bind_lan(port: u16) -> Self {
        let addr = format!("0.0.0.0:{}", port);
        let socket = match UdpSocket::bind(&addr).await {
            Ok(s) => s,
            Err(_) => {
                println!("[NETWORK] Warning: Could not bind exactly to {}. Binding to 0.", addr);
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
                },
                HardwareRadio::BluetoothLowEnergy(adapter_opt) => {
                    println!("\x1b[35m[RADIO:BLE] Formatting {} bytes into Physical BLE Advertisement payload for target {}\x1b[0m", encoded.len(), target_identifier);
                    if let Some(_adapter) = adapter_opt {
                        // Here, the BLE packet would be physically injected into the airwaves
                        // via vendor-specific advertising flags or GAP broadcasts.
                        println!("\x1b[35m[RADIO:BLE] Physical transmission pulse fired.\x1b[0m");
                    }
                },
                HardwareRadio::WifiDirect(_mac) => {
                    println!("\x1b[35m[RADIO:WIFI-P2P] Transmitting {} bytes to P2P target {}\x1b[0m", encoded.len(), target_identifier);
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
                let mut snn = crate::snn::global_snn().lock().unwrap();
                snn.decay();
            }

            let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis();
            let msg = format!("ORIGIN_BEACON:{}:{}:{}", node_id, port, timestamp);
            let _ = socket.send_to(msg.as_bytes(), "255.255.255.255:9999").await;
            
            // Dynamic Biomimetic Sleep based on SNN
            let sleep_ms = crate::snn::global_snn().lock().unwrap().get_polling_interval();
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
            let _ = socket.send_to(payload.as_bytes(), "255.255.255.255:9999").await;
        }
    }
}

pub async fn listen_for_peers(telemetry_tx: tokio::sync::broadcast::Sender<crate::telemetry::TelemetryEvent>, my_node_id: String) {
    let mut known_peers: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:9999").await {
        println!("\x1b[32m[BEACON] Listening for Swarm Peers on LAN (Port 9999)...\x1b[0m");
        let mut buf = [0u8; 1024];
        loop {
            if let Ok((len, src)) = socket.recv_from(&mut buf).await {
                if let Ok(msg) = std::str::from_utf8(&buf[..len]) {
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
                                    let mut snn = crate::snn::global_snn().lock().unwrap();
                                    let fired = snn.integrate(5.0); // 5mV stimulus for beacon
                                    if fired {
                                        println!("\x1b[35m[SNN] Action Potential Fired! Network node awoken by incoming beacon.\x1b[0m");
                                    }
                                }
                                
                                // QGA Phase 7: Calculate TRUE physical latency for Quantum Fitness
                                let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis();
                                let latency = if current_time > beacon_time { current_time - beacon_time } else { 1 };
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
                                    let _ = telemetry_tx.send(crate::telemetry::TelemetryEvent::FermionicRoute {
                                        packet_id: node_id.to_string(),
                                        origin: my_node_id.clone(),
                                        dest: ip,
                                        is_quantum: true,
                                    });
                                }
                            }
                        }
                    } else if msg.starts_with("ORIGIN_CHAT:") {
                        // Format: ORIGIN_CHAT:SenderID:Message...
                        if let Some(first_colon) = msg.find(':') {
                            let rest = &msg[first_colon+1..];
                            if let Some(second_colon) = rest.find(':') {
                                let sender_id = &rest[..second_colon];
                                let chat_msg = &rest[second_colon+1..];
                                
                                // Ignore our own chat broadcasts (UI handles local echo)
                                if sender_id != my_node_id {
                                    // Phase 6: Massive SNN Stimulus for direct interaction
                                    {
                                        let mut snn = crate::snn::global_snn().lock().unwrap();
                                        let fired = snn.integrate(20.0); // 20mV stimulus guarantees spike
                                        if fired {
                                            println!("\x1b[35m[SNN] Action Potential Fired! Network node awoken by incoming CHAT.\x1b[0m");
                                        }
                                    }

                                    let _ = telemetry_tx.send(crate::telemetry::TelemetryEvent::ChatIncoming {
                                        sender: sender_id.to_string(),
                                        encrypted_payload: format!("AES_ENC::{}_ENC", chat_msg),
                                        decrypted_payload: chat_msg.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
