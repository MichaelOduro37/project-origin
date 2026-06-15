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
      const data = JSON.parse(event.data);
      
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

    } catch(e) {
      console.error(e);
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
