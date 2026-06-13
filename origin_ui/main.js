// Origin Core - Phase 6 Deep Physics UI Logic
document.addEventListener('DOMContentLoaded', () => {
  initChaoticAttractor();
  initTensegrityMesh();
  initDataSimulations();
});

// 1. Chaotic Attractor Background (Inspired by RMT Keygen)
function initChaoticAttractor() {
  const canvas = document.getElementById('chaos-canvas');
  const ctx = canvas.getContext('2d');
  
  let width, height;
  function resize() {
    width = canvas.width = window.innerWidth;
    height = canvas.height = window.innerHeight;
  }
  window.addEventListener('resize', resize);
  resize();

  // Lorenz Attractor logic for chaotic particle paths
  let x = 0.1, y = 0, z = 0;
  const a = 10, b = 28, c = 8/3;
  const dt = 0.01;
  const scale = 15;
  
  ctx.fillStyle = '#030305';
  ctx.fillRect(0, 0, width, height);

  function drawChaos() {
    ctx.fillStyle = 'rgba(3, 3, 5, 0.05)'; // Fade effect
    ctx.fillRect(0, 0, width, height);

    for(let i=0; i<10; i++) { // Draw multiple steps per frame to speed up
      let dx = (a * (y - x)) * dt;
      let dy = (x * (b - z) - y) * dt;
      let dz = (x * y - c * z) * dt;
      
      x += dx; y += dy; z += dz;

      // Project 3D to 2D
      const px = width/2 + x * scale;
      const py = height/2 + y * scale;

      ctx.beginPath();
      ctx.arc(px, py, 1.5, 0, Math.PI*2);
      ctx.fillStyle = `rgba(138, 43, 226, ${Math.random() * 0.5 + 0.1})`; // Fermionic Purple
      ctx.fill();
      
      // Mirror for symmetric quantum feel
      ctx.beginPath();
      ctx.arc(width - px, py, 1, 0, Math.PI*2);
      ctx.fillStyle = `rgba(0, 240, 255, ${Math.random() * 0.5 + 0.1})`; // Hawking Cyan
      ctx.fill();
    }
    requestAnimationFrame(drawChaos);
  }
  drawChaos();
}

// 2. Tensegrity Mesh Rendering
function initTensegrityMesh() {
  const container = document.getElementById('tensegrity-mesh');
  const numNodes = 8;
  const nodes = [];

  // Center Self Node
  const selfNode = document.createElement('div');
  selfNode.className = 'node self';
  selfNode.style.left = '50%';
  selfNode.style.top = '50%';
  container.appendChild(selfNode);
  nodes.push({el: selfNode, x: 50, y: 50});

  // Peer Nodes
  for(let i=0; i<numNodes; i++) {
    const peer = document.createElement('div');
    peer.className = 'node peer';
    const angle = (i / numNodes) * Math.PI * 2;
    const dist = 30 + Math.random() * 15;
    const x = 50 + Math.cos(angle) * dist;
    const y = 50 + Math.sin(angle) * dist;
    
    peer.style.left = `${x}%`;
    peer.style.top = `${y}%`;
    container.appendChild(peer);
    nodes.push({el: peer, x, y});

    // Draw connection to center
    const line = document.createElement('div');
    line.className = 'connection-line';
    line.style.left = '50%';
    line.style.top = '50%';
    
    const dx = x - 50;
    const dy = y - 50;
    const length = Math.sqrt(dx*dx + dy*dy);
    const rot = Math.atan2(dy, dx) * 180 / Math.PI;
    
    line.style.width = `calc(${length}% - 10px)`; // Account for padding
    line.style.transform = `rotate(${rot}deg)`;
    container.appendChild(line);
  }

  // Animate node breathing
  setInterval(() => {
    nodes.slice(1).forEach((node, i) => {
      const wobbleX = (Math.random() - 0.5) * 5;
      const wobbleY = (Math.random() - 0.5) * 5;
      node.el.style.transform = `translate(calc(-50% + ${wobbleX}px), calc(-50% + ${wobbleY}px))`;
    });
  }, 1000);
}

