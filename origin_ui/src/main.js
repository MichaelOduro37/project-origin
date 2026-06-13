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
      
    } catch(e) {
      console.error(e);
    }
  };
}

btnSend.addEventListener('click', () => {
  const msg = chatInput.value.trim();
  if(!msg) return;
  
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify({ message: msg }));
    addChatLog("ME", msg);
    chatInput.value = "";
  } else {
    addSysLog("Cannot send: Websocket offline.");
  }
});

chatInput.addEventListener('keypress', (e) => {
  if (e.key === 'Enter') btnSend.click();
});

connect();
