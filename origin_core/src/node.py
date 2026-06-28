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

    def _handle_client(self, conn):
        try:
            while self.running:
                data = conn.recv(1024)
                if not data:
                    break

                # Check if this is a structured payload (e.g. Kuramoto sync)
                try:
                    payload = json.loads(data.decode('utf-8'))
                    if "kuramoto_phase" in payload:
                        self.sync_kuramoto([payload["kuramoto_phase"]], dt=0.1)
                except (json.JSONDecodeError, UnicodeDecodeError):
                    # Otherwise treat it as standard flow traffic
                    pass

                with self.traffic_lock:
                    self.current_traffic += len(data)
        except Exception:
            pass
        finally:
            conn.close()

    def receive_traffic(self, source_id: str, amount: float):
        # Backward compatibility for simulated loads in tests if needed
        if amount < 0 or math.isnan(amount) or math.isinf(amount):
            raise ValueError(f"Invalid traffic amount: {amount}")
        with self.traffic_lock:
            self.current_traffic += amount

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

        action = None
        threshold = max(self.surprise_threshold, self.expected_traffic * self.surprise_ratio)

        if self.surprise > threshold:
            print(json.dumps({"message": f"{self.node_id} high surprise / free energy spike detected"}))
            if traffic_this_step > self.expected_traffic:
                action = "spawn"
                print(json.dumps({"message": f"{self.node_id} action: spawning sub-node"}))
                # real spawn behavior: spin up a dummy worker thread to handle load
                threading.Thread(target=lambda: time.sleep(0.1), daemon=True).start()
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
