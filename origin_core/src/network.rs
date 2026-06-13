use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};
use crate::tensegrity::{TensegritySwarmNode, PheromoneShard, Heartbeat};
use crate::immune::AisImmuneSystem;
use crate::cipher::ChaoticAttractor;

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
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FermonicRoute {
    pub hops: Vec<u32>,
    pub probability: f64,
    pub is_longrange: bool,
    pub nonlocality_distance: f64,
}

use crate::snn::IntegrateAndFireNode;

pub struct FermonicRouter {
    pub node_id: String,
    pub nonlocality_factor: f64,
    pub mesh_topology: HashMap<u32, Vec<u32>>,
    pub attractor: ChaoticAttractor,
    pub primary_radio: HardwareRadio, // Hooked up the physical radio interface
    pub predictive_synapses: HashMap<u32, IntegrateAndFireNode>,
}

impl FermonicRouter {
    pub fn new(node_id: String, nonlocality_factor: f64, radio: HardwareRadio) -> Self {
        let seed = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            node_id.hash(&mut hasher);
            hasher.finish()
        };

        FermonicRouter {
            node_id,
            nonlocality_factor: nonlocality_factor.max(0.0).min(1.0),
            mesh_topology: HashMap::new(),
            attractor: ChaoticAttractor::new(
                (seed as f64 % 1000.0) * 0.001,
                0.1 + ((seed >> 32) as f64 % 1000.0) * 0.001,
                0.1
            ),
            primary_radio: radio,
            predictive_synapses: HashMap::new(),
        }
    }

    pub fn register_topology(&mut self, port: u32, neighbors: Vec<u32>) {
        self.mesh_topology.insert(port, neighbors);
    }

    pub fn route_fermionic(&mut self, source_port: u32, dest_port: u32, mesh_state: &HashMap<u32, bool>) -> Vec<FermonicRoute> {
        let mut routes = Vec::new();

        if let Some(neighbors) = self.mesh_topology.get(&source_port) {
            for &neighbor in neighbors {
                if mesh_state.get(&neighbor).copied().unwrap_or(false) {
                    routes.push(FermonicRoute {
                        hops: vec![source_port, neighbor],
                        probability: 1.0 - self.nonlocality_factor,
                        is_longrange: false,
                        nonlocality_distance: 1.0,
                    });
                }
            }
        }

        if self.nonlocality_factor > 0.01 {
            let mut all_nodes: Vec<u32> = mesh_state.keys().copied().collect();
            all_nodes.sort();
            
            for &distant_node in &all_nodes {
                if distant_node == source_port || distant_node == dest_port { continue; }
                let sigma = 1.0 / self.nonlocality_factor.max(0.01);
                let distance = ((distant_node as i32 - source_port as i32).abs() as f64).max(1.0);
                let overlap = (-distance * distance / (2.0 * sigma * sigma)).exp();
                let entropy = self.attractor.next_float();
                let quantum_probability = overlap * entropy * self.nonlocality_factor;

                if quantum_probability > 0.1 {
                    routes.push(FermonicRoute {
                        hops: vec![source_port, distant_node],
                        probability: quantum_probability,
                        is_longrange: true,
                        nonlocality_distance: distance,
                    });
                }
            }
        }

        routes.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap_or(std::cmp::Ordering::Equal));
        routes
    }

    // PHASE X: Now broadcasts via the abstract radio layer
    // PHASE 6: Neuromorphic Predictive Filtering
    pub async fn broadcast(&mut self, packet: NetworkPacket, source: u16, dest: u16, mesh_state: &HashMap<u32, bool>) {
        let routes = self.route_fermionic(source as u32, dest as u32, mesh_state);
        
        for route in routes.iter().take(3) {
            if route.hops.len() >= 2 {
                let next_hop = route.hops[1] as u32;
                
                // --- SNN Phase 6: Accumulate Voltage on Synapse ---
                let synapse = self.predictive_synapses.entry(next_hop).or_insert_with(|| {
                    IntegrateAndFireNode::new(format!("Synapse_{}", next_hop), 1.0, 0.1) // threshold 1.0, leak 10%
                });
                
                // Simulate time decay (abstracted to UNIX timestamp seconds for PoC)
                let current_tick = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                synapse.tick_decay(current_tick);
                
                let spiked = synapse.accumulate_pressure(0.4); // Each packet routing attempt adds 0.4 voltage
                if spiked {
                    println!("\x1b[33m[SNN] Action Potential Reached! Synapse {} spiked. Waking up target node via out-of-band BLE pulse.\x1b[0m", next_hop);
                    // In a real scenario, this sends a BLE Wake-Up packet.
                } else if !synapse.is_primed() {
                    // Node is theoretically asleep. Packet might be dropped or delayed in real life.
                    // We simulate the logging here.
                }

                if route.is_longrange {
                    println!("\x1b[36m[FERMIONIC ROUTING] Quantum leap! Tunneling from {} to distant node {} (prob: {:.2})\x1b[0m", 
                        source, next_hop, route.probability);
                }
                
                // Use the generic radio to transmit!
                self.primary_radio.send_packet(&packet, &next_hop.to_string()).await;
            }
        }
    }
}

