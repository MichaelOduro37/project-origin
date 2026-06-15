import random
import time

class ArtificialDendriticCell:
    def __init__(self, migration_threshold=15):
        self.migration_threshold = migration_threshold
        self.csm = 0.0          # Costimulatory Molecules (Migration signal)
        self.semi_mature = 0.0  # Context: Safe
        self.mature = 0.0       # Context: Danger
        self.antigens = []      # Data fragments collected

    def sample_environment(self, pamp, danger, safe, antigen):
        """
        Calculates the sensor fusion weights according to the Deterministic DCA (dDCA) mathematical model.
        csm measures when the cell is full.
        semi_mature increases with Safe signals.
        mature increases with PAMP and Danger signals.
        """
        # Multi-sensor data fusion weights (standard DCA matrix)
        delta_csm = (pamp * 2) + (danger * 1) + (safe * 2)
        delta_semi_mature = (pamp * 0) + (danger * 0) + (safe * 3)
        delta_mature = (pamp * 2) + (danger * 1) + (safe * -2)

        self.csm += delta_csm
        self.semi_mature += delta_semi_mature
        self.mature += delta_mature
        self.antigens.append(antigen)

    def is_mature(self):
        return self.csm >= self.migration_threshold

    def get_context(self):
        # 1 means Anomalous (Mature > Semi-Mature), 0 means Normal
        return 1 if self.mature > self.semi_mature else 0


class dDCA_Network_Monitor:
    def __init__(self):
        self.cell_pool = []
        self.matured_cells = []
        
    def spawn_cell(self):
        self.cell_pool.append(ArtificialDendriticCell(migration_threshold=random.uniform(10, 20)))

    def process_network_slice(self, network_state):
        """
        network_state dict containing:
        - pamp: e.g., Signature match from Negative Selection (0 to 1)
        - danger: e.g., CPU thermal spike, memory leak (0 to 1)
        - safe: e.g., verified cryptographic heartbeat (0 to 1)
        - payload_id: The ID of the packet/request
        """
        # Ensure we always have sampling cells
        if len(self.cell_pool) < 5:
            for _ in range(5 - len(self.cell_pool)):
                self.spawn_cell()

        for cell in self.cell_pool:
            cell.sample_environment(
                pamp=network_state["pamp"],
                danger=network_state["danger"],
                safe=network_state["safe"],
                antigen=network_state["payload_id"]
            )

        # Migrate matured cells
        active_cells = []
        for cell in self.cell_pool:
            if cell.is_mature():
                self.matured_cells.append(cell)
            else:
                active_cells.append(cell)
        self.cell_pool = active_cells

    def analyze_antigen(self, antigen_id):
        # Calculate Mean Antigen Context (K_alpha)
        anomalous_presentations = 0
        total_presentations = 0
        
        for cell in self.matured_cells:
            if antigen_id in cell.antigens:
                total_presentations += 1
                anomalous_presentations += cell.get_context()
                
        if total_presentations == 0:
            return 0.0
            
        k_alpha = anomalous_presentations / total_presentations
        return k_alpha


def run_proof_of_concept():
    print("[*] Project Origin - dDCA Artificial Immune System PoC")
    print("[*] Initiating polynomial-time edge anomaly detection...\n")
    
    dca = dDCA_Network_Monitor()
    
    print("--- Phase 1: Normal Traffic Baseline ---")
    for i in range(10):
        # High safe signals, low danger/pamp
        state = {"pamp": 0.0, "danger": random.uniform(0.0, 0.2), "safe": 0.9, "payload_id": "normal_stream_A"}
        dca.process_network_slice(state)
        time.sleep(0.05)
        
    k_alpha_normal = dca.analyze_antigen("normal_stream_A")
    print(f"[+] 'normal_stream_A' Mean Antigen Context (K_alpha): {k_alpha_normal:.2f} -> {'ANOMALY' if k_alpha_normal > 0.5 else 'SAFE'}\n")

    print("--- Phase 2: DDoS / Exploitation Attempt ---")
    for i in range(15):
        # High danger (CPU spikes), occasional PAMP (Negative Selection match), low safe
        state = {"pamp": random.uniform(0.5, 1.0), "danger": 0.8, "safe": 0.1, "payload_id": "malicious_stream_B"}
        dca.process_network_slice(state)
        time.sleep(0.05)
        
    k_alpha_malicious = dca.analyze_antigen("malicious_stream_B")
    print(f"[!] 'malicious_stream_B' Mean Antigen Context (K_alpha): {k_alpha_malicious:.2f} -> {'ANOMALY (QUARANTINE TRIGGERED)' if k_alpha_malicious > 0.5 else 'SAFE'}\n")

if __name__ == "__main__":
    run_proof_of_concept()
