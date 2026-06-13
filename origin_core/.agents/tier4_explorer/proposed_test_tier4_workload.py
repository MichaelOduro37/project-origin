import pytest
from tests.e2e.conftest import run_origin_core, parse_logs

def test_workload_baseline_increase():
    """Scenario 1: Baseline steady traffic followed by gradual increase.
    Features Exercised: F1, F4
    """
    process = run_origin_core(["--workload", "baseline_increase"])
    assert process.returncode == 0, f"Process failed with stdout: {process.stdout} stderr: {process.stderr}"
    logs = parse_logs(process.stdout)
    
    # F1: Network Initialization
    assert any(log.get("event") == "network_initialized" and log.get("node_count", 0) >= 5 for log in logs), "Expected network initialization with >= 5 nodes"
    
    # F4: Simulated Load Processing
    assert any(log.get("event") == "load_processed" for log in logs), "Expected baseline load processing"

def test_workload_sudden_spike():
    """Scenario 2: Sudden traffic spike causing spawning.
    Features Exercised: F1, F2, F4
    """
    process = run_origin_core(["--workload", "sudden_spike"])
    assert process.returncode == 0
    logs = parse_logs(process.stdout)
    
    # F2: Free Energy Minimization (Spawn/Throttle)
    assert any(log.get("event") == "high_surprise_detected" for log in logs), "Expected high surprise / free energy detection"
    assert any(log.get("event") in ["node_spawned", "connection_throttled"] for log in logs), "Expected autonomous spawning or throttling"

def test_workload_asymmetric_load():
    """Scenario 3: Asymmetric load causing topology re-wiring.
    Features Exercised: F1, F3, F4
    """
    process = run_origin_core(["--workload", "asymmetric_load"])
    assert process.returncode == 0
    logs = parse_logs(process.stdout)
    
    # F3: Constructal Topology Morphing
    assert any(log.get("event") == "topology_rewired" for log in logs), "Expected topology re-wiring"

def test_workload_massive_anomaly():
    """Scenario 4: Massive injected anomaly requiring full recovery.
    Features Exercised: F1, F2, F3, F4, F5
    """
    process = run_origin_core(["--workload", "massive_anomaly"])
    assert process.returncode == 0
    logs = parse_logs(process.stdout)
    
    # F5: Homeostasis / Recovery from Anomaly
    assert any(log.get("event") == "homeostasis_recovered" for log in logs), "Expected recovery / homeostasis achieved"

def test_workload_sustained_spikes():
    """Scenario 5: Sustained high load with intermittent spikes.
    Features Exercised: F1, F2, F3, F4, F5
    """
    process = run_origin_core(["--workload", "sustained_spikes"])
    assert process.returncode == 0
    logs = parse_logs(process.stdout)
    
    # Check for multiple spawn/throttle and re-wiring events indicating sustained effort
    spawn_events = [log for log in logs if log.get("event") == "node_spawned"]
    throttle_events = [log for log in logs if log.get("event") == "connection_throttled"]
    rewire_events = [log for log in logs if log.get("event") == "topology_rewired"]
    
    assert (len(spawn_events) + len(throttle_events)) >= 2, "Expected multiple spawning/throttling events"
    assert len(rewire_events) >= 1, "Expected at least one re-wiring event"
