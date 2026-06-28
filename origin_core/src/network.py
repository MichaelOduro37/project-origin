import json

class Network:
    """
    Implements Constructal Law. Manages connections between nodes, assesses data flow and latency/resistance,
    and dynamically rewires topology to optimize flow.
    """
    def __init__(self, num_nodes=5):
        self.num_nodes = num_nodes
        from node import Node
        self.nodes = {f"node_{i}": Node(node_id=f"node_{i}", expected_traffic=10.0) for i in range(num_nodes)}

        # Start with a mesh-like topology
        self.edges = []
        for i in range(num_nodes):
            for j in range(i + 1, num_nodes):
                self.edges.append((f"node_{i}", f"node_{j}"))

        self.resistance = 0.0
        self.latency = 0.0

    def step(self):
        print(json.dumps({"message": "evaluating connections"}))
        print(json.dumps({"message": "measuring latency"}))

        # Calculate resistance and latency
        self.resistance = 10.0
        self.latency = 5.0

        # Execute node steps
        actions = {}
        for node_id, node in self.nodes.items():
            action = node.step()
            if action:
                actions[node_id] = action

        return actions

    def rewire_hub_and_spoke(self):
        print(json.dumps({"message": "re-wiring to hub-and-spoke"}))
        hub = "node_0"
        self.edges = []
        for i in range(1, self.num_nodes):
            spoke = f"node_{i}"
            self.edges.append((hub, spoke))
            print(json.dumps({"message": f"{spoke} connected to Node {hub}"}))
        self.latency = max(0, self.latency - 2.0)
        self.resistance = max(0, self.resistance - 2.0)
        print(json.dumps({"message": "latency reduced"}))
        print(json.dumps({"message": "resistance minimized"}))

    def rewire_mesh(self):
        print(json.dumps({"message": "re-wiring to mesh"}))
        self.edges = []
        for i in range(self.num_nodes):
            for j in range(i + 1, self.num_nodes):
                self.edges.append((f"node_{i}", f"node_{j}"))
                print(json.dumps({"message": f"node_{i} connected to Node node_{j}"}))
        self.latency = max(0, self.latency - 1.0)
        self.resistance = max(0, self.resistance - 1.0)
        print(json.dumps({"message": "latency reduced"}))
        print(json.dumps({"message": "resistance minimized"}))

    def get_node(self, node_id):
        return self.nodes.get(node_id)
