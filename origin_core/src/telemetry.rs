use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::accept_async;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TelemetryEvent {
    TensegrityState {
        node: String,
        spin: i8,
        temp: f64,
        load: f64,
    },
    ImmuneAlert {
        distance: f64,
        threshold: f64,
        quarantined: bool,
    },
    FermionicRoute {
        packet_id: String,
        origin: String,
        dest: String,
        is_quantum: bool,
    },
    ChatIncoming {
        sender: String,
        encrypted_payload: String,
        decrypted_payload: String,
    },
    SNNState {
        membrane_potential: f64,
        threshold: f64,
        sleep_interval_ms: u64,
    },
    HologramShardReceived {
        file_id: String,
        shard_index: usize,
        total: usize,
    },
    DnaFountainDropletSprayed {
        file_id: String,
        droplet_seed: u64,
        source_blocks: usize,
    },
    DnaFountainBeliefPropagation {
        file_id: String,
        blocks_recovered: usize,
        total_blocks: usize,
    },
    FileReconstructed {
        file_id: String,
        base64_data: String,
    },
    QuorumState {
        concentration: f64,
        biofilm_active: bool,
    },
    CRISPRCleavage {
        signature: String,
    },
    CRISPRArrayUpdate {
        signatures: Vec<String>,
    },
    CurvatureAlert {
        curvature_k: f64,
        predicted_k: f64,
        wormhole_port: Option<u16>,
    },
    RMTKeyGenerated {
        matrix_size: usize,
        entropy_bits: usize,
    },
    OptimalTransportMapped {
        file_id: String,
        cost: f64,
    },
    CodedTelemetryBatch {
        batch: crate::logos::information_theory::network_coding::CodedTelemetryBatch,
    },
    VCGAuctionSettled {
        winners: Vec<crate::noosphere::swarm_dynamics::VCGResult>,
        total_capacity: usize,
    },
    ProofVerified {
        file_id: String,
        is_valid: bool,
    },
    CompressedTelemetrySnapshot {
        snapshot: crate::logos::information_theory::compressed_sensing::CompressedTelemetrySnapshot,
    },
    CausalIntervention {
        action: String,
        predicted_benefit: f64,
        executed: bool,
    },
    CategoricalComposition {
        cell_a: String,
        cell_b: String,
        morphism_path: String,
        is_valid: bool,
    },
    ComplexitySync {
        lyapunov_exponent: f64,
        target: f64,
        action: String,
    },
    NegativeSelectionAnomaly {
        detector_id: String,
        anomaly_score: f64,
    },
    FreeEnergyMinimization {
        free_energy: f64,
        prediction_error: f64,
        action_taken: String,
    },
    TopologyVoidDetected {
        betti_1: usize,
        persistence_range: String,
    },
    CatalyticClosureAchieved {
        raf_size: usize,
    },
    ConstructalEvolution {
        trunk_id: String,
        capacity_increase: f64,
    },
    InformationBottleneckApplied {
        original_size: usize,
        compressed_size: usize,
        beta: f64,
    },
    SecureFederatedAggregation {
        aggregated_gradient: i64,
        shares_combined: usize,
    },
    MeanFieldEquilibrium {
        density_shift: f64,
        max_hjb_cost: f64,
    },
    SparseMemoryAccess {
        operation: String,
        hamming_radius: usize,
        nodes_activated: usize,
    },
    TuringPatternAnchorElected {
        node_id: usize,
        u_concentration: f64,
    },
    MetabolicScalingEnforced {
        swarm_mass: usize,
        total_metabolism: f64,
        capillary_bandwidth: f64,
    },
    PercolationThresholdApproached {
        current_p: f64,
        critical_pc: f64,
    },
    PercolationHealed {
        new_p_c: f64,
    },
    EpigeneticModification {
        node_id: usize,
        methylation: f64,
        acetylation: f64,
        expression: f64,
    },
    KuramotoSyncAchieved {
        global_phase: f64,
        variance: f64,
    },
    TransformationOpticsCloak {
        node_id: usize,
        refractive_index: f64,
    },
    TopologicalBackscatterPrevented {
        node_id: usize,
        packet_spin: i32,
        defect_bypassed: usize,
    },
    BoseEinsteinCondensationAchieved {
        temperature: f64,
        ground_state: String,
    },
    HawkingEvaporation {
        data_id: String,
        event_horizon_signature: String,
    },
    AntimatterAnnihilation {
        data_id: String,
    },
    QuantumTeleportationAchieved {
        source: usize,
        destination: usize,
        bytes_teleported: usize,
    },
    PhotonicBandGapRejection {
        frequency: f64,
    },
    CalabiYauCompactification {
        original_size: usize,
        compactified_size: usize,
    },
    RelativisticTimeDilation {
        node_id: usize,
        velocity: f64,
        lorentz_factor: f64,
        new_timeout_ms: u64,
    },
    QuantumTunnelingAchieved {
        payload_size: usize,
        tunneling_probability: f64,
    },
    CausalParadoxRejected {
        node_id: usize,
        ds_squared: f64,
    },
    WaveFunctionCollapsed {
        node_id: usize,
        wiretap_detected: bool,
        error_rate: f64,
    },
    ZeroEntropyRoutingAchieved {
        node_id: usize,
        packets_processed: usize,
        heat_dissipated: f64,
    },
    AperiodicEncryptionDeployed {
        payload_size: usize,
        lattice_depth: usize,
    },
    TimeCrystalOscillation {
        node_id: usize,
        temporal_period: usize,
        energy_dissipated: f64,
    },
    RibosomalTranslationComplete {
        node_id: usize,
        protein_length: usize,
        sequence: String,
    },
    CherenkovShockwaveDetected {
        node_id: usize,
        packet_velocity: f64,
        phase_limit: f64,
    },
    QuantumZenoStateFrozen {
        node_id: usize,
        target_state: String,
        observation_frequency: u64,
    },
    SonoluminescentBurst {
        node_id: usize,
        payload_size: usize,
        network_pressure: f64,
    },
    QcdHadronSnap {
        node_id: usize,
        attempted_color: String,
    },
    StrangeAttractorOrbit {
        node_id: usize,
        hops: usize,
        destination: usize,
    },
    SpinIceMonopoleIsolation {
        node_id: usize,
        protected_payload: String,
    },
    BaryogenesisGenesisRemnant {
        subnet_id: usize,
        remnant_hash: String,
        processed_particle_tensors: usize,
        survived_matter: usize,
    },
    CasimirVacuumHarvest {
        node_id: usize,
        predicted_packets: usize,
    },
    PanspermiaSporeGermination {
        subnet_id: usize,
        medium: String,
    },
    MTheoryBraneCollision {
        brane_a_id: usize,
        brane_b_id: usize,
        atomic_payload: String,
    },
    PlasmidHorizontalTransfer {
        source_node: usize,
        target_node: usize,
        immunity_signature: String,
    },
    TopologyMyelination {
        route: String,
        usage_frequency: usize,
    },
    SynapticPruning {
        route: String,
    },
    MycelialResourceShuttle {
        source_node: usize,
        starving_node: usize,
        compute_transferred: usize,
    },
    SymbiogenesisEngulfment {
        sidechain_name: String,
        new_organelle_id: usize,
    },
    QuantumDarwinismCollapse {
        objective_state: String,
        redundancy_achieved: usize,
        decohered_states: usize,
    },
    GrandUnifiedConsensusReached {
        final_state_hash: String,
        steps_executed: Vec<String>,
    },
    ApoptosisTriggered {
        contract_id: String,
        reason: String,
    },
    AutophagyRecycled {
        contract_id: String,
        bytes_freed: usize,
        compute_recovered: usize,
    },
    SybilCytolysis {
        malicious_clones_destroyed: usize,
    },
    ChrysalisEntered {
        version: String,
    },
    MetamorphosisComplete {
        new_version: String,
    },
    TransactionCatalyzed {
        tx_id: String,
        energy_saved: f64,
    },
    SpamRejected {
        tx_id: String,
        energy_barrier: f64,
    },
    DnaArchived {
        original_bytes: usize,
        dna_sequence_length: usize,
    },
    RandomnessGenerated {
        entropy_source: String,
        random_value: u64,
    },
    PayloadSilenced {
        tx_id: String,
        silenced_bytes: usize,
    },
    PayloadUnsilenced {
        tx_id: String,
    },
    ContractFolded {
        contract_id: String,
        free_energy: f64,
    },
    MisfoldRejected {
        contract_id: String,
        reason: String,
    },
    ContractSenescent {
        contract_id: String,
    },
    PiezoelectricBurst {
        current_stress: f64,
        scaled_throughput: usize,
    },
    CryptobiosisActivated {
        active_nodes: usize,
        total_nodes: usize,
    },
    CryptobiosisLifted {
        active_nodes: usize,
        total_nodes: usize,
    },
    OptogeneticToggle {
        feature: String,
        state: String,
    },
    FqheShardingInduced {
        filling_factor: f64,
        capacity: usize,
    },
    MagnetoreceptionAligned {
        heading: f64,
    },
    BlockRedshifted {
        age: usize,
        original_mb: f64,
        current_mb: f64,
    },
    AllostericActivation {
        contract_id: String,
        effector_token: String,
    },
    WormholeOpened {
        location_a: String,
        location_b: String,
        latency: u64,
    },
    ChemotacticMigration {
        target_density: f64,
    },
    TriboluminescentFlash {
        liquidity: u64,
    },
    SuperfluidState {
        processing_ns: u64,
    },
    PlasmonCoupled {
        pow_hash: u64,
        pos_weight: u64,
    },
    StemCellDifferentiated {
        new_type: String,
    },
    SuperradianceAmplified {
        power: u64,
    },
    TelomeraseSynthesized {
        current_lifespan: u64,
    },
    MonopoleCollision {
        transaction_id: String,
    },
    TachyonicPreExecution {
        latency_ms: i32,
    },
    EpigeneticInheritance {
        optimization: String,
    },
    StrangeMetalPhase {
        temp: f64,
        linear_fee: f64,
    },
    AutopoieticPatch {
        status: String,
    },
    VacuumDecayTriggered {
        status: String,
    },
    SingularityHandover {
        status: String,
    },
    // CNTP: Chemotactic NAT Traversal Protocol Events
    CntpSelfDiscovered {
        public_ip: String,
        public_port: u16,
        port_delta: Option<i32>,
        lcg_a: Option<u32>,
        lcg_c: Option<u32>,
        nat_type: String,
    },
    CntpNodeKey {
        key: String,
    },
    CntpPunchAttempt {
        layer: u8,
        ports_tried: u32,
    },
    CntpPeerConnected {
        peer_addr: String,
    },
    CntpConnectionFailed {
        reason: String,
    },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum UiCommand {
    Upload {
        file_id: String,
        base64_data: String,
    },
    Chat {
        message: String,
    },
    HologramRequest {
        file_id: String,
    },
    CntpConnect {
        peer_key: String,
    },
}

pub struct TelemetryServer {
    sender: broadcast::Sender<TelemetryEvent>,
    ui_cmd_tx: mpsc::Sender<UiCommand>,
}

impl TelemetryServer {
    pub fn new() -> (Self, mpsc::Receiver<UiCommand>) {
        let (tx, _) = broadcast::channel(100);
        let (ui_cmd_tx, ui_cmd_rx) = mpsc::channel(100);
        (
            Self {
                sender: tx,
                ui_cmd_tx,
            },
            ui_cmd_rx,
        )
    }

    pub fn get_sender(&self) -> broadcast::Sender<TelemetryEvent> {
        self.sender.clone()
    }

    pub async fn start_daemon(self, port: u16) {
        let addr = format!("0.0.0.0:{}", port);
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                println!(
                    "[TELEMETRY] CRITICAL: Failed to bind WebSocket on {}: {}",
                    addr, e
                );
                return;
            }
        };
        println!("[TELEMETRY] WebSocket daemon listening on ws://{}", addr);

        let sender = self.sender.clone();
        let ui_cmd_tx = self.ui_cmd_tx.clone();

        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let mut rx = sender.subscribe();
                let ui_tx = ui_cmd_tx.clone();
                tokio::spawn(async move {
                    if let Ok(ws_stream) = accept_async(stream).await {
                        println!("[TELEMETRY] UI Dashboard Connected!");
                        let (mut write, mut read) = ws_stream.split();

                        let mut rx_task = tokio::spawn(async move {
                            let mut batch = Vec::new();
                            let mut last_send = std::time::Instant::now();

                            loop {
                                tokio::select! {
                                    Ok(event) = rx.recv() => {
                                        if let Ok(msg) = serde_json::to_string(&event) {
                                            batch.push(msg);
                                        }
                                    }
                                    _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {}
                                }

                                if batch.len() >= 10
                                    || (batch.len() > 0 && last_send.elapsed().as_millis() >= 250)
                                {
                                    if let Some(coded_batch) = crate::logos::information_theory::network_coding::SlepianWolfEncoder::encode_batch(&batch) {
                                        let coded_event = TelemetryEvent::CodedTelemetryBatch { batch: coded_batch };
                                        if let Ok(msg) = serde_json::to_string(&coded_event) {
                                            if write.send(tokio_tungstenite::tungstenite::Message::Text(msg.into())).await.is_err() {
                                                break;
                                            }
                                        }
                                    }
                                    batch.clear();
                                    last_send = std::time::Instant::now();
                                }
                            }
                        });

                        let mut tx_task = tokio::spawn(async move {
                            while let Some(msg) = read.next().await {
                                if let Ok(tokio_tungstenite::tungstenite::Message::Text(text)) = msg
                                {
                                    if let Ok(cmd) = serde_json::from_str::<UiCommand>(&text) {
                                        let _ = ui_tx.send(cmd).await;
                                    }
                                }
                            }
                        });

                        tokio::select! {
                            _ = (&mut rx_task) => tx_task.abort(),
                            _ = (&mut tx_task) => rx_task.abort(),
                        };
                    }
                });
            }
        });
    }
}
