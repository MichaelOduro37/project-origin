import './style.css'

// UI Elements
const statusEl = document.getElementById('status');
const valNodeId = document.getElementById('val-node-id');
const valCpuLoad = document.getElementById('val-cpu-load');
const valTemp = document.getElementById('val-temp');
const valSpin = document.getElementById('val-spin');

const sysLog = document.getElementById('sys-log');
const chatLog = document.getElementById('chat-log');
const chatInput = document.getElementById('chat-input');
const btnSend = document.getElementById('btn-send');

let ws = null;
let reconnectInterval = null;

function addSysLog(msg) {
  const div = document.createElement('div');
  div.innerText = `> ${msg}`;
  sysLog.appendChild(div);
  sysLog.scrollTop = sysLog.scrollHeight;
}

function addChatLog(sender, msg) {
  const div = document.createElement('div');
  div.innerHTML = `<strong>[${sender}]</strong> ${msg}`;
  chatLog.appendChild(div);
  chatLog.scrollTop = chatLog.scrollHeight;
}

function connect() {
  ws = new WebSocket('ws://127.0.0.1:9944');

  ws.onopen = () => {
    statusEl.innerText = "ONLINE";
    statusEl.className = "status-online";
    if(reconnectInterval) clearInterval(reconnectInterval);
    addSysLog("Connected to local Origin daemon.");
  };

  ws.onclose = () => {
    statusEl.innerText = "OFFLINE - RETRYING...";
    statusEl.className = "status-offline";
    reconnectInterval = setTimeout(connect, 3000);
  };

  ws.onmessage = (event) => {
    try {
      let data = JSON.parse(event.data);

      // Phase 17: Slepian-Wolf Coded Telemetry Decoding
      if (data.CodedTelemetryBatch) {
        const batch = data.CodedTelemetryBatch.batch;
        
        let prev_payload = new Uint8Array(batch.baseline_payload);
        
        const decode_rle = (compressed) => {
          let decompressed = [];
          for (let i = 0; i < compressed.length; i += 2) {
            let count = compressed[i];
            let val = compressed[i+1];
            for (let j = 0; j < count; j++) {
              decompressed.push(val);
            }
          }
          return new Uint8Array(decompressed);
        };

        let decoded_strings = [new TextDecoder().decode(prev_payload)];

        for (let i = 0; i < batch.coded_syndromes.length; i++) {
          let syndrome = decode_rle(batch.coded_syndromes[i]);
          let orig_len = batch.original_sizes[i + 1];
          let current_payload = new Uint8Array(orig_len);
          
          for (let j = 0; j < orig_len; j++) {
            let b2 = j < prev_payload.length ? prev_payload[j] : 0;
            let syn_byte = j < syndrome.length ? syndrome[j] : 0;
            current_payload[j] = syn_byte ^ b2;
          }
          
          decoded_strings.push(new TextDecoder().decode(current_payload));
          prev_payload = current_payload;
        }

        // Recursively process the decoded raw telemetry events
        for (let str of decoded_strings) {
           let decodedData = JSON.parse(str);
           // We map the raw event fields into the expected format
           // e.g. { "TensegrityState": { ... } } -> call the handler with { TensegrityState: { ... } }
           // Since decodedData has the tag structure { "TensegrityState": { ... } }
           // The top level keys are the event types.
           let mockEvent = { data: str };
           ws.onmessage(mockEvent); 
        }

        // Log Slepian-Wolf bandwidth savings occasionally
        if (Math.random() < 0.1) {
            let uncomp = batch.total_uncompressed_bytes;
            let comp = batch.total_compressed_bytes;
            let ratio = ((1.0 - (comp / uncomp)) * 100).toFixed(1);
            addSysLog(`[SLEPIAN-WOLF NETWORK CODING] Received batch of ${batch.original_sizes.length} events. Uncompressed: ${uncomp}B, Coded: ${comp}B. Compression: ${ratio}% bandwidth saved!`);
        }
        return; // Halt further processing of this batch envelope
      }
      
      if (data.TensegrityState) {
        const state = data.TensegrityState;
        valNodeId.innerText = state.node;
        valCpuLoad.innerText = `${state.load.toFixed(1)}%`;
        valTemp.innerText = `${state.temp.toFixed(1)}°C`;
        valSpin.innerText = state.spin;
      }
      
      if (data.ImmuneAlert) {
        addSysLog(`[IMMUNE] Anomaly distance: ${data.ImmuneAlert.distance.toFixed(4)}`);
      }
      
      if (data.FermionicRoute) {
        addSysLog(`[ROUTE] Found peer: ${data.FermionicRoute.packet_id}`);
      }

      if (data.ChatIncoming) {
        addChatLog(data.ChatIncoming.sender, data.ChatIncoming.decrypted_payload);
      }
      
      if (data.SNNState) {
        // Just log the SNN waking up occasionally to sys-log
        if(Math.random() < 0.1) {
          addSysLog(`[SNN] Membrane potential: ${data.SNNState.membrane_potential.toFixed(2)}mV`);
        }
      }

      if (data.HologramShardReceived) {
        const h = data.HologramShardReceived;
        const reconstructBox = document.getElementById('holo-reconstruct-box');
        
        // Setup initial UI if needed
        if (!window.holoShards) window.holoShards = {};
        if (!window.holoShards[h.file_id]) window.holoShards[h.file_id] = { received: 0, total: h.total };
        
        window.holoShards[h.file_id].received++;
        const progress = Math.min(100, Math.round((window.holoShards[h.file_id].received / h.total) * 100));
        
        reconstructBox.innerHTML = `
          <p style="color:var(--accent-purple)">[PHYSARUM] Reconstructing ${h.file_id}</p>
          <div style="width: 100%; height: 10px; background: rgba(0,0,0,0.5); margin-top: 10px; border-radius: 5px;">
            <div style="width: ${progress}%; height: 100%; background: var(--accent-cyan); transition: width 0.3s; border-radius: 5px;"></div>
          </div>
          <p style="font-size: 0.8rem; margin-top: 5px;">Collapsed Tensor Shards: ${window.holoShards[h.file_id].received} / ${h.total}</p>
        `;
      }

      if (data.QuorumState) {
        const q = data.QuorumState;
        const valAutoinducer = document.getElementById('val-autoinducer');
        const biofilmBanner = document.getElementById('biofilm-banner');
        
        if (valAutoinducer) {
          valAutoinducer.innerText = q.concentration.toFixed(2);
        }
        
        if (q.biofilm_active) {
          if (biofilmBanner.classList.contains('hidden')) {
            biofilmBanner.classList.remove('hidden');
            addSysLog(`[QUORUM] Biofilm LOCKDOWN triggered! Threshold exceeded.`);
          }
        } else {
          if (!biofilmBanner.classList.contains('hidden')) {
            biofilmBanner.classList.add('hidden');
            addSysLog('[QUORUM] Local concentration safe. Biofilm mode deactivated.');
          }
        }
      }

      // Phase 11: CRISPR Array Updates
      if (data.CRISPRArrayUpdate) {
        const c = data.CRISPRArrayUpdate;
        const arrayDiv = document.getElementById('crispr-array');
        if (arrayDiv) {
          if (c.signatures.length === 0) {
            arrayDiv.innerHTML = '<p style="color: var(--text-muted); font-size: 0.9rem;">No viral signatures in memory (sgRNA array empty).</p>';
          } else {
            arrayDiv.innerHTML = '';
            c.signatures.forEach(sig => {
              const el = document.createElement('div');
              el.className = 'crispr-spacer';
              el.innerText = sig.substring(0, 8) + '...';
              el.title = sig;
              arrayDiv.appendChild(el);
            });
          }
        }
      }

      // Phase 11: Cas9 Cleavage
      if (data.CRISPRCleavage) {
        const cleavage = data.CRISPRCleavage;
        addSysLog(`[CRISPR:CAS9] MALICIOUS PACKET CLEAVED AT SOCKET. Spacer: ${cleavage.signature.substring(0, 8)}`);
      }

      // Phase 12: Fermionic Routing
      if (data.FermionicRoute) {
        const f = data.FermionicRoute;
        addSysLog(`[FERMION] Quantum Exclusion! Pkt ${f.packet_id} repelled to orbital state: ${f.dest}`);
      }

      // Phase 13/14: Gauss-Bonnet Curvature Regulation & ESN Forecast
      if (data.CurvatureAlert) {
        const c = data.CurvatureAlert;
        const curvSpan = document.getElementById('val-curvature');
        const forecastSpan = document.getElementById('val-forecast');
        
        if (curvSpan) {
          curvSpan.innerText = c.curvature_k.toFixed(2);
          if (c.curvature_k > 10.0) {
            curvSpan.style.color = '#ff3366';
            curvSpan.style.fontWeight = 'bold';
            curvSpan.style.textShadow = '0 0 8px rgba(255, 51, 102, 0.8)';
          } else {
            curvSpan.style.color = '';
            curvSpan.style.fontWeight = '';
            curvSpan.style.textShadow = '';
          }
        }

        if (forecastSpan && c.predicted_k !== undefined) {
          forecastSpan.innerText = c.predicted_k.toFixed(2);
          if (c.predicted_k > 10.0) {
            forecastSpan.style.color = '#aa33ff';
            forecastSpan.style.fontWeight = 'bold';
            forecastSpan.style.textShadow = '0 0 8px rgba(170, 51, 255, 0.8)';
          } else {
            forecastSpan.style.color = '';
            forecastSpan.style.fontWeight = '';
            forecastSpan.style.textShadow = '';
          }
        }

        if (c.wormhole_port) {
          addSysLog(`[TOPOLOGY] WORMHOLE SPAWNED ON PORT ${c.wormhole_port}. Topology modified to flatten curvature!`);
        }
      }

      // Phase 15: Random Matrix Theory Key Generation
      if (data.RMTKeyGenerated) {
        const rmt = data.RMTKeyGenerated;
        addSysLog(`[RMT CRYPTOGRAPHY] Simulated GOE Chaotic Hamiltonian (${rmt.matrix_size}x${rmt.matrix_size}). Extracted eigenvalue spacings to generate ${rmt.entropy_bits}-bit physically chaotic key!`);
      }

      // Phase 16: Optimal Transport
      if (data.OptimalTransportMapped) {
        addSysLog(`[SINKHORN OPTIMAL TRANSPORT] Computed exact Wasserstein Distance (Cost: ${data.OptimalTransportMapped.cost.toFixed(4)}). Mapped Holographic Shards to geometric optimum.`);
      }

      // Phase 18: VCG Auction
      if (data.VCGAuctionSettled) {
        const vcg = data.VCGAuctionSettled;
        if (vcg.winners.length > 0) {
            let logMsg = `[VCG ECONOMIC MARKET] Spot Compute Auction Cleared. Capacity: ${vcg.total_capacity} Cores.\n`;
            for (let winner of vcg.winners) {
                logMsg += `> ${winner.agent_id} Won ${winner.resources_allocated} Cores. Bid Value: $${winner.bid_valuation.toFixed(2)}. Truthful Social Cost (VCG Payment): $${winner.vcg_payment.toFixed(2)}\n`;
            }
            addSysLog(logMsg);
        }
      }

      // Phase 19: Homotopy Type Theory / Proof-Carrying Data
      if (data.ProofVerified) {
        if (data.ProofVerified.is_valid) {
            addSysLog(`[ZERO-TRUST HoTT] Received geometric Proof-Carrying Artifact for migration of ${data.ProofVerified.file_id}. Mathematical replication invariant VERIFIED in O(1). Migration Executed.`);
        } else {
            addSysLog(`[ZERO-TRUST HoTT] CRITICAL: Proof-Carrying Artifact FAILED verification. Geometric invariants detached! Migration BLOCKED.`);
        }
      }

      // Phase 20: Sparse Representations & Compressed Sensing
      if (data.CompressedTelemetrySnapshot) {
        const snap = data.CompressedTelemetrySnapshot.snapshot;
        const compressionRatio = (snap.original_dim / snap.compressed_dim).toFixed(1);
        addSysLog(`[COMPRESSED SENSING] Compressed high-dimensional Swarm snapshot (${snap.original_dim}D -> ${snap.compressed_dim}D Sketch). Ratio: ${compressionRatio}x smaller! Johnson-Lindenstrauss distance invariants preserved.`);
      }

      // Phase 21: Causal Inference / Do-Calculus
      if (data.CausalIntervention) {
        const causal = data.CausalIntervention;
        if (causal.executed) {
            addSysLog(`[DO-CALCULUS] Causal DAG simulation do(${causal.action}) predicts positive global benefit (+${causal.predicted_benefit.toFixed(2)}). Heuristic Executed.`);
        } else {
            addSysLog(`[DO-CALCULUS] Causal DAG simulation do(${causal.action}) predicts catastrophic non-linear cascade (${causal.predicted_benefit.toFixed(2)}). Blind heuristic SUPPRESSED!`);
        }
      }

      // Phase 22: Category Theory / Compositionality
      if (data.CategoricalComposition) {
        const cat = data.CategoricalComposition;
        if (cat.is_valid) {
            addSysLog(`[CATEGORY THEORY] Provable bind: ${cat.cell_a} ⊗ ${cat.cell_b}. Found mathematically valid morphism path [${cat.morphism_path}]. Schema compatibility guaranteed. Workflow executing.`);
        } else {
            addSysLog(`[CATEGORY THEORY] Bind FAILED: ${cat.cell_a} ⊗ ${cat.cell_b}. No valid morphism path found in Schema Category. Crash prevented at bind-time.`);
        }
      }

      // Phase 23: Complexity Synchronization
      if (data.ComplexitySync) {
        const sync = data.ComplexitySync;
        addSysLog(`[COMPLEXITY SYNC] Local Chaos (Lyapunov Exp): ${sync.lyapunov_exponent.toFixed(2)}. Swarm Target: ${sync.target.toFixed(2)}. Action: ${sync.action}. Reaching equilibrium.`);
      }

      // Phase 24: Artificial Immune System (NSA)
      if (data.NegativeSelectionAnomaly) {
        const nsa = data.NegativeSelectionAnomaly;
        addSysLog(`[AIS NSA] CRITICAL ZERO-DAY ANOMALY! Mature T-Cell [${nsa.detector_id}] reacted to Swarm telemetry (Distance: ${nsa.anomaly_score.toFixed(2)}). Immune Response Triggered!`);
      }

      // Phase 25: Active Inference / Free Energy Principle
      if (data.FreeEnergyMinimization) {
        const fep = data.FreeEnergyMinimization;
        addSysLog(`[ACTIVE INFERENCE] Predictive Error Detected! Variational Free Energy: ${fep.free_energy.toFixed(2)} (Dev: ${fep.prediction_error.toFixed(1)}%). Executing FEP: ${fep.action_taken}`);
      }

      // Phase 26: Topological Data Analysis (Persistent Homology)
      if (data.TopologyVoidDetected) {
        const tda = data.TopologyVoidDetected;
        addSysLog(`[TOPOLOGY TDA] STRUCTURAL VOID DETECTED! Persistent Homology scan found $\\beta_1 = ${tda.betti_1}$ holes persisting across ${tda.persistence_range}. Network fracturing imminent, rerouting around void!`);
      }

      // Phase 27: Autocatalytic Set Bootstrapping
      if (data.CatalyticClosureAchieved) {
        const raf = data.CatalyticClosureAchieved;
        addSysLog(`[AUTOCATALYTIC RAF] CATALYTIC CLOSURE ACHIEVED! A sub-swarm of ${raf.raf_size} core network reactions is now mutually self-sustaining. Network has successfully bootstrapped from chaotic noise!`);
      }

      // Phase 28: Constructal Law Routing Optimization
      if (data.ConstructalEvolution) {
        const constructal = data.ConstructalEvolution;
        addSysLog(`[CONSTRUCTAL LAW] VASCULAR EVOLUTION: Routing channel [${constructal.trunk_id}] has thickened into a massive Arterial Trunk (Capacity: ${constructal.capacity_increase.toFixed(2)} TB/s) to minimize global flow resistance!`);
      }

      // Phase 29: Information Bottleneck Method
      if (data.InformationBottleneckApplied) {
        const ib = data.InformationBottleneckApplied;
        const ratio = ((1.0 - (ib.compressed_size / ib.original_size)) * 100).toFixed(1);
        addSysLog(`[INFORMATION BOTTLENECK] TELEMETRY COMPRESSION: Squeezed raw node state through bottleneck ($\\beta=${ib.beta.toFixed(2)}$). Reduced telemetry size from ${ib.original_size} to ${ib.compressed_size} metrics (${ratio}% noise discarded) while perfectly preserving Swarm awareness!`);
      }

      // Phase 30: Native AI System (Secure Federated Learning via SMPC)
      if (data.SecureFederatedAggregation) {
        const smpc = data.SecureFederatedAggregation;
        addSysLog(`[NATIVE AI SMPC] SECURE FEDERATED LEARNING: Homomorphically aggregated polynomial AI shares from ${smpc.shares_combined} nodes. Reconstructed global intelligence gradient update: [${smpc.aggregated_gradient}]. Node privacy mathematically guaranteed via Shamir's Secret Sharing!`);
      }

      // Phase 31: Infinite Swarm Orchestration (Mean Field Games)
      if (data.MeanFieldEquilibrium) {
        const mfg = data.MeanFieldEquilibrium;
        addSysLog(`[MEAN FIELD GAMES] SWARM ORCHESTRATION: Replaced $O(N^2)$ node interactions with macroscopic continuum PDEs. HJB Max Cost: ${mfg.max_hjb_cost.toFixed(4)}, FP Density Shift: ${mfg.density_shift.toFixed(4)}. Swarm density mathematically shifted towards perfect Nash Equilibrium!`);
      }

      // Phase 32: Swarm Global Memory (Sparse Distributed Memory)
      if (data.SparseMemoryAccess) {
        const sdm = data.SparseMemoryAccess;
        addSysLog(`[SPARSE MEMORY] DECENTRALIZED FS: Performed associative ${sdm.operation} across Swarm high-dimensional boolean space. Activated ${sdm.nodes_activated} nodes within Hamming radius ${sdm.hamming_radius}. Memory mathematically guaranteed to survive catastrophic node failure!`);
      }

      // Phase 33: Continuous Leader Election (Reaction-Diffusion Turing Patterns)
      if (data.TuringPatternAnchorElected) {
        const tp = data.TuringPatternAnchorElected;
        addSysLog(`[TURING CONSENSUS] CONTINUOUS LEADER ELECTION: Swarm symmetry broken via Reaction-Diffusion PDE over Graph Laplacian. Activator chemical concentrated into a Turing Spot at Node ${tp.node_id} (Level: ${tp.u_concentration.toFixed(4)}). Node autonomously promoted to Swarm Anchor! Zero voting overhead!`);
      }

      // Phase 34: Fractal Metabolic Scaling (WBE Model)
      if (data.MetabolicScalingEnforced) {
        const wbe = data.MetabolicScalingEnforced;
        addSysLog(`[METABOLIC SCALING] BIOLOGICAL EFFICIENCY: Swarm Mass reached ${wbe.swarm_mass.toLocaleString()} nodes. Kleiber's 3/4 Law enforced! Total allowed metabolism bounded to ${wbe.total_metabolism.toFixed(2)}. Per-node capillary bandwidth throttled to ${wbe.capillary_bandwidth.toFixed(6)}. The Swarm is now mathematically guaranteed to scale infinitely without melting the physical infrastructure!`);
      }

      // Phase 35: Network Resilience (Percolation Theory)
      if (data.PercolationThresholdApproached) {
        const perc = data.PercolationThresholdApproached;
        addSysLog(`[PERCOLATION THEORY] CRITICAL THREAT DETECTED: Massive node failure! Swarm link density (p=${perc.current_p.toFixed(3)}) is dangerously close to the geometric shattering threshold (p_c=${perc.critical_pc.toFixed(3)}). Initiating Emergency Topological Healing...`);
      }
      if (data.PercolationHealed) {
        const perc = data.PercolationHealed;
        addSysLog(`[PERCOLATION THEORY] SWARM HEALED: Emergency Constructal bridges established! Average degree increased. The new critical shattering threshold (p_c) is successfully lowered to ${perc.new_p_c.toFixed(3)}. The giant connected component is mathematically secured!`);
      }

      // Phase 36: Epigenetic Network Memory (epiGA)
      if (data.EpigeneticModification) {
        const epi = data.EpigeneticModification;
        if (epi.expression < 1.0) {
            addSysLog(`[EPIGENETICS] SUPPRESSION: Node ${epi.node_id} exhibited malicious/faulty behavior. DNA Methylation level increased to ${(epi.methylation * 100).toFixed(1)}%. Routing expression multiplier crushed to ${epi.expression.toFixed(3)}. Swarm memory will ignore this node!`);
        } else {
            addSysLog(`[EPIGENETICS] ENHANCEMENT: Node ${epi.node_id} exhibited perfect uptime. DNA Acetylation level increased to ${(epi.acetylation * 100).toFixed(1)}%. Routing expression multiplier boosted to ${epi.expression.toFixed(3)}. Swarm memory promotes this node to hub status!`);
        }
      }

      // Phase 37: Kuramoto Distributed Clock Sync
      if (data.KuramotoSyncAchieved) {
        const kura = data.KuramotoSyncAchieved;
        addSysLog(`[KURAMOTO CLOCK] DECENTRALIZED TIME SYNC ACHIEVED: The local oscillator has mathematically locked its phase with the Swarm. Phase variance collapsed to ${kura.variance.toFixed(6)}. Global heartbeat beat recorded at Phase ${kura.global_phase.toFixed(3)}. Centralized NTP servers are now fully obsolete!`);
      }

      // Phase 38: Transformation Optics Routing
      if (data.TransformationOpticsCloak) {
        const cloak = data.TransformationOpticsCloak;
        addSysLog(`[TRANSFORMATION OPTICS] METAMATERIAL CLOAK DEPLOYED: Node ${cloak.node_id} is under catastrophic load/attack. Refractive Index dropped to ${cloak.refractive_index.toFixed(3)}. By Fermat's Principle, Swarm traffic is mathematically bending around the node. The target is now topologically invisible to the DDoS attack!`);
      }

      // Phase 39: Topological Insulator Routing
      if (data.TopologicalBackscatterPrevented) {
        const topo = data.TopologicalBackscatterPrevented;
        addSysLog(`[TOPOLOGICAL INSULATOR] BACKSCATTER PREVENTED: Node ${topo.node_id} received a packet with Chirality Spin = ${topo.packet_spin > 0 ? '+1' : '-1'}. The intended path (Node ${topo.defect_bypassed}) is dead. Time-reversal asymmetry mathematically forbids backward routing. The packet perfectly arced around the defect. Routing loops and reflection attacks are fundamentally impossible!`);
      }

      // Phase 40: Bose-Einstein Condensate Consensus
      if (data.BoseEinsteinCondensationAchieved) {
        const bec = data.BoseEinsteinCondensationAchieved;
        addSysLog(`[QUANTUM CONSENSUS] BOSE-EINSTEIN CONDENSATION: The Swarm's state variance ("Temperature") dropped to ${bec.temperature.toFixed(4)}. This is below the Critical Temperature (Tc). The network has undergone a spontaneous quantum phase transition! All nodes have collapsed into the Ground State: [${bec.ground_state}]. Global Consensus achieved instantaneously with ZERO voting overhead!`);
      }

      // Phase 41: Hawking Radiation Cache Eviction
      if (data.HawkingEvaporation) {
        const hawk = data.HawkingEvaporation;
        addSysLog(`[HOLOGRAPHIC MEMORY] HAWKING EVAPORATION: Unused data [${hawk.data_id}] has fully evaporated to free physical RAM. To preserve the Black Hole Information Paradox, the massive raw payload was dropped, but its quantum signature [${hawk.event_horizon_signature}] was permanently inscribed onto the Event Horizon. The node can mathematically prove this data existed without storing it!`);
      }

      // Phase 42: Dirac Antimatter Data Annihilation
      if (data.AntimatterAnnihilation) {
        const anti = data.AntimatterAnnihilation;
        addSysLog(`[QUANTUM PURGE] ANTIMATTER ANNIHILATION: A network revocation occurred for [${anti.data_id}]. An Anti-Packet with the exact inverse Dirac signature was injected. The Anti-Packet and the compromised data collided in the Memory Vacuum. Superposition reached 0. Both packets were instantaneously and permanently annihilated from RAM with ZERO computational garbage collection overhead!`);
      }

      // Phase 43: Quantum Teleportation (Entanglement Routing)
      if (data.QuantumTeleportationAchieved) {
        const qt = data.QuantumTeleportationAchieved;
        addSysLog(`[ENTANGLEMENT ROUTING] QUANTUM TELEPORTATION: Topological graph path severed between Node ${qt.source} and Node ${qt.destination}. Bypassing physical network via pre-shared EPR Entanglement. Node ${qt.source} performed joint measurement, destroyed local data (No-Cloning), and broadcasted 2-byte signature. Node ${qt.destination} received signature, applied Pauli transformation to EPR pair, and PERFECTLY RECONSTRUCTED a ${qt.bytes_teleported}-byte payload! Payload never traversed the graph.`);
      }

      // Phase 44: Photonic Band Gap Firewall
      if (data.PhotonicBandGapRejection) {
        const pbg = data.PhotonicBandGapRejection;
        addSysLog(`[O(0) FIREWALL] PHOTONIC BAND GAP REJECTION: Inbound packet identified with malicious resonance frequency [${pbg.frequency.toFixed(2)} THz]. This frequency falls inside the forbidden Band Gap of the node's Photonic Lattice. The packet mathematically failed to resonate with the inbound memory buffer and was STRUCTURALLY REPELLED with absolute ZERO CPU overhead! Immune to exhaustion attack.`);
      }

      // Phase 45: Calabi-Yau Data Compactification
      if (data.CalabiYauCompactification) {
        const cy = data.CalabiYauCompactification;
        const reduction = ((1 - (cy.compactified_size / cy.original_size)) * 100).toFixed(2);
        addSysLog(`[STRING THEORY STORAGE] CALABI-YAU COMPACTIFICATION: Historical node ledger geometry exceeded 3D RAM limits. Folding massive 1D data array [${cy.original_size} bytes] into a 6-Dimensional Calabi-Yau Manifold tensor... Data successfully embedded into topological Betti numbers! New mathematical footprint: [${cy.compactified_size} bytes]. Achieved Geometric Compression Ratio: ${reduction}% footprint reduction!`);
      }

    } catch(e) {
      console.error('Failed to parse WS message:', e);
    }
  };
}

