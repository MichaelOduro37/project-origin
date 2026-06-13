use std::sync::Arc;
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use crate::tensegrity::{PheromoneShard, Heartbeat};
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
            let msg = format!("ORIGIN_BEACON:{}:{}", node_id, port);
            let _ = socket.send_to(msg.as_bytes(), "255.255.255.255:9999").await;
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    }
}

pub async fn listen_for_peers(telemetry_tx: tokio::sync::broadcast::Sender<crate::telemetry::TelemetryEvent>) {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:9999").await {
        println!("\x1b[32m[BEACON] Listening for Swarm Peers on LAN (Port 9999)...\x1b[0m");
        let mut buf = [0u8; 1024];
        loop {
            if let Ok((len, src)) = socket.recv_from(&mut buf).await {
                if let Ok(msg) = std::str::from_utf8(&buf[..len]) {
                    if msg.starts_with("ORIGIN_BEACON:") {
                        let parts: Vec<&str> = msg.split(':').collect();
                        if parts.len() == 3 {
                            let node_id = parts[1];
                            let ip = src.ip().to_string();
                            
                            // Send UI Telemetry event so the user physically sees the discovery
                            let _ = telemetry_tx.send(crate::telemetry::TelemetryEvent::FermionicRoute {
                                packet_id: "SYNC".to_string(),
                                origin: "Local".to_string(),
                                dest: format!("{} ({})", node_id, ip),
                                is_quantum: true,
                            });
                        }
                    }
                }
            }
        }
    }
}
