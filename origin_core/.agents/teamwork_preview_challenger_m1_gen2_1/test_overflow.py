import sys
import os

# Add src to path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../../')))

from src.node import Node
from src.load_generator import LoadGenerator

def test_overflow_bug():
    node = Node("n1")
    # 1e308 is not inf yet
    node.receive_traffic("src1", 1e308)
    # adding another 1e308 makes current_traffic inf!
    node.receive_traffic("src2", 1e308)
    
    print(f"Current traffic after additions: {node.current_traffic}")
    
    action = node.step()
    print(f"Action triggered on overflow: {action}")
    print(f"Expected traffic after step: {node.expected_traffic}")
    print(f"Surprise: {node.surprise}")

    # Now let's try another step with normal traffic
    node.receive_traffic("src1", 10.0)
    action2 = node.step()
    print(f"Action on next step: {action2}")
    print(f"Surprise on next step: {node.surprise}")

test_overflow_bug()
