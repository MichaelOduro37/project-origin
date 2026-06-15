import time
import threading
import queue

class PheromoneFabric:
    """Simulates the ambient environment (like Bluetooth LE or local Wi-Fi Direct).
    There are no IP addresses here. It is just a shared medium."""
    def __init__(self):
        self.active_pheromones = []
        self.lock = threading.Lock()

    def drop_pheromone(self, sender_id, target_id, payload, ttl=3):
        with self.lock:
            pheromone = {
                'sender': sender_id,
                'target': target_id,
                'payload': payload,
                'ttl': ttl,
                'timestamp': time.time()
            }
            self.active_pheromones.append(pheromone)
            print(f"[FABRIC] --- Pheromone dropped by {sender_id}. Target: {target_id}")

    def smell(self, node_id):
        with self.lock:
            # Clean up evaporated pheromones
            current_time = time.time()
            self.active_pheromones = [p for p in self.active_pheromones if current_time - p['timestamp'] < p['ttl']]
            return list(self.active_pheromones)

class OriginNode:
    """Simulates the Universal Binary running on a user's phone."""
    def __init__(self, node_id, fabric: PheromoneFabric, is_relay=False):
        self.node_id = node_id
        self.fabric = fabric
        self.is_relay = is_relay
        self.processed_messages = set()
        self.running = True

    def listen(self):
        """Constantly smells the environment for pheromones."""
        while self.running:
            pheromones = self.fabric.smell(self.node_id)
            for p in pheromones:
                msg_id = hash(p['payload'] + str(p['timestamp']))
                if msg_id in self.processed_messages:
                    continue # Already processed this

                if p['target'] == self.node_id:
                    print(f"[{self.node_id}] MATCH! Received payload: '{p['payload']}'")
                    self.processed_messages.add(msg_id)
                elif self.is_relay and p['sender'] != self.node_id:
                    print(f"[{self.node_id}] STRANGER HOP: Smelled packet for {p['target']}. Relaying...")
                    self.processed_messages.add(msg_id)
                    # Re-drop the pheromone to extend its reach (simulating hopping networks)
                    self.fabric.drop_pheromone(self.node_id, p['target'], p['payload'], ttl=3)
            
            time.sleep(0.5)

def run_mesh_simulation():
    fabric = PheromoneFabric()

    # Create 3 Nodes. 
    # Alice is on a blocked University Wi-Fi.
    # Charlie is on the global internet.
    # Bob is a stranger in the library with cellular data (Acting as a relay).
    alice = OriginNode("Alice", fabric)
    bob = OriginNode("Bob (Stranger)", fabric, is_relay=True)
    charlie = OriginNode("Charlie", fabric)

    nodes = [alice, bob, charlie]
    threads = []
    
    print("=== INITIALIZING ORIGIN-COMM SWARM MESH ===")
    print("Scenario: Alice wants to message Charlie. They cannot connect directly.")
    print("Bob's phone is nearby and acts as a silent, ephemeral relay.\n")

    for node in nodes:
        t = threading.Thread(target=node.listen)
        t.start()
        threads.append(t)

    time.sleep(1)

    # Alice drops a message into the ambient fabric. She does not know Bob or Charlie's IP.
    print("[Alice] Action: Sending message to Charlie via Swarm...")
    fabric.drop_pheromone("Alice", "Charlie", "Hello Charlie, we are unblockable.")

    time.sleep(3) # Let the swarm process

    for node in nodes:
        node.running = False
    for t in threads:
        t.join()
        
    print("\n=== SIMULATION COMPLETE ===")

if __name__ == "__main__":
    run_mesh_simulation()
