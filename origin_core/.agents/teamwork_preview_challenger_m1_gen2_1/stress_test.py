import sys
import os
import math
import random

# Add src to path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../../')))

from src.node import Node
from src.load_generator import LoadGenerator

def run_stress_tests():
    print("--- Running Adversarial Stress Tests ---")
    
    test_node_overflow_bricking()
    test_node_negative_threshold_throttle_loop()
    test_node_init_validation()
    test_loadgen_negative_anomaly_flip()
    test_loadgen_prob_out_of_bounds()
    
    print("--- Stress Tests Complete ---")

def test_node_overflow_bricking():
    print("\n[Test] Node float overflow bricking")
    node = Node("n1")
    # Simulate a massive DDoS or compounding traffic that exceeds float max
    # 1e308 + 1e308 = inf in Python
    try:
        node.receive_traffic("src1", 1e308)
        node.receive_traffic("src2", 1e308)
    except Exception as e:
        print(f"  FAIL: Unhandled exception on massive traffic: {e}")
        return

    if math.isinf(node.current_traffic):
        print("  WARN: current_traffic overflowed to inf")
        
    action1 = node.step()
    # Now expected_traffic is inf.
    node.receive_traffic("src1", 10.0)
    action2 = node.step()
    
    if math.isnan(node.surprise) or math.isinf(node.surprise):
        print(f"  VULNERABILITY CONFIRMED: Node bricked. Surprise is {node.surprise}, subsequent action: {action2}")
    else:
        print("  PASS: Node handled overflow gracefully.")

def test_node_negative_threshold_throttle_loop():
    print("\n[Test] Node negative threshold exact-match bug")
    # If parameters are unvalidated and negative
    node = Node("n1", expected_traffic=10.0, surprise_threshold=-1.0, surprise_ratio=-0.1)
    
    node.receive_traffic("src1", 10.0) # exact match!
    action = node.step()
    
    if action == "throttle":
        print("  VULNERABILITY CONFIRMED: Node throttles despite exact traffic match due to lack of threshold lower-bounding and unhandled equality condition.")
    else:
        print(f"  PASS: Node behaved correctly. Action: {action}")

def test_node_init_validation():
    print("\n[Test] Node __init__ parameter validation (NaN/Inf)")
    try:
        node = Node("n1", expected_traffic=float('nan'))
        # If it doesn't raise ValueError, it's vulnerable to silent corruption
        print("  VULNERABILITY CONFIRMED: Node allows initialization with NaN expected_traffic.")
    except ValueError:
        print("  PASS: Node rejects NaN on init.")

def test_loadgen_negative_anomaly_flip():
    print("\n[Test] LoadGenerator negative multiplier sign flip")
    # A negative multiplier is intended to cause traffic drops (to 0).
    # But if base is 0 and variance is high, random.gauss goes negative.
    # negative * negative = positive -> generates positive traffic instead of dropping!
    random.seed(42)
    gen = LoadGenerator(base_traffic=0.0, variance=10.0, anomaly_prob=1.0, anomaly_multiplier=-5.0)
    
    anomalous_positives = 0
    for _ in range(100):
        t = gen.generate()
        if t > 0:
            anomalous_positives += 1
            
    if anomalous_positives > 0:
        print(f"  VULNERABILITY CONFIRMED: Negative anomaly multiplier generated {anomalous_positives}/100 positive traffic spikes!")
    else:
        print("  PASS: Negative anomaly multiplier behaves as expected.")

def test_loadgen_prob_out_of_bounds():
    print("\n[Test] LoadGenerator anomaly_prob bounds checking")
    gen = LoadGenerator(anomaly_prob=5.0) # > 1.0
    # No error raised.
    print("  VULNERABILITY CONFIRMED: LoadGenerator does not validate anomaly_prob domain [0, 1].")

if __name__ == "__main__":
    run_stress_tests()
