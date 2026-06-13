import pytest
from src.node import Node

def test_node_initialization():
    node = Node("n1", expected_traffic=5.0)
    assert node.node_id == "n1"
    assert node.expected_traffic == 5.0
    assert node.current_traffic == 0.0
    assert node.surprise == 0.0

def test_node_initialization_invalid_values():
    import math
    with pytest.raises(ValueError):
        Node("n1", expected_traffic=math.nan)
    with pytest.raises(ValueError):
        Node("n1", expected_traffic=math.inf)
    with pytest.raises(ValueError):
        Node("n1", surprise_threshold=-1.0)
    with pytest.raises(ValueError):
        Node("n1", surprise_ratio=-0.5)
    with pytest.raises(ValueError):
        Node("n1", surprise_threshold=math.nan)
    with pytest.raises(ValueError):
        Node("n1", surprise_ratio=math.inf)

def test_receive_traffic():
    node = Node("n1")
    node.receive_traffic("src_1", 10.0)
    node.receive_traffic("src_2", 5.0)
    assert node.current_traffic == 15.0

def test_step_updates_surprise_and_expected_traffic():
    node = Node("n1", expected_traffic=10.0)
    node.receive_traffic("src_1", 12.0)
    
    action = node.step()
    
    # Surprise = |12.0 - 10.0| = 2.0
    assert node.get_surprise() == 2.0
    
    # No action since surprise < 10.0
    assert action is None
    
    # Expected traffic updated: alpha=0.2
    # new_expected = 0.2 * 12.0 + 0.8 * 10.0 = 2.4 + 8.0 = 10.4
    assert abs(node.expected_traffic - 10.4) < 1e-6
    
    # Current traffic resets
    assert node.current_traffic == 0.0

def test_step_action_spawn():
    node = Node("n1", expected_traffic=10.0)
    # Huge spike in traffic
    node.receive_traffic("src_1", 25.0)
    
    action = node.step()
    
    # Surprise = |25.0 - 10.0| = 15.0
    assert node.get_surprise() == 15.0
    
    # surprise > 10 and current > expected -> spawn
    assert action == "spawn"

def test_step_action_throttle():
    node = Node("n1", expected_traffic=20.0)
    # Massive drop in traffic
    node.receive_traffic("src_1", 5.0)
    
    action = node.step()
    
    # Surprise = |5.0 - 20.0| = 15.0
    assert node.get_surprise() == 15.0
    
    # surprise > 10 and current < expected -> throttle
    assert action == "throttle"

def test_receive_traffic_invalid_values():
    import math
    node = Node("n1")
    with pytest.raises(ValueError):
        node.receive_traffic("src_1", -5.0)
    with pytest.raises(ValueError):
        node.receive_traffic("src_1", math.nan)
    with pytest.raises(ValueError):
        node.receive_traffic("src_1", math.inf)

def test_receive_traffic_overflow():
    node = Node("n1")
    node.receive_traffic("src_1", 1e308)
    with pytest.raises(OverflowError):
        node.receive_traffic("src_1", 1e308)

def test_step_dynamic_threshold_scaling():
    node = Node("n1", expected_traffic=10000.0, surprise_threshold=10.0, surprise_ratio=0.1)
    # Threshold should be max(10.0, 10000.0 * 0.1) = 1000.0
    # Receiving 10050.0 traffic -> surprise = 50.0
    # 50.0 < 1000.0, so no action should occur
    node.receive_traffic("src_1", 10050.0)
    action = node.step()
    assert action is None
    assert node.get_surprise() == 50.0
