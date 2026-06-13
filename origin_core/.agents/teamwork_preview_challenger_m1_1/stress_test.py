import sys
import os
import math

# Add the project root to sys.path so we can import src
project_root = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../'))
if project_root not in sys.path:
    sys.path.append(project_root)

from src.node import Node
from src.load_generator import LoadGenerator

def run_stress_tests():
    print("--- Testing Node ---")
    
    # 1. Negative traffic accumulation
    node1 = Node("n1", expected_traffic=10.0)
    node1.receive_traffic("src1", -5.0)
    action = node1.step()
    print(f"Negative traffic: surprise={node1.surprise}, action={action}, new_expected={node1.expected_traffic}")
    
    # 2. NaN traffic
    node2 = Node("n2", expected_traffic=10.0)
    node2.receive_traffic("src1", float('nan'))
    action = node2.step()
    print(f"NaN traffic: surprise={node2.surprise}, action={action}, new_expected={node2.expected_traffic}")

    # 3. Infinity traffic
    node3 = Node("n3", expected_traffic=10.0)
    node3.receive_traffic("src1", float('inf'))
    action = node3.step()
    print(f"Infinity traffic: surprise={node3.surprise}, action={action}, new_expected={node3.expected_traffic}")

    print("\n--- Testing Load Generator ---")
    
    # 4. Negative configuration parameters
    gen1 = LoadGenerator(base_traffic=-20.0, variance=-5.0, anomaly_prob=1.5, anomaly_multiplier=-10.0)
    t = gen1.generate()
    print(f"Negative params generate(): {t}")
    
    # 5. Deterministic unbounded
    t_det = gen1.generate_deterministic(step=1, anomaly_step=2)
    print(f"Negative params generate_deterministic(no anomaly): {t_det}")
    t_det_anomaly = gen1.generate_deterministic(step=2, anomaly_step=2)
    print(f"Negative params generate_deterministic(anomaly): {t_det_anomaly}")

if __name__ == "__main__":
    run_stress_tests()
