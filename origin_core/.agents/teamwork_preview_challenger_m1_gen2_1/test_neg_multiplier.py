import sys
import os

# Add src to path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../../')))

from src.load_generator import LoadGenerator
import random

def test_negative_multiplier():
    # If base_traffic is close to 0 and variance is high, random.gauss will often return negative
    # If anomaly multiplier is negative, negative * negative = positive!
    # So max(0.0, traffic) will return a large positive value, which is NOT an intended behavior 
    # of a "negative anomaly".
    
    random.seed(42)  # ensure reproducibility
    gen = LoadGenerator(base_traffic=0.0, variance=10.0, anomaly_prob=1.0, anomaly_multiplier=-5.0)
    
    positives = []
    for _ in range(10):
        t = gen.generate()
        if t > 0:
            positives.append(t)
            
    print(f"Traffic generated (should be 0 since multiplier is negative and base is 0): {positives}")

test_negative_multiplier()
