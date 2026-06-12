use origin_core::cipher::OriginAI;
use origin_core::tensegrity::{TensegritySwarmNode, PheromoneShard, HardwareEnvironment, Heartbeat};
use origin_core::network::{start_node_listener, broadcast_packet, NetworkPacket};
use origin_core::immune::AisImmuneSystem;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

fn generate_msg_id() -> String {
    format!("msg-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())
}

async fn send_message_sim_async(
    sender_name: &str,
    msg: &str,
    group_id: Option<String>,
    ai_cipher: &OriginAI,
    source_node: Arc<Mutex<TensegritySwarmNode>>,
    target_ports: &[u16],
) {
    println!("\n[{}] Preparing to send message: '{}'", sender_name, msg);
    
    // 1. Encrypt and add Steganographic Camouflage
    let (encrypted_bytes, zkp) = ai_cipher.encrypt_pheromone(msg.as_bytes());
    
    // 2. Atomize into Pheromone Shards (Erasure Coding simulation)
    let chunks: Vec<&[u8]> = encrypted_bytes.chunks(encrypted_bytes.len() / 3 + 1).collect();
    let msg_id = generate_msg_id();
    
    let mut shards = Vec::new();
    for (i, chunk) in chunks.iter().enumerate() {
        let shard = PheromoneShard {
            message_id: msg_id.clone(),
            group_id: group_id.clone(),
            shard_index: i,
            total_shards: chunks.len(),
            encrypted_payload: chunk.to_vec(),
            zero_knowledge_proof: zkp,
        };
        shards.push(shard);
    }
    
    println!("[COMM] Message camouflaged as ambient noise and atomized into {} Shards.", shards.len());

    // 3. Inject into the multi-path swarm via UDP
    println!("[COMM] Injecting shards across multi-path subversion layer via UDP...");
    for shard in shards.iter() {
        broadcast_packet(NetworkPacket::Shard(shard.clone()), target_ports).await;
        // Small delay to simulate network latency
        sleep(Duration::from_millis(10)).await;
    }
    
    // Sender keeps one locally to seed the CRDT
    if let Some(last_shard) = shards.last() {
        source_node.lock().unwrap().pheromones.insert(last_shard.clone());
    }
}

#[tokio::main]
async fn main() {
    println!("===========================================================");
    println!("=== ORIGIN-CORE: PERFECTED MATRIX BOOT SEQUENCE (V3)    ===");
    println!("=== TENSEGRITY RESOLUTION & AI IMMUNE SYSTEM ACTIVE     ===");
    println!("===========================================================\n");

    // 1. Initialize the Origin AI & Immune System
    let ai_cipher = OriginAI::new();
    ai_cipher.awaken_autonomous_ai();
    
    // Define deterministic Self space (Simulated valid network packets)
    let dummy_hb = NetworkPacket::Pulse(Heartbeat { node_id: "self_space".to_string(), timestamp: 0 });
    let self_space_bytes = bincode::serialize(&dummy_hb).unwrap();
    
    // r_chunk size of 4 bytes allows strong matching
    let mut immune_system = AisImmuneSystem::new(vec![self_space_bytes], 4);
    immune_system.train_detectors(20, 16); // Train 20 polynomial-time detectors
    let shared_immune_system = Arc::new(Mutex::new(immune_system));
    
    sleep(Duration::from_millis(100)).await;

    // 2. Initialize Swarm Nodes
    println!("\n[COMM] Initializing Tensegrity Swarm Nodes with Quorum Sensing...");
    
    let node_alice = Arc::new(Mutex::new(TensegritySwarmNode::new("Alice_Phone", HardwareEnvironment {
        battery_level: 85, is_plugged_in: false, thermal_load: 40, cpu_cores: 8
    })));
    
    let node_relay1 = Arc::new(Mutex::new(TensegritySwarmNode::new("Stranger_Laptop", HardwareEnvironment {
        battery_level: 80, is_plugged_in: false, thermal_load: 50, cpu_cores: 4
    })));
    
    let node_relay2 = Arc::new(Mutex::new(TensegritySwarmNode::new("Street_Camera (IoT_Relay)", HardwareEnvironment {
        battery_level: 100, is_plugged_in: true, thermal_load: 30, cpu_cores: 2
    })));
    
    let node_bob = Arc::new(Mutex::new(TensegritySwarmNode::new("Bob_Tablet", HardwareEnvironment {
        battery_level: 90, is_plugged_in: false, thermal_load: 35, cpu_cores: 8
    })));

    // Assign ports
    let p_alice = 8001;
    let p_relay1 = 8002;
    let p_relay2 = 8003;
    let p_bob = 8004;

    // Spawn listeners with AI Immune System attached
    tokio::spawn(start_node_listener(p_alice, Arc::clone(&node_alice), Arc::clone(&shared_immune_system)));
    tokio::spawn(start_node_listener(p_relay1, Arc::clone(&node_relay1), Arc::clone(&shared_immune_system)));
    tokio::spawn(start_node_listener(p_relay2, Arc::clone(&node_relay2), Arc::clone(&shared_immune_system)));
    tokio::spawn(start_node_listener(p_bob, Arc::clone(&node_bob), Arc::clone(&shared_immune_system)));

    sleep(Duration::from_millis(500)).await;

    // Simulate initial healthy heartbeats across the mesh
    println!("\n[SWARM] Generating ambient baseline heartbeats (Tension = 0.0)...");
    let hb1 = node_relay1.lock().unwrap().generate_heartbeat();
    broadcast_packet(NetworkPacket::Pulse(hb1), &[p_relay2]).await;
    
    let hb2 = node_relay2.lock().unwrap().generate_heartbeat();
    broadcast_packet(NetworkPacket::Pulse(hb2), &[p_relay1]).await;
    
    sleep(Duration::from_millis(100)).await;

    // 3. SCENARIO A: AI IMMUNE SYSTEM QUARANTINE
    println!("\n--- SCENARIO: MALICIOUS INJECTION & AI QUARANTINE ---");
    println!("[ATTACKER] Injecting anomalous raw payload into Stranger_Laptop (Relay 1)...");
    let malicious_socket = tokio::net::UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let malicious_payload = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xBA, 0xAD, 0xF0, 0x0D]; // Matches nothing in Self space
    malicious_socket.send_to(&malicious_payload, format!("127.0.0.1:{}", p_relay1)).await.unwrap();
    sleep(Duration::from_millis(300)).await;

    // 4. SCENARIO B: UDP 1-on-1 Message with Trophic Cascade
    println!("\n--- SCENARIO: TROPHIC CASCADE & HOLOGRAPHIC REPLICATION ---");
    let target_ports = vec![p_relay1, p_relay2];
    
    send_message_sim_async(
        "ALICE", 
        "The universe is asymmetric.", 
        None, 
        &ai_cipher, 
        Arc::clone(&node_alice), 
        &target_ports
    ).await;

    sleep(Duration::from_millis(300)).await;

    println!("\n[SYSTEM] Simulating Catastrophic Node Failure: Stranger_Laptop battery dies...");
    // We stop relay1 from sending any more heartbeats.
    
    println!("[SYSTEM] 4 Seconds pass. Tension builds in the mesh...");
    sleep(Duration::from_secs(4)).await;

    // Relay 2 evaluates tension
    let mut _cascaded_shards = None;
    {
        let mut r2 = node_relay2.lock().unwrap();
        _cascaded_shards = r2.evaluate_tension_cascade();
    }

    if let Some(shards) = _cascaded_shards {
        println!("\n[SWARM] Bose-Einstein Condensation Triggered! Replicating {} memory fragments to surviving nodes...", shards.len());
        for shard in shards {
            broadcast_packet(NetworkPacket::Shard(shard), &[p_bob]).await;
        }
    }
    
    sleep(Duration::from_millis(500)).await;

    println!("[BOB] Device attempting to reassemble missing state...");
    let reassembled_bob = node_bob.lock().unwrap().reassemble_messages(&ai_cipher);
    for (id, group_id, content) in reassembled_bob {
        if group_id.is_none() {
            println!("\n\x1b[32m=== SUCCESSFUL SELF-HEALING TRANSMISSION ===\x1b[0m");
            println!("Message ID: {}", id);
            println!("Decrypted Content: {}", content);
            println!("============================================");
        }
    }

    sleep(Duration::from_millis(500)).await;
    println!("\n[SYSTEM] Origin Core V3 Execution Complete. The mesh is unbroken.");
}
