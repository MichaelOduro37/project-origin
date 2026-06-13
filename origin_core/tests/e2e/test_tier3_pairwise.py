import pytest
from conftest import run_origin_core, parse_logs

def test_f1_f2_initialization_and_energy_minimization():
    """
    Test interaction between Network Initialization (F1) and Free Energy Minimization (F2).
    Initializes the network and triggers a sudden traffic change to cause node spawning.
    """
    result = run_origin_core(["--scenario", "init_and_spawn"])
    assert result.returncode == 0, f"Process failed with stderr: {result.stderr}"
    
    logs = parse_logs(result.stdout)
    
    # Assert F1: Network initialized with >= 5 nodes
    init_logs = [log for log in logs if log.get("event") == "network_initialized"]
    assert len(init_logs) > 0, "Network initialization log not found"
    assert init_logs[0].get("node_count", 0) >= 5, "Network must have at least 5 nodes"
    
    # Assert F2: High surprise / Free Energy minimization (spawning)
    surprise_logs = [log for log in logs if log.get("event") == "high_surprise_detected"]
    assert len(surprise_logs) > 0, "High surprise (free energy) not detected"
    
    spawn_logs = [log for log in logs if log.get("action") in ("spawn_node", "throttle_connection")]
    assert len(spawn_logs) > 0, "No spawn or throttle action taken to minimize free energy"

def test_f1_f3_initialization_and_topology_morphing():
    """
    Test interaction between Network Initialization (F1) and Topology Morphing (F3).
    Initializes the network and applies asymmetric load to trigger re-wiring.
    """
    result = run_origin_core(["--scenario", "init_and_rewire"])
    assert result.returncode == 0, f"Process failed with stderr: {result.stderr}"
    
    logs = parse_logs(result.stdout)
    
    # Assert F1: Network initialized
    init_logs = [log for log in logs if log.get("event") == "network_initialized"]
    assert len(init_logs) > 0, "Network initialization log not found"
    
    # Assert F3: Topology morphing / re-wiring
    rewire_logs = [log for log in logs if log.get("event") == "topology_rewire"]
    assert len(rewire_logs) > 0, "Topology re-wiring log not found"
    assert "source_node" in rewire_logs[0] and "target_node" in rewire_logs[0], "Re-wiring log missing node details"

def test_f2_f4_energy_minimization_and_load_processing():
    """
    Test interaction between Free Energy Minimization (F2) and Simulated Load Processing (F4).
    Ensures that processing simulated load triggers free energy minimization actions.
    """
    result = run_origin_core(["--scenario", "load_and_minimize"])
    assert result.returncode == 0, f"Process failed with stderr: {result.stderr}"
    
    logs = parse_logs(result.stdout)
    
    # Assert F4: Simulated load processing
    load_logs = [log for log in logs if log.get("event") == "load_processed"]
    assert len(load_logs) > 0, "Simulated load processing log not found"
    
    # Assert F2: Free energy minimization
    minimize_logs = [log for log in logs if log.get("event") == "free_energy_minimized"]
    assert len(minimize_logs) > 0, "Free energy minimization log not found"

def test_f3_f4_topology_morphing_and_load_processing():
    """
    Test interaction between Topology Morphing (F3) and Simulated Load Processing (F4).
    Ensures that processing specific simulated loads triggers network morphing.
    """
    result = run_origin_core(["--scenario", "load_and_rewire"])
    assert result.returncode == 0, f"Process failed with stderr: {result.stderr}"
    
    logs = parse_logs(result.stdout)
    
    # Assert F4: Simulated load processing
    load_logs = [log for log in logs if log.get("event") == "load_processed"]
    assert len(load_logs) > 0, "Simulated load processing log not found"
    
    # Assert F3: Topology morphing / re-wiring
    rewire_logs = [log for log in logs if log.get("event") == "topology_rewire"]
    assert len(rewire_logs) > 0, "Topology re-wiring log not found"

def test_f2_f5_energy_minimization_and_anomaly_recovery():
    """
    Test interaction between Free Energy Minimization (F2) and Anomaly Recovery/Homeostasis (F5).
    Injects a massive anomaly and verifies that energy minimization actions lead to recovery.
    """
    result = run_origin_core(["--scenario", "anomaly_recovery"])
    assert result.returncode == 0, f"Process failed with stderr: {result.stderr}"
    
    logs = parse_logs(result.stdout)
    
    # Assert F5: Massive anomaly injected
    anomaly_logs = [log for log in logs if log.get("event") == "anomaly_injected"]
    assert len(anomaly_logs) > 0, "Anomaly injection log not found"
    
    # Assert F2: Free energy minimization during anomaly
    spawn_logs = [log for log in logs if log.get("action") in ("spawn_node", "throttle_connection")]
    assert len(spawn_logs) > 0, "No spawn or throttle action taken during anomaly"
    
    # Assert F5: Homeostasis / Recovery achieved
    recovery_logs = [log for log in logs if log.get("event") == "homeostasis_achieved"]
    assert len(recovery_logs) > 0, "Homeostasis/recovery log not found"
