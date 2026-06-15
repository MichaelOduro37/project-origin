import random
import time

class AIS_ImmuneSystem:
    def __init__(self, self_space, r_chunk_size=3):
        self.self_space = self_space
        self.r_chunk_size = r_chunk_size
        self.detectors = []

    def _generate_candidate(self, length):
        """Generates a random binary string candidate."""
        return ''.join(random.choice(['0', '1']) for _ in range(length))

    def _r_chunk_match(self, string1, string2):
        """Checks if two strings match at any r-contiguous positions."""
        if len(string1) != len(string2):
            return False
        for i in range(len(string1) - self.r_chunk_size + 1):
            if string1[i:i+self.r_chunk_size] == string2[i:i+self.r_chunk_size]:
                return True
        return False

    def train_detectors(self, num_detectors, string_length):
        """Generates detectors in the Non-Self space."""
        print(f"Training {num_detectors} detectors (Negative Selection)...")
        while len(self.detectors) < num_detectors:
            candidate = self._generate_candidate(string_length)
            is_self_reactive = False
            for self_string in self.self_space:
                if self._r_chunk_match(candidate, self_string):
                    is_self_reactive = True
                    break
            
            if not is_self_reactive:
                self.detectors.append(candidate)
        print(f"Successfully generated {len(self.detectors)} detectors in Non-Self space.")

    def monitor_traffic(self, traffic_string):
        """Monitors incoming traffic using mature detectors."""
        for detector in self.detectors:
            if self._r_chunk_match(traffic_string, detector):
                return True # Anomaly detected (matches Non-Self detector)
        return False # Normal traffic (matches no detectors, assumed Self)

def run_poc():
    # 1. Define the "Self" (S) space: deterministic baseline of normal traffic
    # Representing normal network states as binary strings (e.g., specific protocol bitmasks)
    self_space = [
        "10101010",
        "11001100",
        "00110011",
        "11110000"
    ]
    
    print("=== Origin AI Immune System: Negative Selection PoC ===")
    print("Defined Self Space (S):", self_space)
    
    # Initialize Immune System
    immune_system = AIS_ImmuneSystem(self_space, r_chunk_size=4)
    
    # 2. Generate r-chunk detectors in the "Non-Self" (U \ S) space
    start_time = time.time()
    immune_system.train_detectors(num_detectors=50, string_length=8)
    print(f"Training took {time.time() - start_time:.4f} seconds.")
    
    # 3. Test monitoring (Polynomial-time anomaly detection)
    test_traffic = [
        ("10101010", "Known Self (Normal)"),
        ("11001100", "Known Self (Normal)"),
        ("10111010", "Slight Anomaly (Mutated)"),
        ("00001111", "Complete Anomaly (Unknown)")
    ]
    
    print("\n--- Monitoring Network Traffic ---")
    for traffic, description in test_traffic:
        is_anomaly = immune_system.monitor_traffic(traffic)
        status = "QUARANTINED (Anomaly)" if is_anomaly else "ALLOWED (Normal)"
        print(f"Traffic: {traffic} | Type: {description.ljust(30)} | Result: {status}")

if __name__ == "__main__":
    run_poc()
