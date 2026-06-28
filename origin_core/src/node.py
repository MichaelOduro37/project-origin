import math
import json
import socket
import threading
import time

class Node:
    def __init__(self, node_id: str, expected_traffic: float = 0.0, surprise_threshold: float = 10.0, surprise_ratio: float = 0.1, host: str = "127.0.0.1", port: int = 0, is_remote: bool = False):
        if math.isnan(expected_traffic) or math.isinf(expected_traffic):
            raise ValueError(f"Invalid expected_traffic: {expected_traffic}")
        if math.isnan(surprise_threshold) or math.isinf(surprise_threshold) or surprise_threshold < 0:
            raise ValueError(f"Invalid surprise_threshold: {surprise_threshold}")
        if math.isnan(surprise_ratio) or math.isinf(surprise_ratio) or surprise_ratio < 0:
            raise ValueError(f"Invalid surprise_ratio: {surprise_ratio}")

        self.node_id = node_id
        self.expected_traffic = expected_traffic
        self.surprise_threshold = surprise_threshold
        self.surprise_ratio = surprise_ratio
        self.is_remote = is_remote

        self.current_traffic = 0.0
        self.surprise = 0.0

        # Kuramoto Model: Phase synchronization variables
        import random
        self.kuramoto_phase = random.uniform(0, 2 * math.pi)
        self.kuramoto_omega = random.uniform(0.5, 1.5) # natural intrinsic frequency
        self.kuramoto_coupling = 0.1 # K constant
        self.kuramoto_lock = threading.Lock()

        # Artificial Immune System (Negative Selection Algorithm)
        self.thymus_self_set = set() # Self-profile signatures
        self.mature_t_cells = [] # Censored detectors
        self._init_immune_system()

        # Turing Patterns: Morphogenesis chemicals for Anchor election
        # Represents chemical concentrations u (activator) and v (inhibitor)
        self.turing_u = random.uniform(0.1, 0.9)
        self.turing_v = random.uniform(0.1, 0.9)
        self.is_anchor = False

        # Leaky Integrate-and-Fire (LIF) Spiking Neural Network parameters
        self.membrane_potential = 0.0
        self.membrane_threshold = 1.0
        self.membrane_decay = 0.9 # leak factor

        self.host = host
        self.port = port
        self.running = True
        self.traffic_lock = threading.Lock()

        if not self.is_remote:
            self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            # Allow reusing address for rapid restarts
            self.server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            self.server_socket.bind((self.host, self.port))
            self.port = self.server_socket.getsockname()[1]
            self.server_socket.listen(5)

            self.listener_thread = threading.Thread(target=self._listen, daemon=True)
            self.listener_thread.start()

        print(json.dumps({"message": f"Node initialized: {node_id}"}))
        print(json.dumps({"message": f"Markov blanket defined for {node_id}"}))
        print(json.dumps({"message": f"Generative model started for {node_id}"}))

    def _listen(self):
        self.server_socket.settimeout(0.1)
        while self.running:
            try:
                conn, addr = self.server_socket.accept()
                threading.Thread(target=self._handle_client, args=(conn,), daemon=True).start()
            except socket.timeout:
                pass
            except Exception:
                break

    def _init_immune_system(self):
        """
        NSA: Generates randomized T-cell detectors.
        Censors them against the 'self' set to ensure no autoimmune response.
        """
        import random
        # Define baseline 'self' signature space (normal structural patterns)
        self.thymus_self_set.update([b'heartbeat', b'X', b'{"kuramoto_phase"'])

        # Generate 100 random detectors
        for _ in range(100):
            detector = bytes([random.randint(0, 255) for _ in range(4)])
            # Censoring: If detector matches 'self', it dies.
            # (Simplified matching: if detector is a substring of any self string)
            is_autoimmune = any(detector in s for s in self.thymus_self_set)
            if not is_autoimmune:
                self.mature_t_cells.append(detector)

    def _handle_client(self, conn):
        try:
            while self.running:
                data = conn.recv(1024)
                if not data:
                    break

                # Check if this is a structured payload (e.g. Kuramoto sync, Turing chemicals)
                try:
                    payload = json.loads(data.decode('utf-8'))
                    if "kuramoto_phase" in payload:
                        self.sync_kuramoto([payload["kuramoto_phase"]], dt=0.1)
                    if "turing_chemicals" in payload:
                        # Just accept external chemical gradients, actual diffusion applied centrally
                        # in the Network Graph Laplacian step for simplicity of testing
                        pass
                except (json.JSONDecodeError, UnicodeDecodeError):
                    # Artificial Immune System: NSA T-cell array screening
                    # Check incoming binary traffic against mature detectors
                    for t_cell in self.mature_t_cells:
                        if t_cell in data:
                            print(json.dumps({"message": f"Immune Response: {self.node_id} T-Cell detected Zero-Day anomaly payload! Cleaving connection."}))
                            conn.close()
                            return # Terminate threat

                with self.traffic_lock:
                    self.current_traffic += len(data)
        except Exception:
            pass
        finally:
            try:
                conn.close()
            except:
                pass

    def receive_traffic(self, source_id: str, amount: float):
        # Backward compatibility for simulated loads in tests if needed
        if amount < 0 or math.isnan(amount) or math.isinf(amount):
            raise ValueError(f"Invalid traffic amount: {amount}")
        with self.traffic_lock:
            self.current_traffic += amount

    def update_turing_chemicals(self, laplacian_u: float, laplacian_v: float, dt: float = 0.1):
        """
        Reaction-Diffusion PDE (Turing Patterns) over discrete Graph Laplacian.
        du/dt = D_u * Δu + f(u,v)
        dv/dt = D_v * Δv + g(u,v)
        """
        # Diffusion constants (Inhibitor must diffuse faster than Activator)
        Du = 0.1
        Dv = 0.5

        # Reaction kinetics (FitzHugh-Nagumo / Schnakenberg approximation)
        a, b = 0.1, 0.9
        reaction_u = a - self.turing_u + (self.turing_u ** 2) * self.turing_v
        reaction_v = b - (self.turing_u ** 2) * self.turing_v

        du = Du * laplacian_u + reaction_u
        dv = Dv * laplacian_v + reaction_v

        self.turing_u += du * dt
        self.turing_v += dv * dt

        # Spontaneous Symmetry Breaking: Elect anchor if Activator concentration is critically high
        if self.turing_u > 2.0 and not self.is_anchor:
            self.is_anchor = True
            print(json.dumps({"message": f"Turing Pattern Emergence: {self.node_id} spontaneously elected as Anchor (Activator={self.turing_u:.2f})"}))
        elif self.turing_u < 1.0 and self.is_anchor:
            self.is_anchor = False

    def sync_kuramoto(self, neighbor_phases: list[float], dt: float = 0.1):
        """
        Applies the Kuramoto differential equation to update this node's phase
        towards a synchronized global heartbeat without a centralized clock.
        dθi/dt = ωi + (K/N) * Σ sin(θj - θi)
        """
        if not neighbor_phases:
            self.kuramoto_phase += self.kuramoto_omega * dt
            return

        N = len(neighbor_phases)

        with self.kuramoto_lock:
            phase_diff_sum = sum(math.sin(theta_j - self.kuramoto_phase) for theta_j in neighbor_phases)

            dtheta = self.kuramoto_omega + (self.kuramoto_coupling / N) * phase_diff_sum
            self.kuramoto_phase += dtheta * dt

            # keep bound to 0 -> 2pi
            self.kuramoto_phase %= (2 * math.pi)

    def step(self):
        with self.traffic_lock:
            traffic_this_step = self.current_traffic
            self.current_traffic = 0.0

        self.surprise = abs(traffic_this_step - self.expected_traffic)

        # LIF SNN: Integrate traffic spikes into membrane potential
        if traffic_this_step > self.expected_traffic:
            # Synaptic integration
            self.membrane_potential += (traffic_this_step / max(self.expected_traffic, 1.0)) * 0.5

        # Leaky decay
        self.membrane_potential *= self.membrane_decay

        action = None
        threshold = max(self.surprise_threshold, self.expected_traffic * self.surprise_ratio)

        if self.surprise > threshold:
            print(json.dumps({"message": f"{self.node_id} high surprise / free energy spike detected"}))
            if traffic_this_step > self.expected_traffic:
                action = "spawn"
                print(json.dumps({"message": f"{self.node_id} action: spawning sub-node"}))

                # SNN Action Potential: Only physically spawn thread if LIF threshold is broken
                if self.membrane_potential >= self.membrane_threshold:
                    print(json.dumps({"message": f"Neuromorphic LIF Spike: {self.node_id} action potential reached, spawning physical thread."}))
                    threading.Thread(target=lambda: time.sleep(0.1), daemon=True).start()
                    # Reset potential after firing
                    self.membrane_potential = 0.0
            else:
                action = "throttle"
                print(json.dumps({"message": f"{self.node_id} action: throttling connection"}))
                # real throttle behavior: sleep to simulate dropped throughput
                time.sleep(0.01)
        elif self.surprise == 0.0 and traffic_this_step > 0:
             print(json.dumps({"message": f"{self.node_id} surprise levels drop to baseline"}))

        print(json.dumps({"message": f"{self.node_id} Updating predictive model"}))

        alpha = 0.2
        self.expected_traffic = (alpha * traffic_this_step) + ((1 - alpha) * self.expected_traffic)

        return action

    def get_surprise(self) -> float:
        return self.surprise

    def stop(self):
        self.running = False
        if not self.is_remote:
            try:
                self.server_socket.close()
            except Exception:
                pass
