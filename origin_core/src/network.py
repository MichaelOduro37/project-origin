import json
import socket
import threading

class Network:
    """
    Implements Constructal Law. Manages actual active TCP connections between nodes.
    """
    def __init__(self, num_nodes=5):
        self.num_nodes = num_nodes
        from node import Node

        # Start node TCP servers
        self.nodes = {f"node_{i}": Node(node_id=f"node_{i}", expected_traffic=10.0) for i in range(num_nodes)}

        self.active_connections = {} # track established inter-node socket connections
        self.edges = []

        self.resistance = 0.0
        self.latency = 0.0

        # Hebbian Learning: Synaptic weights
        self.synaptic_weights = {} # track connection strength (myelination)

        self.rewire_mesh() # establish real mesh topology

    def _disconnect_all(self):
        for (u, v), sock in self.active_connections.items():
            try:
                sock.close()
            except:
                pass
        self.active_connections.clear()
        self.edges.clear()

    def _connect(self, src_id, tgt_id):
        src_node = self.nodes.get(src_id)
        tgt_node = self.nodes.get(tgt_id)
        if not src_node or not tgt_node: return

        try:
            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            s.connect((tgt_node.host, tgt_node.port))
            self.active_connections[(src_id, tgt_id)] = s
            self.edges.append((src_id, tgt_id))
            if (src_id, tgt_id) not in self.synaptic_weights:
                self.synaptic_weights[(src_id, tgt_id)] = 1.0 # default weight
        except Exception:
            pass

    def evaluate_percolation_threshold(self):
        """
        Percolation Theory: Evaluates if the network is close to shattering.
        Critical threshold p_c ≈ 1 / (average degree).
        """
        if self.num_nodes == 0 or len(self.edges) == 0:
            return

        avg_degree = (2.0 * len(self.edges)) / self.num_nodes
        if avg_degree == 0:
            return

        p_c = 1.0 / avg_degree
        current_density = len(self.edges) / (self.num_nodes * (self.num_nodes - 1) / 2)

        if current_density < p_c:
            print(json.dumps({"message": f"Percolation Alert: Network density ({current_density:.2f}) below critical threshold ({p_c:.2f}). Shattering risk high."}))

    def apply_hebbian_learning(self):
        """
        Neuroplasticity: 'Cells that fire together, wire together.'
        Strengthens (myelinates) connections if both nodes are highly active.
        """
        for (src_id, tgt_id) in self.edges:
            src_node = self.nodes.get(src_id)
            tgt_node = self.nodes.get(tgt_id)
            if src_node and tgt_node:
                # Use current surprise or traffic to determine mutual activity
                if src_node.current_traffic > 5 and tgt_node.current_traffic > 5:
                    self.synaptic_weights[(src_id, tgt_id)] += 0.1 # myelinate
                    print(json.dumps({"message": f"Hebbian myelination: {src_id} -> {tgt_id} strengthened to {self.synaptic_weights[(src_id, tgt_id)]:.2f}"}))
                else:
                    self.synaptic_weights[(src_id, tgt_id)] = max(0.1, self.synaptic_weights[(src_id, tgt_id)] - 0.01) # prune

    def apply_turing_patterns(self):
        """
        Computes the discrete Graph Laplacian to diffuse Turing Chemicals (Activator/Inhibitor)
        across the physical TCP network topology, triggering spontaneous decentralized Leader election.
        """
        for node_id, node in self.nodes.items():
            # Find neighbors for the Laplacian
            neighbors = []
            for (u, v) in self.edges:
                if u == node_id: neighbors.append(self.nodes.get(v))
                if v == node_id: neighbors.append(self.nodes.get(u))
            neighbors = [n for n in neighbors if n is not None]

            degree = len(neighbors)
            if degree == 0:
                laplacian_u = 0.0
                laplacian_v = 0.0
            else:
                # Δf_i = Σ(f_j) - d_i * f_i
                sum_u = sum(n.turing_u for n in neighbors)
                sum_v = sum(n.turing_v for n in neighbors)

                laplacian_u = sum_u - (degree * node.turing_u)
                laplacian_v = sum_v - (degree * node.turing_v)

            node.update_turing_chemicals(laplacian_u, laplacian_v, dt=0.1)

    def step(self):
        self.apply_hebbian_learning()
        self.evaluate_percolation_threshold()
        self.apply_turing_patterns()

        # Kuramoto Model: synchronize nodes' heartbeat phases based on real edge topology
        for (src_id, tgt_id) in self.edges:
            src_node = self.nodes.get(src_id)
            tgt_node = self.nodes.get(tgt_id)
            if src_node and tgt_node:
                # Locally sync phases (if both nodes exist in the simulated orchestrator memory)
                src_node.sync_kuramoto([tgt_node.kuramoto_phase], dt=0.1)
                if not tgt_node.is_remote:
                    tgt_node.sync_kuramoto([src_node.kuramoto_phase], dt=0.1)

                # To make the connection "real", send the actual JSON phase payload across the TCP socket
                sock = self.active_connections.get((src_id, tgt_id))
                if sock:
                    try:
                        payload = json.dumps({
                            "kuramoto_phase": src_node.kuramoto_phase,
                            "turing_chemicals": {"u": src_node.turing_u, "v": src_node.turing_v}
                        }).encode('utf-8')
                        sock.sendall(payload)
                    except Exception:
                        pass

        print(json.dumps({"message": "evaluating connections"}))
        print(json.dumps({"message": "measuring latency"}))

        # Constructal Law: Dynamic calculation of resistance/latency based on real flow
        total_traffic = sum(node.current_traffic for node in self.nodes.values())
        edge_count = len(self.edges)

        # Latency scales inversely with edge count (more parallel paths = lower latency), but scales with traffic
        if edge_count > 0:
            self.latency = (total_traffic / 100.0) * (self.num_nodes / edge_count)
            # Resistance calculates the 'friction'. We lower resistance if connections are highly myelinated.
            avg_myelin = sum(self.synaptic_weights.values()) / len(self.synaptic_weights) if self.synaptic_weights else 1.0
            self.resistance = (total_traffic / 50.0) / avg_myelin
        else:
            self.latency = float('inf')
            self.resistance = float('inf')

        actions = {}
        for node_id, node in self.nodes.items():
            action = node.step()
            if action:
                actions[node_id] = action

        return actions

    def rewire_hub_and_spoke(self):
        print(json.dumps({"message": "re-wiring to hub-and-spoke"}))
        self._disconnect_all()

        hub = "node_0"
        for i in range(1, self.num_nodes):
            spoke = f"node_{i}"
            self._connect(spoke, hub)
            print(json.dumps({"message": f"{spoke} connected to Node {hub}"}))

        self.latency = max(0, self.latency - 2.0)
        self.resistance = max(0, self.resistance - 2.0)
        print(json.dumps({"message": "latency reduced"}))
        print(json.dumps({"message": "resistance minimized"}))

    def rewire_mesh(self):
        print(json.dumps({"message": "re-wiring to mesh"}))
        self._disconnect_all()

        for i in range(self.num_nodes):
            for j in range(i + 1, self.num_nodes):
                src = f"node_{i}"
                tgt = f"node_{j}"
                self._connect(src, tgt)
                print(json.dumps({"message": f"{src} connected to Node {tgt}"}))

        self.latency = max(0, self.latency - 1.0)
        self.resistance = max(0, self.resistance - 1.0)
        print(json.dumps({"message": "latency reduced"}))
        print(json.dumps({"message": "resistance minimized"}))

    def get_node(self, node_id):
        return self.nodes.get(node_id)

    def shutdown(self):
        self._disconnect_all()
        for node in self.nodes.values():
            node.stop()
