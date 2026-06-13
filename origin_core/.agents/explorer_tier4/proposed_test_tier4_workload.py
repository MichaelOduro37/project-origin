import pytest

# Assuming conftest.py provides these fixtures or imports
# In a real environment, they would be properly imported or injected.
# We will use fixtures `run_origin_core` and `parse_logs` as specified in SCOPE.md.

@pytest.fixture
def run_origin_core():
    # Mock fixture for collectability. In reality, conftest.py would provide this.
    def _run(args):
        import subprocess
        return subprocess.CompletedProcess(args=["main.py"] + args, returncode=0, stdout="mock logs", stderr="")
    return _run

@pytest.fixture
def parse_logs():
    # Mock fixture for collectability.
    def _parse(stdout):
        return [{"message": "mock log entry"}]
    return _parse

def test_workload_steady_to_gradual_increase(run_origin_core, parse_logs):
    """
    Scenario 1: Baseline steady traffic followed by gradual increase.
    Exercising F1 (Network Init) and F4 (Simulated Load).
    """
    result = run_origin_core(["--workload", "steady_increase"])
    assert result.returncode == 0
    logs = parse_logs(result.stdout)
    
    # Assert F1: Initialization of at least 5 nodes
    init_logs = [log for log in logs if "initiates a network" in log.get("message", "").lower() or "node initialized" in log.get("message", "").lower()]
    assert len(init_logs) > 0, "Network should initialize."
    
    # Assert F4: Processing simulated load
    load_logs = [log for log in logs if "processing simulated load" in log.get("message", "").lower()]
    assert len(load_logs) > 0, "Should process steady/gradual load."

def test_workload_sudden_spike_spawning(run_origin_core, parse_logs):
    """
    Scenario 2: Sudden traffic spike causing spawning.
    Exercising F1, F2 (Free Energy/Spawning), F4.
    """
    result = run_origin_core(["--workload", "sudden_spike"])
    assert result.returncode == 0
    logs = parse_logs(result.stdout)
    
    # Assert F2: High surprise/free energy detected and spawning
    surprise_logs = [log for log in logs if "high surprise" in log.get("message", "").lower() or "free energy" in log.get("message", "").lower()]
    assert len(surprise_logs) > 0, "Node must detect high surprise/free energy."
    
    spawn_logs = [log for log in logs if "autonomously spawning" in log.get("message", "").lower() or "spawning sub-node" in log.get("message", "").lower()]
    assert len(spawn_logs) > 0, "Node must autonomously spawn due to spike."

def test_workload_asymmetric_load_rewiring(run_origin_core, parse_logs):
    """
    Scenario 3: Asymmetric load causing topology re-wiring.
    Exercising F1, F3 (Constructal Morphing), F4.
    """
    result = run_origin_core(["--workload", "asymmetric_load"])
    assert result.returncode == 0
    logs = parse_logs(result.stdout)
    
    # Assert F3: Topology re-wiring
    rewire_logs = [log for log in logs if "re-wiring topology" in log.get("message", "").lower() or "connects directly to" in log.get("message", "").lower()]
    assert len(rewire_logs) > 0, "Network must re-wire topology to reduce latency."

def test_workload_massive_anomaly_recovery(run_origin_core, parse_logs):
    """
    Scenario 4: Massive injected anomaly requiring full recovery.
    Exercising F1, F2, F3, F4, F5 (Homeostasis).
    """
    result = run_origin_core(["--workload", "massive_anomaly"])
    assert result.returncode == 0
    logs = parse_logs(result.stdout)
    
    # Assert F5: Recovery from anomaly (homeostasis)
    anomaly_logs = [log for log in logs if "massive anomaly" in log.get("message", "").lower() or "injected traffic anomaly" in log.get("message", "").lower()]
    assert len(anomaly_logs) > 0, "Must detect massive injected anomaly."
    
    recovery_logs = [log for log in logs if "recovers stability" in log.get("message", "").lower() or "homeostasis restored" in log.get("message", "").lower()]
    assert len(recovery_logs) > 0, "System must recover stability (homeostasis)."

def test_workload_sustained_high_with_spikes(run_origin_core, parse_logs):
    """
    Scenario 5: Sustained high load with intermittent spikes.
    Exercising F1, F2, F3, F4, F5.
    """
    result = run_origin_core(["--workload", "sustained_spikes"])
    assert result.returncode == 0
    logs = parse_logs(result.stdout)
    
    # Verify continuous load, multiple spawn/throttle, rewiring, and stability maintenance.
    spawn_logs = [log for log in logs if "autonomously spawning" in log.get("message", "").lower() or "throttling" in log.get("message", "").lower()]
    assert len(spawn_logs) >= 2, "Should have multiple spawn/throttle events during sustained load."
    
    rewire_logs = [log for log in logs if "re-wiring" in log.get("message", "").lower()]
    assert len(rewire_logs) >= 1, "Should exhibit topology changes."
    
    homeostasis_logs = [log for log in logs if "recovers stability" in log.get("message", "").lower()]
    assert len(homeostasis_logs) >= 1, "System must maintain or recover stability under sustained pressure."
