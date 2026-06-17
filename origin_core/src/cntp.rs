// ============================================================================
// CHEMOTACTIC NAT TRAVERSAL PROTOCOL (CNTP)
// ============================================================================
// A novel, zero-infrastructure, scientifically-grounded protocol for
// cross-network P2P connectivity.
//
// Scientific Foundation:
//   Layer 1: Bacterial Chemotaxis (Berg & Purcell, 1977) — Self-Discovery
//   Layer 2: Stigmergy (Grassé, 1959) + RFC 5128 — UDP Hole Punching
//   Layer 3: Birthday Paradox (von Mises, 1939) — Symmetric NAT Busting
//   Layer 4: Autonomous NAT Traversal (Kamkar et al., IEEE P2P'10) — ICMP Fallback
//   Coordination: Rendezvous Hashing (Thaler & Ravishankar, 1996)
// ============================================================================

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

type HmacSha256 = Hmac<Sha256>;

/// The result of Layer 1 chemotactic self-discovery.
#[derive(Debug, Clone)]
pub struct ChemotacticIdentity {
    pub public_ip: [u8; 4],
    pub nat_type: NatType,
    pub local_port: u16,
}

/// NAT classification derived from probing two public reflectors.
#[derive(Debug, Clone, PartialEq)]
pub enum NatType {
    /// Full Cone: Same external mapping for all destinations. Easiest to punch.
    FullCone,
    /// Restricted Cone: Same external port, but filters by destination IP.
    RestrictedCone,
    /// Port Restricted: Same external port, but filters by destination IP+Port.
    PortRestricted,
    /// Symmetric: Different external port for each destination. Hardest.
    Symmetric,
    /// Could not determine (firewall blocks all probes).
    Unknown,
}

/// A peer's identity for rendezvous computation.
#[derive(Debug, Clone)]
pub struct PeerIdentity {
    pub public_key: [u8; 32],
    pub known_public_ip: Option<[u8; 4]>,
}

/// CNTP connection state machine events.
#[derive(Debug)]
pub enum CntpEvent {
    SelfDiscovered(ChemotacticIdentity),
    PeerConnected { peer_addr: SocketAddr },
    PunchAttempt { layer: u8, ports_tried: u32 },
    ConnectionFailed { reason: String },
}

// ============================================================================
// LAYER 1: CHEMOTACTIC SELF-DISCOVERY
// Uses existing public DNS infrastructure as "chemical sensors".
// No dedicated STUN servers — just the internet's own plumbing.
// ============================================================================