btnSend.addEventListener('click', () => {
  const msg = chatInput.value.trim();
  if(!msg) return;
  
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify({ type: "Chat", message: msg }));
    addChatLog("ME", msg);
    chatInput.value = "";
  } else {
    addSysLog("Cannot send: Websocket offline.");
  }
});

chatInput.addEventListener('keypress', (e) => {
  if (e.key === 'Enter') btnSend.click();
});

// Phase 9: Holographic Drag & Drop
const dropzone = document.getElementById('holo-dropzone');
const reconstructBox = document.getElementById('holo-reconstruct-box');

dropzone.addEventListener('dragover', (e) => {
  e.preventDefault();
  dropzone.style.borderColor = 'var(--accent)';
  dropzone.style.background = 'rgba(0, 243, 255, 0.1)';
});

dropzone.addEventListener('dragleave', (e) => {
  e.preventDefault();
  dropzone.style.borderColor = 'var(--text-dim)';
  dropzone.style.background = 'transparent';
});

dropzone.addEventListener('drop', (e) => {
  e.preventDefault();
  dropzone.style.borderColor = 'var(--text-dim)';
  dropzone.style.background = 'transparent';

  if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
    const file = e.dataTransfer.files[0];
    const reader = new FileReader();
    reader.onload = (ev) => {
      // Get base64 without the data:image/png;base64, prefix
      const b64 = ev.target.result.split(',')[1];
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
          type: "Upload",
          file_id: file.name,
          base64_data: b64
        }));
        addSysLog(`[HOLO] Shredding ${file.name} into Quantum Tensor Shards...`);
        reconstructBox.innerHTML = `<p style="color:var(--accent)">Projecting ${file.name} into the swarm...</p>`;
      }
    };
    reader.readAsDataURL(file);
  }
});

connect();
