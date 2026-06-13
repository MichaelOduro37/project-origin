import pytest
import re
import sys
from conftest import run_origin_core

# Feature 1: Network Initialization
def test_f1_b1_min_valid():
    res = run_origin_core(["--nodes=5"])
    assert res.returncode == 0
    assert re.search(r"Network initialized with 5 nodes", res.stdout)

def test_f1_b2_zero_nodes():
    res = run_origin_core(["--nodes=0"])
    assert res.returncode > 0
    assert re.search(r"Error.*minimum nodes", res.stderr)

def test_f1_c3_negative_nodes():
    res = run_origin_core(["--nodes=-1"])
    assert res.returncode > 0
    assert re.search(r"Error.*invalid node count", res.stderr)

def test_f1_b4_max_stress():
    res = run_origin_core(["--nodes=1000"])
    assert res.returncode == 0
    assert "1000 nodes" in res.stdout

def test_f1_c5_invalid_type():
    res = run_origin_core(["--nodes=five"])
    assert res.returncode > 0
    assert "Traceback" not in res.stderr

# Feature 2: Free Energy Minimization
def test_f2_b6_zero_surprise():
    res = run_origin_core(["--load=matching"])
    assert not re.search(r"action=(SPAWN|THROTTLE)", res.stdout)

def test_f2_b7_threshold_minus_one():
    res = run_origin_core(["--load=threshold-minus-1"])
    # No action triggered
    assert not re.search(r"action=(SPAWN|THROTTLE)", res.stdout)

def test_f2_b8_threshold_exact():
    res = run_origin_core(["--load=threshold-exact"])
    assert re.search(r"Surprise threshold met.*action=(SPAWN|THROTTLE)", res.stdout)

def test_f2_c9_instant_drop():
    res = run_origin_core(["--load=instant-drop"])
    assert re.search(r"action=(HIBERNATE|THROTTLE)", res.stdout)

def test_f2_c10_integer_overflow():
    res = run_origin_core([f"--load={sys.maxsize}"])
    assert re.search(r"action=SPAWN_MAX", res.stdout)

# Feature 3: Constructal Topology Morphing
def test_f3_b11_perfect_balance():
    res = run_origin_core(["--load-dist=perfect-balance"])
    assert not re.search(r"Topology re-wire", res.stdout)

def test_f3_c12_extreme_asymmetry():
    res = run_origin_core(["--load-dist=extreme-asymmetry"])
    assert re.search(r"Re-wiring.*hub-and-spoke", res.stdout)

def test_f3_c13_fully_mesh_boundary():
    res = run_origin_core(["--topology=fully-mesh", "--load=uniform-high"])
    # Assuming the absence of flapping or adding edges implies it maintains state
    assert not re.search(r"Adding edge", res.stdout)

def test_f3_c14_line_graph_bypass():
    res = run_origin_core(["--topology=line-graph", "--load-nodes=A,E"])
    assert re.search(r"Adding edge A <-> E", res.stdout)

def test_f3_c15_flapping_dampening():
    res = run_origin_core(["--load=oscillation-A-B"])
    assert re.search(r"Re-wire damped", res.stdout)

# Feature 4: Simulated Load Processing
def test_f4_b16_zero_load():
    res = run_origin_core(["--load-volume=0"])
    assert re.search(r"load=0", res.stdout)

def test_f4_c17_invalid_target():
    res = run_origin_core(["--load-target=999"])
    assert re.search(r"Dropped load.*invalid node 999", res.stdout)

def test_f4_c18_high_freq_updates():
    res = run_origin_core(["--load-freq=0.001"])
    # Buffer the micro-loads
    assert "buffer" in res.stdout.lower() or "cumulativ" in res.stdout.lower()

def test_f4_b19_massive_spike():
    res = run_origin_core(["--load-spike=1000000"])
    assert res.returncode == 0

def test_f4_c20_negative_load():
    res = run_origin_core(["--load=-50"])
    assert re.search(r"Invalid load value", res.stderr)

# Feature 5: Homeostasis / Recovery
def test_f5_b21_impulse_anomaly():
    res = run_origin_core(["--anomaly=impulse"])
    assert re.search(r"Homeostasis restored", res.stdout)

def test_f5_c22_sustained_anomaly():
    res = run_origin_core(["--anomaly=permanent"])
    assert re.search(r"New homeostasis baseline established", res.stdout)

def test_f5_c23_multi_simultaneous_anomaly():
    res = run_origin_core(["--anomaly=multi-node"])
    assert re.search(r"recovery", res.stdout.lower())

def test_f5_b24_no_anomaly():
    res = run_origin_core(["--anomaly=none"])
    assert re.search(r"stable", res.stdout.lower())

def test_f5_c25_max_throttle_deadlock():
    res = run_origin_core(["--anomaly=max-throttle"])
    assert re.search(r"Global throttle", res.stdout)
