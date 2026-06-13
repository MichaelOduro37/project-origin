import pytest
from src.node import Node
from src.load_generator import LoadGenerator
import math

def test_node_negative_threshold():
    node = Node("n1", expected_traffic=10.0, surprise_threshold=-5.0, surprise_ratio=-0.1)
    node.receive_traffic("src1", 10.0)
    action = node.step()
    print(f"Negative threshold test: action={action}")

def test_load_generator_negative_fluctuation():
    gen = LoadGenerator(base_traffic=0.0, variance=10.0, anomaly_prob=1.0, anomaly_multiplier=-5.0)
    import random
    original_gauss = random.gauss
    random.gauss = lambda mu, sigma: -2.0
    try:
        traffic = gen.generate()
        print(f"Negative fluctuation with negative multiplier test: traffic={traffic}")
    finally:
        random.gauss = original_gauss

if __name__ == "__main__":
    test_node_negative_threshold()
    test_load_generator_negative_fluctuation()
