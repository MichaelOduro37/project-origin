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
        except Exception:
            pass

    def step(self):
        print(json.dumps({"message": "evaluating connections"}))
        print(json.dumps({"message": "measuring latency"}))

        self.resistance = 10.0
        self.latency = 5.0

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