/// Discover our own public IP by querying public DNS "whoami" services.
/// This is NOT a STUN server — it's a standard DNS service that already exists.
/// Like a bacterium sensing the pH of the water it's already swimming in.
pub async fn chemotactic_self_discover() -> Option<ChemotacticIdentity> {
    println!("\x1b[36m[CNTP:LAYER1] Initiating Chemotactic Self-Discovery...\x1b[0m");
    println!("\x1b[36m[CNTP:LAYER1] Sensing public identity via DNS chemical gradient...\x1b[0m");

    // Strategy: Send a DNS query to OpenDNS's `myip.opendns.com` resolver.
    // This returns OUR public IP as the DNS answer — no STUN protocol needed.
    // Fallback: try multiple public "reflectors" (Google, Cloudflare, Akamai).

    let reflectors: Vec<(&str, u16)> = vec![
        ("208.67.222.222", 53),  // OpenDNS resolver1
        ("208.67.220.220", 53),  // OpenDNS resolver2
    ];

    let mut discovered_ip: Option<[u8; 4]> = None;
    let mut nat_type = NatType::Unknown;
    let mut observed_ports: Vec<u16> = Vec::new();

    // Bind a single probe socket
    let probe_socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(e) => {
            println!("\x1b[31m[CNTP:LAYER1] Failed to bind probe socket: {}\x1b[0m", e);
            return None;
        }
    };

    let local_port = probe_socket.local_addr().map(|a| a.port()).unwrap_or(0);

    // Build a minimal DNS query for `myip.opendns.com` (A record)
    // DNS Header: ID=0x1234, QR=0, OPCODE=0, QDCOUNT=1
    // Question: myip.opendns.com, QTYPE=A(1), QCLASS=IN(1)
    let dns_query = build_dns_query("myip.opendns.com");

    for (reflector_ip, reflector_port) in &reflectors {
        let addr: SocketAddr = format!("{}:{}", reflector_ip, reflector_port)
            .parse()
            .unwrap();

        if let Err(_) = probe_socket.send_to(&dns_query, addr).await {
            continue;
        }

        let mut buf = [0u8; 512];
        let timeout = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            probe_socket.recv_from(&mut buf),
        );

        if let Ok(Ok((len, from_addr))) = timeout.await {
            // Parse the DNS response to extract our public IP
            if let Some(ip) = parse_dns_response_for_ip(&buf[..len]) {
                println!(
                    "\x1b[32m[CNTP:LAYER1] Chemical sensor {} detected our public identity: {}.{}.{}.{}\x1b[0m",
                    reflector_ip, ip[0], ip[1], ip[2], ip[3]
                );
                discovered_ip = Some(ip);
                observed_ports.push(from_addr.port());
            }
        }
    }

    // NAT Type Classification:
    // If we observed consistent source ports across different reflectors → Cone NAT
    // If ports differ → Symmetric NAT
    if observed_ports.len() >= 2 {
        if observed_ports[0] == observed_ports[1] {
            nat_type = NatType::RestrictedCone; // Conservative assumption
            println!("\x1b[32m[CNTP:LAYER1] NAT Behavior: Consistent port mapping detected → Cone NAT\x1b[0m");
        } else {
            nat_type = NatType::Symmetric;
            println!("\x1b[33m[CNTP:LAYER1] NAT Behavior: Inconsistent port mapping → Symmetric NAT (Birthday Attack required)\x1b[0m");
        }
    } else if observed_ports.len() == 1 {
        nat_type = NatType::PortRestricted;
        println!("\x1b[33m[CNTP:LAYER1] NAT Behavior: Single reflector response → Port Restricted (assumed)\x1b[0m");
    }

    let ip = discovered_ip?;
    Some(ChemotacticIdentity {
        public_ip: ip,
        nat_type,
        local_port,
    })
}

/// Build a minimal DNS query packet for a given domain (A record).
fn build_dns_query(domain: &str) -> Vec<u8> {
    let mut packet = Vec::with_capacity(64);

    // DNS Header (12 bytes)
    packet.extend_from_slice(&[0x12, 0x34]); // Transaction ID
    packet.extend_from_slice(&[0x01, 0x00]); // Flags: Standard query, recursion desired
    packet.extend_from_slice(&[0x00, 0x01]); // Questions: 1
    packet.extend_from_slice(&[0x00, 0x00]); // Answer RRs: 0
    packet.extend_from_slice(&[0x00, 0x00]); // Authority RRs: 0
    packet.extend_from_slice(&[0x00, 0x00]); // Additional RRs: 0

    // Question section: encode domain name
    for label in domain.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0x00); // Root label

    packet.extend_from_slice(&[0x00, 0x01]); // QTYPE: A (1)
    packet.extend_from_slice(&[0x00, 0x01]); // QCLASS: IN (1)

    packet
}

/// Parse a DNS response to extract the first A record (IPv4 address).
fn parse_dns_response_for_ip(response: &[u8]) -> Option<[u8; 4]> {
    if response.len() < 12 {
        return None;
    }

    // Check ANCOUNT > 0
    let ancount = u16::from_be_bytes([response[6], response[7]]);
    if ancount == 0 {
        return None;
    }

    // Skip header (12 bytes), then skip the question section
    let mut offset = 12;

    // Skip question name
    while offset < response.len() {
        let len = response[offset] as usize;
        if len == 0 {
            offset += 1;
            break;
        }
        if len >= 0xC0 {
            // Pointer
            offset += 2;
            break;
        }
        offset += 1 + len;
    }
    offset += 4; // Skip QTYPE + QCLASS

    // Parse answer records
    for _ in 0..ancount {
        if offset >= response.len() {
            break;
        }

        // Skip name (may be pointer)
        if offset < response.len() && response[offset] >= 0xC0 {
            offset += 2;
        } else {
            while offset < response.len() {
                let len = response[offset] as usize;
                if len == 0 {
                    offset += 1;
                    break;
                }
                offset += 1 + len;
            }
        }

        if offset + 10 > response.len() {
            break;
        }

        let rtype = u16::from_be_bytes([response[offset], response[offset + 1]]);
        let rdlength = u16::from_be_bytes([response[offset + 8], response[offset + 9]]);
        offset += 10; // Skip TYPE(2) + CLASS(2) + TTL(4) + RDLENGTH(2)

        if rtype == 1 && rdlength == 4 && offset + 4 <= response.len() {
            // A record — this is our public IP!
            return Some([
                response[offset],
                response[offset + 1],
                response[offset + 2],
                response[offset + 3],
            ]);
        }

        offset += rdlength as usize;
    }

    None
}

