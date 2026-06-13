import pytest
from src.node import Node
from src.load_generator import LoadGenerator

def test_node_negative_traffic():
    node = Node("n1")
    node.receive_traffic("evil", -50.0)
    assert node.current_traffic == -50.0 # It accepts negative traffic!
    action = node.step()
    assert action == "throttle"
    assert node.expected_traffic == -10.0 # EMA becomes negative!

def test_generator_negative_multiplier():
    gen = LoadGenerator(base_traffic=10.0, variance=0.0, anomaly_prob=1.0, anomaly_multiplier=-5.0)
    traffic = gen.generate()
    assert traffic == -50.0 # Generates negative traffic!
