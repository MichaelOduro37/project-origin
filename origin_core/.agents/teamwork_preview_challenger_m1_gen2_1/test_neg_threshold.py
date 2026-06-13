import sys
import os

# Add src to path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../../')))

from src.node import Node

def test_negative_thresholds():
    # Node initialized with negative thresholds
    node = Node("n1", expected_traffic=10.0, surprise_threshold=-1.0, surprise_ratio=-0.1)
    
    actions = []
    for _ in range(5):
        # Provide EXACTLY expected traffic
        node.receive_traffic("src1", 10.0)
        action = node.step()
        actions.append(action)
        
    print(f"Actions taken despite exactly matching expected traffic: {actions}")
    
test_negative_thresholds()