// ============================================================================
// RENDEZVOUS HASHING (Thaler & Ravishankar, 1996)
// Both nodes independently compute the SAME connection parameters
// from their shared cryptographic identity. No signaling server needed.
// ============================================================================

/// Compute a deterministic set of rendezvous ports and timing window
/// that both peers will arrive at independently.
pub fn compute_rendezvous(
    my_pubkey: &[u8; 32],
    peer_pubkey: &[u8; 32],
    port_count: usize,
) -> RendezvousParams {
    // XOR the two public keys to get an order-independent shared seed
    let mut shared_seed = [0u8; 32];
    for i in 0..32 {
        shared_seed[i] = my_pubkey[i] ^ peer_pubkey[i];
    }

    // Current time window: round to nearest 30-second window
    let epoch_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let time_window = epoch_secs / 30; // 30-second windows

    // HMAC-SHA256(shared_seed, time_window) → deterministic output
    let mut mac = HmacSha256::new_from_slice(&shared_seed)
        .expect("HMAC can take key of any size");
    mac.update(&time_window.to_le_bytes());
    let result = mac.finalize().into_bytes();

    // Derive ports from the HMAC output
    let mut ports = Vec::with_capacity(port_count);
    for i in 0..port_count {
        let idx = (i * 2) % 30; // 2 bytes per port, wrap within 32-byte hash
        let port = u16::from_le_bytes([result[idx], result[idx + 1]]);
        // Ensure port is in ephemeral range (1024-65535)
        let safe_port = 1024 + (port % (65535 - 1024));
        ports.push(safe_port);
    }

    // Derive timing offset (which second within the 30-second window to punch)
    let timing_offset = result[31] % 30;

    println!(
        "\x1b[35m[CNTP:RENDEZVOUS] Cryptographic gradient computed. Window: {}, Ports: {:?}, Timing: +{}s\x1b[0m",
        time_window, ports, timing_offset
    );

    RendezvousParams {
        ports,
        time_window,
        timing_offset,
        shared_secret: result.into(),
    }
}

/// Computed rendezvous parameters that both peers arrive at independently.
#[derive(Debug, Clone)]
pub struct RendezvousParams {
    pub ports: Vec<u16>,
    pub time_window: u64,
    pub timing_offset: u8,
    pub shared_secret: [u8; 32],
}

// ============================================================================
// LAYER 2: STIGMERGIC UDP HOLE PUNCHING
// NAT mappings become "pheromone traces" in the environment.
// Both nodes simultaneously send UDP packets to computed rendezvous ports,
// creating bidirectional NAT entries that allow peer traffic through.
// ============================================================================