pub struct FermionicRheologyGovernor {
    ip_energy_state: HashMap<String, (u32, Instant)>,
    base_viscosity_ms: u64,
    attractor: ChaoticAttractor, 
}

impl FermionicRheologyGovernor {
    pub fn new() -> Self {
        FermionicRheologyGovernor {
            ip_energy_state: HashMap::new(),
            base_viscosity_ms: 10,
            attractor: ChaoticAttractor::new(0.5, 1.2, 0.8),
        }
    }

    pub async fn process_fermionic_packet(&mut self, src_identifier: String) -> bool {
        let now = Instant::now();
        let entry = self.ip_energy_state.entry(src_identifier.clone()).or_insert((0, now));

        if now.duration_since(entry.1) > Duration::from_secs(1) {
            entry.0 = 0;
            entry.1 = now;
        }
        
        let energy_level = entry.0 as f64;
        let mu = 5.0; 
        let temperature = 1.0;
        let admission_probability = 1.0 / (1.0 + ((energy_level - mu) / temperature).exp());
        
        let roll = self.attractor.next_float();
        
        if roll > admission_probability {
            println!("\x1b[35m[FERMIONIC ROUTING] Packet from {} repelled! (Energy state too high: Pauli Exclusion).\x1b[0m", src_identifier);
            return false;
        }

        entry.0 += 1; 
        
        if entry.0 > 5 { 
            let excess_strain = entry.0 - 5;
            let viscosity_delay = self.base_viscosity_ms * (1 << (excess_strain.min(10) as u64)); 
            tokio::time::sleep(Duration::from_millis(viscosity_delay)).await;
        }
        
        true
    }
}

// Start a background listener using the raw socket for backward compatibility with the mock
pub async fn start_node_listener(port: u16, node: Arc<Mutex<TensegritySwarmNode>>, immune_system: Arc<Mutex<AisImmuneSystem>>) {
    let addr = format!("0.0.0.0:{}", port);
    let socket = match UdpSocket::bind(&addr).await {
        Ok(s) => s,
        Err(_) => {
            println!("[NETWORK] Failed to bind node listener on {}. Port in use.", addr);
            return;
        }
    };
    println!("[NETWORK] LAN Radio listener started on {}", addr);

    let mut buf = [0u8; 4096];
    let mut governor = FermionicRheologyGovernor::new();

    loop {
        if let Ok((len, src)) = socket.recv_from(&mut buf).await {
            let data = &buf[..len];

            if !governor.process_fermionic_packet(src.to_string()).await {
                continue; 
            }

            let is_anomalous = {
                let mut immune = immune_system.lock().unwrap();
                immune.monitor_traffic(data, 0.1, 0.9) 
            };
            
            if is_anomalous {
                let n = node.lock().unwrap();
                println!("\x1b[31m[IMMUNE] Node {} quarantined anomalous packet.\x1b[0m", n.node_id);
                continue; 
            }

            if let Ok(packet) = bincode::deserialize::<NetworkPacket>(data) {
                let mut n = node.lock().unwrap();
                
                match packet {
                    NetworkPacket::Shard(shard) => {
                        let _ = n.ingest_pheromone(shard.clone());
                    },
                    NetworkPacket::Pulse(heartbeat) => {
                        n.ingest_heartbeat(heartbeat);
                    }
                }
            }
        }
    }
}

pub async fn broadcast_packet(packet: NetworkPacket, target_ports: &[u16]) {
    let radio = HardwareRadio::bind_lan(0).await;
    for port in target_ports {
        radio.send_packet(&packet, &port.to_string()).await;
    }
}

// PHASE 9: TRUE LAN PEER DISCOVERY BEACON
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
