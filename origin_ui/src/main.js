import './style.css'

// -----------------------------------------------------
// 1. CHAOTIC ATTRACTOR CANVAS RENDERER
// -----------------------------------------------------
const canvas = document.getElementById('chaos-canvas');
const ctx = canvas.getContext('2d');

let width, height;
function resize() {
  width = window.innerWidth;
  height = window.innerHeight;
  canvas.width = width;
  canvas.height = height;
}
window.addEventListener('resize', resize);
resize();

// Lorenz Attractor Parameters
let x = 0.1, y = 0, z = 0;
const sigma = 10;
const rho = 28;
const beta = 8/3;
const dt = 0.01;

let points = [];
const maxPoints = 2000;

function drawChaos() {
  ctx.fillStyle = 'rgba(3, 3, 5, 0.1)'; // Fade effect for trails
  ctx.fillRect(0, 0, width, height);

  // Compute next step
  const dx = (sigma * (y - x)) * dt;
  const dy = (x * (rho - z) - y) * dt;
  const dz = (x * y - beta * z) * dt;

  x += dx;
  y += dy;
  z += dz;

  points.push({x, y, z});
  if(points.length > maxPoints) {
    points.shift();
  }

  // Draw points
  ctx.beginPath();
  for(let i=0; i<points.length; i++) {
    const p = points[i];
    // Project 3D to 2D
    const scale = 15;
    const px = width/2 + p.x * scale;
    const py = height/2 + p.y * scale;

    if(i === 0) ctx.moveTo(px, py);
    else ctx.lineTo(px, py);
  }
  
  ctx.strokeStyle = 'rgba(138, 43, 226, 0.5)'; // Purple accent
  ctx.lineWidth = 1.5;
  ctx.stroke();

  requestAnimationFrame(drawChaos);
}
drawChaos();

// -----------------------------------------------------
// 2. WEBSOCKET CONNECTION TO CORE DAEMON
// -----------------------------------------------------
const statusEl = document.getElementById('connection-status');
const pulseDot = document.querySelector('.pulse-dot');
const spinStateEl = document.getElementById('spin-state');
const thermalLoadEl = document.getElementById('thermal-load');
const hamiltonianEnergyEl = document.getElementById('hamiltonian-energy');
const quarantineLogEl = document.getElementById('quarantine-log');
const chatFeed = document.getElementById('chat-feed');
const chatInput = document.getElementById('chat-input');
const chatSend = document.getElementById('chat-send');
const routeList = document.getElementById('route-list');

let ws = null;
let reconnectInterval = null;

function connect() {
  ws = new WebSocket('ws://127.0.0.1:8080');

  ws.onopen = () => {
    statusEl.innerText = "Tensegrity Mesh Linked";
    statusEl.style.color = "var(--accent-cyan)";
    pulseDot.classList.add('active');
    pulseDot.style.backgroundColor = "var(--accent-cyan)";
    pulseDot.style.boxShadow = "0 0 15px var(--accent-cyan)";
    if(reconnectInterval) clearInterval(reconnectInterval);
    addLog("System securely linked to Daemon.", "alert");
  };

  ws.onclose = () => {
    statusEl.innerText = "Mesh Disconnected. Retrying...";
    statusEl.style.color = "var(--accent-red)";
    pulseDot.classList.remove('active');
    pulseDot.style.backgroundColor = "var(--accent-red)";
    pulseDot.style.boxShadow = "0 0 15px var(--accent-red)";
    reconnectInterval = setTimeout(connect, 3000);
  };

  ws.onmessage = (event) => {
    try {
      const payload = JSON.parse(event.data);
      handlePayload(payload);
    } catch(e) {
      console.error("Invalid WS payload", e);
    }
  };
}

function handlePayload(payload) {
  // 1. Chat Messages
  if (payload.type === "chat_message") {
    appendChat(payload.sender, payload.message, payload.sender === "local" ? "outgoing" : "incoming");
  }

  // 2. Telemetry Updates
  if (payload.type === "telemetry") {
    // Randomize slightly for the visual effect if the backend sends static values
    spinStateEl.innerText = payload.spin > 0 ? `+${payload.spin} (ACCEPT)` : `${payload.spin} (REJECT)`;
    spinStateEl.className = payload.spin > 0 ? "value positive" : "value negative";
    
    thermalLoadEl.innerText = `${payload.thermal.toFixed(2)}°C`;
    hamiltonianEnergyEl.innerText = `${payload.energy.toFixed(3)} eV`;
    
    // Add to Route List randomly to simulate nonlocal routing
    if (Math.random() > 0.8) {
      addRoute(payload.route_id || "FERMION-" + Math.floor(Math.random()*10000), payload.distance || Math.random().toFixed(4));
    }
  }

  // 3. HDC Immune Events
  if (payload.type === "hdc_event") {
    addLog(`> Anomaly detected: Dist ${payload.distance.toFixed(2)}`, "alert");
    const kAlphaBar = document.getElementById('k-alpha-bar');
    kAlphaBar.style.width = Math.min(100, payload.distance * 100) + "%";
  }
}

// -----------------------------------------------------
// 3. UI INTERACTIONS & HELPERS
// -----------------------------------------------------
function appendChat(sender, message, type) {
  const el = document.createElement('div');
  el.className = `chat-message ${type}`;
  el.innerHTML = `<div class="sender">${sender}</div>${message}`;
  chatFeed.appendChild(el);
  chatFeed.scrollTop = chatFeed.scrollHeight;
}

function addLog(text, className = "") {
  const li = document.createElement('li');
  li.className = className;
  li.innerText = text;
  quarantineLogEl.appendChild(li);
  if(quarantineLogEl.children.length > 5) quarantineLogEl.removeChild(quarantineLogEl.children[0]);
}

function addRoute(id, dist) {
  const el = document.createElement('div');
  el.className = 'route-item';
  el.innerHTML = `
    <div>
      <div class="route-id">NODE::${id}</div>
      <div class="route-path">Distance metric: ${dist}</div>
    </div>
    <div class="route-type quantum">Quantum Entangled</div>
  `;
  routeList.prepend(el);
  if(routeList.children.length > 3) routeList.removeChild(routeList.lastChild);
}

// Simulate some initial routes
addRoute("A05-LOCAL", "0.0001");
addRoute("PEER-9XF2", "0.4122");

chatSend.addEventListener('click', () => {
  const msg = chatInput.value.trim();
  if(!msg) return;
  
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify({ type: "chat_send", message: msg }));
    appendChat("local", msg, "outgoing");
  } else {
    appendChat("SYSTEM", "Cannot transmit. Mesh offline.", "system");
  }
  chatInput.value = "";
});

chatInput.addEventListener('keypress', (e) => {
  if (e.key === 'Enter') chatSend.click();
});

// Start connection
connect();