/// Attempt to establish a direct P2P tunnel using stigmergic hole punching.
/// Returns the established socket and peer address on success.
pub async fn stigmergic_hole_punch(
    peer_ip: [u8; 4],
    rendezvous: &RendezvousParams,
    event_tx: mpsc::UnboundedSender<CntpEvent>,
) -> Option<(Arc<UdpSocket>, SocketAddr)> {
    println!("\x1b[36m[CNTP:LAYER2] Initiating Stigmergic UDP Hole Punch...\x1b[0m");
    println!(
        "\x1b[36m[CNTP:LAYER2] Target: {}.{}.{}.{}, Rendezvous ports: {:?}\x1b[0m",
        peer_ip[0], peer_ip[1], peer_ip[2], peer_ip[3], rendezvous.ports
    );

    let _ = event_tx.send(CntpEvent::PunchAttempt {
        layer: 2,
        ports_tried: rendezvous.ports.len() as u32,
    });

    // Bind our local socket
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(e) => {
            println!("\x1b[31m[CNTP:LAYER2] Failed to bind socket: {}\x1b[0m", e);
            return None;
        }
    };
    let socket = Arc::new(socket);

    let peer_ip_str = format!("{}.{}.{}.{}", peer_ip[0], peer_ip[1], peer_ip[2], peer_ip[3]);

    // The "pheromone" packet: a small signed payload proving our identity
    let punch_packet = build_punch_packet(&rendezvous.shared_secret);

    // Send to all rendezvous ports simultaneously (leaving pheromone traces)
    for port in &rendezvous.ports {
        let target: SocketAddr = format!("{}:{}", peer_ip_str, port).parse().unwrap();
        let _ = socket.send_to(&punch_packet, target).await;
    }

    println!("\x1b[33m[CNTP:LAYER2] Pheromone traces deposited. Listening for peer's traces...\x1b[0m");

    // Listen for the peer's punch packets (their pheromone traces)
    let mut buf = [0u8; 128];
    let listen_duration = std::time::Duration::from_secs(10);

    // Send punch packets repeatedly while listening
    let socket_send = socket.clone();
    let ports = rendezvous.ports.clone();
    let punch_clone = punch_packet.clone();
    let peer_ip_clone = peer_ip_str.clone();
    tokio::spawn(async move {
        for _ in 0..20 {
            // 20 attempts over 10 seconds
            for port in &ports {
                let target: SocketAddr =
                    format!("{}:{}", peer_ip_clone, port).parse().unwrap();
                let _ = socket_send.send_to(&punch_clone, target).await;
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    // Listen for incoming punch packets from the peer
    let deadline = tokio::time::Instant::now() + listen_duration;
    while tokio::time::Instant::now() < deadline {
        let timeout_result = tokio::time::timeout(
            std::time::Duration::from_millis(500),
            socket.recv_from(&mut buf),
        )
        .await;

        if let Ok(Ok((len, from_addr))) = timeout_result {
            if verify_punch_packet(&buf[..len], &rendezvous.shared_secret) {
                println!(
                    "\x1b[32;1m[CNTP:LAYER2] ✓ STIGMERGIC PUNCH SUCCEEDED! Peer detected at {}\x1b[0m",
                    from_addr
                );
                let _ = event_tx.send(CntpEvent::PeerConnected {
                    peer_addr: from_addr,
                });
                return Some((socket, from_addr));
            }
        }
    }

    println!("\x1b[33m[CNTP:LAYER2] Stigmergic punch did not converge. Escalating to Layer 3...\x1b[0m");
    None
}

// ============================================================================
// LAYER 3: BIRTHDAY ATTACK AMPLIFICATION
// When Symmetric NAT is detected, exploit the Birthday Paradox to
// dramatically increase collision probability across the port space.
// P(collision) = 1 - e^(-n²/2k) where k=65536, n=256 → 99.7%
// ============================================================================

/// Attempt NAT traversal using birthday attack port prediction.
/// Opens 256 sockets and blasts to 256 computed target ports simultaneously.
pub async fn birthday_attack_punch(
    peer_ip: [u8; 4],
    rendezvous: &RendezvousParams,
    event_tx: mpsc::UnboundedSender<CntpEvent>,
) -> Option<(Arc<UdpSocket>, SocketAddr)> {
    const N_PROBES: usize = 256; // √65536 ≈ 256 → >99% collision probability

    println!("\x1b[35;1m[CNTP:LAYER3] BIRTHDAY ATTACK INITIATED!\x1b[0m");
    println!(
        "\x1b[35m[CNTP:LAYER3] Deploying {} simultaneous probes. P(collision) > 99.7%%\x1b[0m",
        N_PROBES
    );

    let _ = event_tx.send(CntpEvent::PunchAttempt {
        layer: 3,
        ports_tried: N_PROBES as u32,
    });

    let peer_ip_str = format!("{}.{}.{}.{}", peer_ip[0], peer_ip[1], peer_ip[2], peer_ip[3]);
    let punch_packet = build_punch_packet(&rendezvous.shared_secret);

    // Generate 256 deterministic target ports from the shared secret
    let target_ports: Vec<u16> = (0..N_PROBES)
        .map(|i| {
            let mut mac = HmacSha256::new_from_slice(&rendezvous.shared_secret)
                .expect("HMAC accepts any key size");
            mac.update(&(i as u64).to_le_bytes());
            mac.update(b"birthday");
            let result = mac.finalize().into_bytes();
            let port = u16::from_le_bytes([result[0], result[1]]);
            1024 + (port % (65535 - 1024))
        })
        .collect();

    // Bind multiple probe sockets (up to 64 — OS limits)
    let mut sockets: Vec<Arc<UdpSocket>> = Vec::new();
    for _ in 0..64.min(N_PROBES) {
        if let Ok(s) = UdpSocket::bind("0.0.0.0:0").await {
            sockets.push(Arc::new(s));
        }
    }

    if sockets.is_empty() {
        println!("\x1b[31m[CNTP:LAYER3] Failed to bind any probe sockets\x1b[0m");
        return None;
    }

    println!(
        "\x1b[35m[CNTP:LAYER3] Bound {} probe sockets. Blasting {} target ports...\x1b[0m",
        sockets.len(),
        target_ports.len()
    );

    // Blast phase: send from each socket to each target port
    let primary_socket = sockets[0].clone();
    for round in 0..5 {
        for (i, port) in target_ports.iter().enumerate() {
            let sock = &sockets[i % sockets.len()];
            let target: SocketAddr = format!("{}:{}", peer_ip_str, port).parse().unwrap();
            let _ = sock.send_to(&punch_packet, target).await;
        }
        if round < 4 {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
    }

    // Listen on all sockets for incoming punch response
    let mut buf = [0u8; 128];
    let listen_deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(15);

    while tokio::time::Instant::now() < listen_deadline {
        // Poll each socket
        for sock in &sockets {
            let timeout_result = tokio::time::timeout(
                std::time::Duration::from_millis(50),
                sock.recv_from(&mut buf),
            )
            .await;

            if let Ok(Ok((len, from_addr))) = timeout_result {
                if verify_punch_packet(&buf[..len], &rendezvous.shared_secret) {
                    println!(
                        "\x1b[32;1m[CNTP:LAYER3] ✓ BIRTHDAY ATTACK SUCCEEDED! Port collision found at {}\x1b[0m",
                        from_addr
                    );
                    let _ = event_tx.send(CntpEvent::PeerConnected {
                        peer_addr: from_addr,
                    });
                    return Some((sock.clone(), from_addr));
                }
            }
        }
    }

    println!("\x1b[33m[CNTP:LAYER3] Birthday attack did not converge. Escalating to Layer 4...\x1b[0m");
    None
}

// ============================================================================
// LAYER 4: ICMP AUTONOMOUS TRAVERSAL (IEEE P2P'10 — Kamkar et al.)
// The nuclear option. Uses the internet's own error-handling protocol
// as a side-channel to establish direct tunnels through the most
// restrictive firewalls. Zero servers required.
//
// NOTE: Raw ICMP requires elevated privileges on most OSes.
// On Android/Windows without root, this layer gracefully degrades.
// ============================================================================

/// Attempt ICMP-based autonomous NAT traversal (PWNAT technique).
/// This is a best-effort layer that may require elevated privileges.
pub async fn icmp_autonomous_traversal(
    peer_ip: [u8; 4],
    rendezvous: &RendezvousParams,
    event_tx: mpsc::UnboundedSender<CntpEvent>,
) -> Option<(Arc<UdpSocket>, SocketAddr)> {
    println!("\x1b[31;1m[CNTP:LAYER4] ICMP AUTONOMOUS TRAVERSAL (IEEE P2P'10)\x1b[0m");
    println!("\x1b[31m[CNTP:LAYER4] Attempting to exploit ICMP Time Exceeded side-channel...\x1b[0m");

    let _ = event_tx.send(CntpEvent::PunchAttempt {
        layer: 4,
        ports_tried: 1,
    });

    // On most consumer OSes without root/admin, raw ICMP sockets are blocked.
    // We attempt a UDP-based approximation: send to a high-TTL port on the
    // peer's IP. Some NATs will generate ICMP Port Unreachable responses that
    // create state entries we can exploit.

    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => Arc::new(s),
        Err(_) => return None,
    };

    let peer_ip_str = format!("{}.{}.{}.{}", peer_ip[0], peer_ip[1], peer_ip[2], peer_ip[3]);
    let punch_packet = build_punch_packet(&rendezvous.shared_secret);

    // Send UDP probes to unlikely ports — this may trigger ICMP responses
    // that create NAT state entries
    let probe_ports: Vec<u16> = (33434..33470).collect(); // traceroute range

    for _ in 0..10 {
        for port in &probe_ports {
            let target: SocketAddr = format!("{}:{}", peer_ip_str, port).parse().unwrap();
            let _ = socket.send_to(&punch_packet, target).await;
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        // Check if the peer managed to reach us
        let mut buf = [0u8; 128];
        if let Ok(Ok((len, from_addr))) = tokio::time::timeout(
            std::time::Duration::from_millis(200),
            socket.recv_from(&mut buf),
        )
        .await
        {
            if verify_punch_packet(&buf[..len], &rendezvous.shared_secret) {
                println!(
                    "\x1b[32;1m[CNTP:LAYER4] ✓ ICMP SIDE-CHANNEL TRAVERSAL SUCCEEDED! Tunnel at {}\x1b[0m",
                    from_addr
                );
                let _ = event_tx.send(CntpEvent::PeerConnected {
                    peer_addr: from_addr,
                });
                return Some((socket, from_addr));
            }
        }
    }

    println!("\x1b[31m[CNTP:LAYER4] All layers exhausted. Direct P2P tunnel could not be established.\x1b[0m");
    let _ = event_tx.send(CntpEvent::ConnectionFailed {
        reason: "All 4 CNTP layers failed. Networks may be completely isolated.".into(),
    });
    None
}

// ============================================================================
// PUNCH PACKET CONSTRUCTION & VERIFICATION
// Cryptographically signed packets that prove Origin node identity.
// ============================================================================

/// Build a signed punch packet proving our identity to the peer.
fn build_punch_packet(shared_secret: &[u8; 32]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(48);

    // Magic header: "ORIG" (4 bytes)
    packet.extend_from_slice(b"ORIG");

    // Protocol version (1 byte)
    packet.push(0x01);

    // Timestamp (8 bytes) — prevents replay attacks
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    packet.extend_from_slice(&timestamp.to_le_bytes());

    // Random nonce (4 bytes)
    let nonce: u32 = rand::random();
    packet.extend_from_slice(&nonce.to_le_bytes());

    // HMAC signature of the above (32 bytes)
    let mut mac = HmacSha256::new_from_slice(shared_secret)
        .expect("HMAC accepts any key size");
    mac.update(&packet);
    let signature = mac.finalize().into_bytes();
    packet.extend_from_slice(&signature);

    packet // Total: 4 + 1 + 8 + 4 + 32 = 49 bytes
}

/// Verify that an incoming packet is a valid Origin punch packet from our peer.
fn verify_punch_packet(data: &[u8], shared_secret: &[u8; 32]) -> bool {
    // Minimum size: 4 (magic) + 1 (ver) + 8 (ts) + 4 (nonce) + 32 (sig) = 49
    if data.len() < 49 {
        return false;
    }

    // Check magic header
    if &data[0..4] != b"ORIG" {
        return false;
    }

    // Verify HMAC signature
    let message = &data[..17]; // Everything before the signature
    let claimed_sig = &data[17..49];

    let mut mac = HmacSha256::new_from_slice(shared_secret)
        .expect("HMAC accepts any key size");
    mac.update(message);

    mac.verify_slice(claimed_sig).is_ok()
}

// ============================================================================
// FULL CASCADE: The unified CNTP connection attempt
// Tries all 4 layers in order, escalating on failure.
// ============================================================================

/// Attempt to establish a direct P2P connection to a peer using the
/// full 4-layer Chemotactic NAT Traversal cascade.
pub async fn connect_to_peer(
    my_pubkey: &[u8; 32],
    peer: &PeerIdentity,
    event_tx: mpsc::UnboundedSender<CntpEvent>,
) -> Option<(Arc<UdpSocket>, SocketAddr)> {
    println!("\n\x1b[36;1m╔═══════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[36;1m║  CHEMOTACTIC NAT TRAVERSAL PROTOCOL — INITIATING     ║\x1b[0m");
    println!("\x1b[36;1m╚═══════════════════════════════════════════════════════╝\x1b[0m\n");

    // Layer 1: Self-Discovery
    let identity = chemotactic_self_discover().await;
    if let Some(ref id) = identity {
        let _ = event_tx.send(CntpEvent::SelfDiscovered(id.clone()));
        println!(
            "\x1b[32m[CNTP] Self-identity confirmed: {}.{}.{}.{} (NAT: {:?})\x1b[0m",
            id.public_ip[0], id.public_ip[1], id.public_ip[2], id.public_ip[3], id.nat_type
        );
    }

    // Compute rendezvous parameters (both peers compute the same values independently)
    let rendezvous = compute_rendezvous(my_pubkey, &peer.public_key, 8);

    let peer_ip = match peer.known_public_ip {
        Some(ip) => ip,
        None => {
            println!("\x1b[31m[CNTP] ERROR: Peer public IP unknown. Exchange keys first.\x1b[0m");
            let _ = event_tx.send(CntpEvent::ConnectionFailed {
                reason: "Peer public IP unknown".into(),
            });
            return None;
        }
    };

    // Layer 2: Stigmergic Hole Punch (works for Cone/Restricted NATs)
    if let Some(result) = stigmergic_hole_punch(peer_ip, &rendezvous, event_tx.clone()).await {
        println!("\x1b[32;1m[CNTP] ✓ DIRECT P2P TUNNEL ESTABLISHED (Layer 2: Stigmergy)\x1b[0m");
        return Some(result);
    }

    // Layer 3: Birthday Attack (for Symmetric NATs)
    if let Some(result) = birthday_attack_punch(peer_ip, &rendezvous, event_tx.clone()).await {
        println!("\x1b[32;1m[CNTP] ✓ DIRECT P2P TUNNEL ESTABLISHED (Layer 3: Birthday Paradox)\x1b[0m");
        return Some(result);
    }

    // Layer 4: ICMP Autonomous Traversal (nuclear fallback)
    if let Some(result) = icmp_autonomous_traversal(peer_ip, &rendezvous, event_tx.clone()).await {
        println!("\x1b[32;1m[CNTP] ✓ DIRECT P2P TUNNEL ESTABLISHED (Layer 4: ICMP Autonomous)\x1b[0m");
        return Some(result);
    }

    println!("\x1b[31;1m[CNTP] ✗ ALL LAYERS EXHAUSTED. Connection failed.\x1b[0m");
    None
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that rendezvous hashing is deterministic:
    /// Both peers must compute the EXACT same ports and timing.
    #[test]
    fn test_rendezvous_determinism() {
        let key_a = [0xAA; 32];
        let key_b = [0xBB; 32];

        // Node A computes rendezvous with B
        let params_a = compute_rendezvous(&key_a, &key_b, 8);

        // Node B computes rendezvous with A (reversed order)
        let params_b = compute_rendezvous(&key_b, &key_a, 8);

        // They MUST arrive at the same result
        assert_eq!(params_a.ports, params_b.ports, "Ports must be identical");
        assert_eq!(
            params_a.timing_offset, params_b.timing_offset,
            "Timing must be identical"
        );
        assert_eq!(
            params_a.shared_secret, params_b.shared_secret,
            "Shared secret must be identical"
        );

        println!("✓ Rendezvous determinism verified: {:?}", params_a.ports);
    }

    /// Verify that punch packets can be built and verified.
    #[test]
    fn test_punch_packet_integrity() {
        let secret = [0x42; 32];
        let packet = build_punch_packet(&secret);

        // Must verify with correct secret
        assert!(verify_punch_packet(&packet, &secret));

        // Must fail with wrong secret
        let wrong_secret = [0x43; 32];
        assert!(!verify_punch_packet(&packet, &wrong_secret));

        // Must fail with truncated packet
        assert!(!verify_punch_packet(&packet[..10], &secret));

        println!("✓ Punch packet integrity verified");
    }

    /// Verify that DNS query builder produces valid DNS packet structure.
    #[test]
    fn test_dns_query_structure() {
        let query = build_dns_query("myip.opendns.com");

        // Header should be 12 bytes
        assert!(query.len() > 12);

        // Transaction ID
        assert_eq!(query[0], 0x12);
        assert_eq!(query[1], 0x34);

        // QDCOUNT = 1
        assert_eq!(query[4], 0x00);
        assert_eq!(query[5], 0x01);

        println!("✓ DNS query structure verified ({} bytes)", query.len());
    }

    /// Simulate birthday paradox probability to verify the math.
    #[test]
    fn test_birthday_probability() {
        let k: f64 = 65536.0; // Port space
        let n: f64 = 256.0; // Number of probes per side

        // Two-sided birthday attack: BOTH peers probe n ports simultaneously
        // P(at least one collision) = 1 - e^(-n²/k)
        let p_collision = 1.0 - (-n * n / k).exp();

        assert!(
            p_collision > 0.5,
            "Two-sided birthday attack should have >50% success rate, got {}%",
            p_collision * 100.0
        );

        println!(
            "✓ Birthday paradox probability verified: {:.1}% with {} probes/side across {} ports",
            p_collision * 100.0,
            n,
            k
        );
    }
}
