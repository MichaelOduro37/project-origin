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
