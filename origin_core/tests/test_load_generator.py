import pytest
from src.load_generator import LoadGenerator

def test_load_generator_initialization():
    gen = LoadGenerator(base_traffic=15.0, variance=1.0, anomaly_prob=0.1, anomaly_multiplier=4.0)
    assert gen.base_traffic == 15.0
    assert gen.variance == 1.0
    assert gen.anomaly_prob == 0.1
    assert gen.anomaly_multiplier == 4.0

def test_load_generator_initialization_invalid():
    import math
    with pytest.raises(ValueError):
        LoadGenerator(variance=-1.0)
    with pytest.raises(ValueError):
        LoadGenerator(anomaly_prob=1.5)
    with pytest.raises(ValueError):
        LoadGenerator(anomaly_prob=-0.1)
    with pytest.raises(ValueError):
        LoadGenerator(base_traffic=math.nan)
    with pytest.raises(ValueError):
        LoadGenerator(anomaly_multiplier=math.inf)

def test_generate_baseline():
    gen = LoadGenerator(base_traffic=10.0, variance=0.0, anomaly_prob=0.0)
    traffic = gen.generate()
    # With 0 variance and 0 anomaly prob, should be exactly base_traffic
    assert traffic == 10.0

def test_generate_no_negative_traffic():
    gen = LoadGenerator(base_traffic=0.0, variance=100.0, anomaly_prob=0.0)
    for _ in range(100):
        traffic = gen.generate()
        assert traffic >= 0.0

def test_generate_deterministic_normal():
    gen = LoadGenerator(base_traffic=10.0, anomaly_multiplier=5.0)
    traffic = gen.generate_deterministic(step=1, anomaly_step=5)
    assert traffic == 10.0

def test_generate_deterministic_anomaly():
    gen = LoadGenerator(base_traffic=10.0, anomaly_multiplier=5.0)
    traffic = gen.generate_deterministic(step=5, anomaly_step=5)
    assert traffic == 50.0

def test_generate_negative_base_and_multiplier():
    # Negative base, positive multiplier
    gen = LoadGenerator(base_traffic=-10.0, variance=0.0, anomaly_prob=1.0, anomaly_multiplier=5.0)
    assert gen.generate() == 0.0
    
    # Positive base, negative multiplier
    gen2 = LoadGenerator(base_traffic=10.0, variance=0.0, anomaly_prob=1.0, anomaly_multiplier=-5.0)
    assert gen2.generate() == 0.0

    # Negative base, negative multiplier
    gen3 = LoadGenerator(base_traffic=-10.0, variance=0.0, anomaly_prob=1.0, anomaly_multiplier=-5.0)
    assert gen3.generate() == 0.0

def test_generate_deterministic_negative_multiplier():
    gen = LoadGenerator(base_traffic=10.0, anomaly_multiplier=-5.0)
    traffic = gen.generate_deterministic(step=1, anomaly_step=1)
    # 10.0 * -5.0 = -50.0 -> max(0.0, -50.0) = 0.0
    assert traffic == 0.0
