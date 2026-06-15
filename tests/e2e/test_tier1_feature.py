import pytest

# F1 (Network Init)
def test_f1_network_starts_with_minimum_5_nodes(run_origin_core, parse_logs):
    result = run_origin_core(["--init"])
    logs = parse_logs(result.stdout)
    init_count = sum(1 for log in logs if "Node initialized" in str(log.get("message", log)))
    assert init_count >= 5

def test_f1_node_markov_blanket_established(run_origin_core, parse_logs):
    result = run_origin_core(["--init"])
    logs = parse_logs(result.stdout)
    assert any("Markov blanket defined" in str(log.get("message", log)) for log in logs)

def test_f1_initial_generative_model_created(run_origin_core, parse_logs):
    result = run_origin_core(["--init"])
    logs = parse_logs(result.stdout)
    assert any("Generative model started" in str(log.get("message", log)) for log in logs)

def test_f1_network_initialization_completes(run_origin_core, parse_logs):
    result = run_origin_core(["--init"])
    logs = parse_logs(result.stdout)
    assert any("Network Ready" in str(log.get("message", log)) for log in logs)

def test_f1_no_errors_on_startup(run_origin_core, parse_logs):
    result = run_origin_core(["--init"])
    assert result.returncode == 0

# F2 (Free Energy)
def test_f2_detects_high_free_energy(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=spike"])
    logs = parse_logs(result.stdout)
    assert any("high surprise" in str(log.get("message", log)) or "free energy spike" in str(log.get("message", log)) for log in logs)

def test_f2_autonomously_spawns_subnode(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=spike"]) # assuming spike or drop
    logs = parse_logs(result.stdout)
    assert any("spawning sub-node" in str(log.get("message", log)) for log in logs)

def test_f2_autonomously_throttles_connection(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=drop"])
    logs = parse_logs(result.stdout)
    assert any("throttling connection" in str(log.get("message", log)) for log in logs)

def test_f2_free_energy_returns_to_baseline(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=spike"])
    logs = parse_logs(result.stdout)
    assert any("surprise levels drop" in str(log.get("message", log)) for log in logs)

def test_f2_independent_action_no_central_controller(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=spike"])
    logs = parse_logs(result.stdout)
    action_logs = [log for log in logs if "action" in str(log.get("message", log)).lower()]
    assert len(action_logs) > 0
    assert not any("Controller" in str(log.get("message", log)) for log in action_logs)

# F3 (Morphing)
def test_f3_evaluates_topology_periodically(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=morph"])
    logs = parse_logs(result.stdout)
    assert any("evaluating connections" in str(log.get("message", log)) or "measuring latency" in str(log.get("message", log)) for log in logs)

def test_f3_rewires_hub_and_spoke(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=central_load"])
    logs = parse_logs(result.stdout)
    assert any("re-wiring to hub-and-spoke" in str(log.get("message", log)) for log in logs)

def test_f3_rewires_mesh(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=distributed_load"])
    logs = parse_logs(result.stdout)
    assert any("re-wiring to mesh" in str(log.get("message", log)) for log in logs)

def test_f3_logs_source_and_target_rewire(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=central_load"])
    logs = parse_logs(result.stdout)
    assert any("connected to Node" in str(log.get("message", log)) for log in logs)

def test_f3_reduces_network_latency(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=central_load"])
    logs = parse_logs(result.stdout)
    assert any("latency reduced" in str(log.get("message", log)) or "resistance minimized" in str(log.get("message", log)) for log in logs)

# F4 (Simulated Load)
def test_f4_simulated_load_generator_starts(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=load"])
    logs = parse_logs(result.stdout)
    assert any("Load generator started" in str(log.get("message", log)) for log in logs)

def test_f4_processes_steady_traffic(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=load"])
    logs = parse_logs(result.stdout)
    assert any("Traffic processed" in str(log.get("message", log)) for log in logs)

def test_f4_handles_varying_traffic_patterns(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=load"])
    logs = parse_logs(result.stdout)
    assert any("changing traffic pattern" in str(log.get("message", log)) for log in logs)

def test_f4_nodes_update_predictions(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=load"])
    logs = parse_logs(result.stdout)
    assert any("Updating predictive model" in str(log.get("message", log)) for log in logs)

def test_f4_traffic_metrics_logged(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=load"])
    logs = parse_logs(result.stdout)
    assert any("throughput" in str(log.get("message", log)).lower() for log in logs)

# F5 (Homeostasis/Recovery)
def test_f5_detects_massive_anomaly(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=anomaly"])
    logs = parse_logs(result.stdout)
    assert any("Massive anomaly detected" in str(log.get("message", log)) for log in logs)

def test_f5_temporary_instability_recorded(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=anomaly"])
    logs = parse_logs(result.stdout)
    assert any("error" in str(log.get("message", log)).lower() or "instability" in str(log.get("message", log)).lower() for log in logs)

def test_f5_initiates_recovery_actions(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=anomaly"])
    logs = parse_logs(result.stdout)
    recovery_actions = [log for log in logs if "action" in str(log.get("message", log)).lower() or "recover" in str(log.get("message", log)).lower()]
    assert len(recovery_actions) >= 2

def test_f5_achieves_homeostasis(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=anomaly"])
    logs = parse_logs(result.stdout)
    assert any("Homeostasis restored" in str(log.get("message", log)) for log in logs)

def test_f5_survives_without_crashing(run_origin_core, parse_logs):
    result = run_origin_core(["--scenario=anomaly"])
    assert result.returncode == 0