// 3. Data Simulation (Connected to Real Core WebSockets)
function initDataSimulations() {
  const spinStateEl = document.getElementById('spin-state');
  const tempEl = document.getElementById('thermal-load');
  const energyEl = document.getElementById('hamiltonian-energy');
  const logEl = document.getElementById('quarantine-log');
  const kAlphaBar = document.getElementById('k-alpha-bar');
  const routeList = document.getElementById('route-list');
  const chatFeed = document.getElementById('chat-feed');
  const chatInput = document.getElementById('chat-input');
  const chatSendBtn = document.getElementById('chat-send');

  const ws = new WebSocket('ws://127.0.0.1:9944');

  ws.onopen = () => {
    addLogEntry('> \x1b[32m[SYSTEM]\x1b[0m WebSocket linked to Origin Core.');
  };

  ws.onmessage = (event) => {
    const data = JSON.parse(event.data);

    if (data.TensegrityState) {
      const state = data.TensegrityState;
      if (state.spin === -1) {
        spinStateEl.textContent = "-1 (SHEDDING)";
        spinStateEl.className = "value negative";
      } else {
        spinStateEl.textContent = "+1 (ACCEPTING)";
        spinStateEl.className = "value positive";
      }
      if (state.temp === 0.0) {
        tempEl.textContent = `[RESTRICTED]`;
      } else {
        tempEl.textContent = `${state.temp.toFixed(1)}°C`;
      }
      energyEl.textContent = `${(0.02 + Math.random()*0.01).toFixed(4)} eV`; // Simulated visual
    }

    if (data.ImmuneAlert) {
      const alert = data.ImmuneAlert;
      addLogEntry(`> \x1b[31m[ANOMALY]\x1b[0m HDC distance: ${alert.distance.toFixed(4)}. Quarantined: ${alert.quarantined}`, true);
      kAlphaBar.style.width = `${Math.min(100, parseFloat(kAlphaBar.style.width || '15') + 25)}%`;
      
      setTimeout(() => {
        kAlphaBar.style.width = `${Math.max(5, parseFloat(kAlphaBar.style.width) - 5)}%`;
      }, 2000);
    }

    if (data.FermionicRoute) {
      const route = data.FermionicRoute;
      const el = document.createElement('div');
      el.className = 'route-item';
      el.innerHTML = `
        <span class="route-id">PCKT_0x${route.packet_id}</span>
        <span class="route-path">${route.origin} &rarr; ${route.dest}</span>
        <span class="route-type quantum">${route.is_quantum ? 'Fermionic Leap' : 'Classical Route'}</span>
      `;
      routeList.prepend(el);
      if(routeList.children.length > 5) {
        routeList.lastChild.remove();
      }
    }

    if (data.ChatIncoming) {
      const msg = data.ChatIncoming;
      renderIncomingMessage(msg.sender, msg.encrypted_payload, msg.decrypted_payload);
    }
  };

  ws.onclose = () => {
    addLogEntry('> \x1b[31m[ERROR]\x1b[0m Core disconnected. Attempting to reconnect...', true);
  };

  // Chat Sending Logic
  function sendMessage() {
    const text = chatInput.value.trim();
    if (text === '') return;
    
    // Add to UI immediately
    const el = document.createElement('div');
    el.className = 'chat-message outgoing';
    el.innerHTML = `<div class="sender">YOU (Node_0)</div><div class="body">${text}</div>`;
    chatFeed.appendChild(el);
    chatFeed.scrollTop = chatFeed.scrollHeight;

    // Send to WebSocket
    ws.send(JSON.stringify({ message: text }));
    chatInput.value = '';
  }

  chatSendBtn.addEventListener('click', sendMessage);
  chatInput.addEventListener('keypress', (e) => {
    if(e.key === 'Enter') sendMessage();
  });

  // Chaotic Decryption Visual Effect
  function renderIncomingMessage(sender, encrypted, decrypted) {
    const el = document.createElement('div');
    el.className = 'chat-message incoming';
    el.innerHTML = `<div class="sender">${sender}</div><div class="body" style="font-family: monospace; color: var(--accent-cyan);">${encrypted}</div>`;
    chatFeed.appendChild(el);
    chatFeed.scrollTop = chatFeed.scrollHeight;

    // Animate decryption over 1.5 seconds
    const body = el.querySelector('.body');
    let iterations = 0;
    const maxIters = 20;
    const interval = setInterval(() => {
      let chaoticText = '';
      for(let i=0; i<decrypted.length; i++) {
        chaoticText += String.fromCharCode(33 + Math.floor(Math.random() * 94));
      }
      body.innerText = chaoticText;
      iterations++;
      if(iterations >= maxIters) {
        clearInterval(interval);
        body.innerText = decrypted;
        body.style.fontFamily = 'var(--font-main)';
        body.style.color = '#fff';
      }
    }, 50);
  }

  function addLogEntry(msg, isAlert = false) {
    const li = document.createElement('li');
    li.textContent = msg;
    if (isAlert) li.className = 'alert';
    logEl.appendChild(li);
    const terminal = logEl.parentElement;
    terminal.scrollTop = terminal.scrollHeight;
  }
}
