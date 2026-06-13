import sys
import os

# Add origin_core to path so we can import src
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

from src.node import Node
from src.load_generator import LoadGenerator

def run_stress_tests():
    failures = []

    print("--- Adversarial Stress Test: src/node.py ---")
    
    # Challenge 1: The surprise threshold is fixed at 10.0.
    # At massive scale (expected=10000), a tiny relative change (0.15% = 15) triggers a spawn/throttle, 
    # making the node hyper-reactive to noise at scale.
    print("Testing Node scale insensitivity (fixed absolute threshold)...")
    node_scale = Node("n_scale", expected_traffic=10000.0)
    # A 0.15% change! In real traffic, this is tiny noise.
    node_scale.receive_traffic("src", 10011.0) 
    action = node_scale.step()
    if action is not None:
        print(f"[FAIL] Node triggers {action} on a 0.11% traffic fluctuation because threshold is absolute (10.0). Surprise: {node_scale.get_surprise()}")
        failures.append("Node: Scale Insensitivity (Absolute Threshold)")

    # Challenge 2: Negative traffic inputs
    print("Testing Node with negative traffic...")
    node_neg = Node("n_neg", expected_traffic=5.0)
    node_neg.receive_traffic("src", -20.0) 
    action = node_neg.step()
    if action == "throttle":
        print(f"[FAIL] Node allows negative traffic and throttles based on it! Evaluated against -20.0.")
        failures.append("Node: Negative Traffic Vulnerability")

    print("\n--- Adversarial Stress Test: src/load_generator.py ---")
    
    # Challenge 3: Negative base_traffic in deterministic generation
    print("Testing LoadGenerator deterministic with negative base...")
    gen_det_neg = LoadGenerator(base_traffic=-10.0, variance=0.0, anomaly_prob=0.0)
    val = gen_det_neg.generate_deterministic(1)
    if val < 0:
        print(f"[FAIL] LoadGenerator deterministic allows negative traffic! Returned {val}")
        failures.append("LoadGenerator: Deterministic Negative Traffic")

    # Challenge 4: Negative anomaly_multiplier in regular generation
    print("Testing LoadGenerator generate with negative anomaly multiplier...")
    gen_rand_neg = LoadGenerator(base_traffic=10.0, variance=0.0, anomaly_prob=1.0, anomaly_multiplier=-5.0)
    val = gen_rand_neg.generate()
    if val < 0:
        print(f"[FAIL] LoadGenerator generate allows negative traffic on anomaly! Returned {val}")
        failures.append("LoadGenerator: Random Negative Traffic on Anomaly")

    if not failures:
        print("\nAll stress tests passed!")
        sys.exit(0)
    else:
        print(f"\nFailed tests: {failures}")
        sys.exit(1)

if __name__ == '__main__':
    run_stress_tests()
