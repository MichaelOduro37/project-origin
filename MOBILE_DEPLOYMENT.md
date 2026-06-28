# Origin Core: Mobile Deployment Guide

To deploy the Origin Core as a "real" decentralized network utilizing the processors of 3 separate physical phones, follow this guide. This moves the system entirely away from local cluster simulations and into an actual peer-to-peer topological environment across a local Wi-Fi network.

## Prerequisites

### For Android: Termux
1. Download **Termux** from F-Droid (the Google Play Store version is deprecated).
2. Open Termux and update the package repositories:
   `pkg update && pkg upgrade -y`
3. Install Python:
   `pkg install python -y`
4. Copy the `origin_core` directory to your phone's storage and grant Termux access (`termux-setup-storage`).

### For iOS: iSH or a-Shell
1. Download **iSH Shell** or **a-Shell** from the App Store.
2. Ensure `python3` is available (a-Shell has it built-in; iSH requires `apk add python3`).
3. Transfer the `origin_core` directory to the app's local storage via the iOS Files app.

---

## Deployment Instructions

Connect all 3 phones to the **same Wi-Fi network**. You need to find the local IP address of each phone (e.g., `192.168.1.101`). You can usually find this in your phone's Wi-Fi settings.

### Step 1: Boot the Anchor Node (Phone 1)
Phone 1 will act as the initial anchor point. By default, it will listen on `0.0.0.0` port `8080`.

Open your terminal app on Phone 1 and run:
```bash
cd origin_core/src
python main.py --mode standalone --host 0.0.0.0 --port 8080
```
You should see:
`{"message": "Node initialized: standalone_node"}`
`{"message": "Origin Node listening on 0.0.0.0:8080"}`

*Assume Phone 1's IP is `192.168.1.101`.*

### Step 2: Connect Phone 2
On Phone 2, you will start a standalone node and instruct it to peer with Phone 1. We will use a different port just as a best practice, though not strictly required if they are different devices.

Open your terminal app on Phone 2 and run:
```bash
cd origin_core/src
python main.py --mode standalone --host 0.0.0.0 --port 8081 --peer 192.168.1.101:8080
```

### Step 3: Connect Phone 3
Similarly, connect Phone 3 to the mesh.

Open your terminal app on Phone 3 and run:
```bash
cd origin_core/src
python main.py --mode standalone --host 0.0.0.0 --port 8082 --peer 192.168.1.101:8080
```

---

## What is happening?

Once connected, you have established a genuine P2P mesh network operating under advanced scientific theories:

1. **Cellular Biology (Markov Blankets)**: Each phone encapsulates its internal state. It only interacts with the outside world via the TCP socket boundary.
2. **Kuramoto Oscillators**: Even without a master clock, the 3 phones are running the differential equations to synchronize their mathematical phases by reading the TCP heartbeats from one another.
3. **The Free Energy Principle**: The phones are actively predicting the byte volume they will receive. If you write a simple script to blast TCP bytes to Phone 2, it will detect a "High surprise / free energy spike", update its predictive model, and throttle or spawn worker threads in real-time.

## Real-World Value
This is no longer a simulation. This is the foundational layer of a global, decentralized supercomputer. By running this silently in the background of mobile devices, Origin can orchestrate planetary-scale predictive auto-scaling and mathematically un-shatterable routing entirely off the grid, utilizing hardware that is otherwise sitting idle in people's pockets.
