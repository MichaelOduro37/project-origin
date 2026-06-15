# Handoff Report: Tier 2 (Boundary/Corner) E2E Test Design

## 1. Observation
- The `origin_core/TEST_INFRA.md` file defines the test philosophy and feature inventory.
- Tests must be opaque-box, requirement-driven, using category-partition and BVA (Boundary Value Analysis).
- 5 core features are identified: Network Initialization (>= 5 nodes), Free Energy Minimization (Spawn/Throttle), Constructal Topology Morphing (Re-wiring), Simulated Load Processing, and Homeostasis / Recovery from Anomaly.
- Tests will be implemented in Python using `subprocess` to call `python main.py` and must parse `stdout`, `stderr`, and return codes to verify expected behavior.
- The system implementation is not yet completed; these tests define the contract.

## 2. Logic Chain
- To achieve BVA (Tier 2 coverage) for each of the 5 features, we must test the extremes of their input domains, such as minimum bounds, maximum bounds, empty/zero bounds, invalid types, and concurrency/state extremes.
- For each test, we define a CLI command (e.g., `python main.py --feature-flag <val>`), the expected logging signature in `stdout`/`stderr` based on standard project logging practices, and the expected process exit code (`0` for success, non-zero for failures/graceful aborts).

### Feature 1: Network Initialization (>= 5 nodes)
*Boundary: Minimum required nodes, below minimum, maximum limit, zero/negative, and invalid types.*
1. **T2.1.1 [Exact Minimum Boundary]**: Start with exactly 5 nodes.
   - Command: `python main.py --init-nodes 5`
   - Assertion: `assert "Network initialized with 5 nodes" in stdout`
   - Return Code: `0`
2. **T2.1.2 [Just Below Minimum Boundary]**: Start with 4 nodes.
   - Command: `python main.py --init-nodes 4`
   - Assertion: `assert "Insufficient nodes" in stderr` and `assert "defaulting to 5" in stdout` (or similar warning).
   - Return Code: `0` (or `1` if strict).
3. **T2.1.3 [Stress/Max Boundary]**: Start with a massive number of nodes.
   - Command: `python main.py --init-nodes 10000`
   - Assertion: `assert "Node count capped" in stdout` or system initializes successfully without crashing.
   - Return Code: `0`
4. **T2.1.4 [Zero/Negative Boundary]**: Start with 0 or -1 nodes.
   - Command: `python main.py --init-nodes 0`
   - Assertion: `assert "Invalid node count" in stderr`
   - Return Code: `>0`
5. **T2.1.5 [Type Boundary]**: Start with non-integer.
   - Command: `python main.py --init-nodes five`
   - Assertion: `assert "invalid literal" in stderr` or `assert "Argument parsing error" in stderr`
   - Return Code: `>0`

### Feature 2: Free Energy Minimization (Spawn/Throttle)
*Boundary: Zero energy, max capacity constraints, minimum capacity constraints, threshold debouncing.*
1. **T2.2.1 [Zero Energy Boundary]**: System starts with exactly 0 free energy.
   - Command: `python main.py --sim-energy 0`
   - Assertion: `assert "Throttling: 0 free energy" in stdout`
   - Return Code: `0`
2. **T2.2.2 [Max Node Capacity Boundary]**: Spawn triggered but system is at maximum allowed nodes.
   - Command: `python main.py --force-spawn --max-nodes 10 --init-nodes 10`
   - Assertion: `assert "Spawn rejected: max nodes reached" in stdout`
   - Return Code: `0`
3. **T2.2.3 [Min Node Capacity Boundary]**: Throttle triggered but system is at the absolute minimum nodes (5).
   - Command: `python main.py --force-throttle --init-nodes 5`
   - Assertion: `assert "Throttle rejected: min nodes reached" in stdout`
   - Return Code: `0`
4. **T2.2.4 [Debounce/Oscillation Boundary]**: Rapid alternating spawn/throttle signals.
   - Command: `python main.py --sim-oscillation-energy`
   - Assertion: `assert "Debouncing" in stdout` or `assert "Rate limit exceeded" in stdout`
   - Return Code: `0`
5. **T2.2.5 [Excessive Energy Boundary]**: Extremely high free energy injected instantly.
   - Command: `python main.py --sim-energy 99999999`
   - Assertion: `assert "Spawning max capacity" in stdout`
   - Return Code: `0`

### Feature 3: Constructal Topology Morphing (Re-wiring)
*Boundary: Fully connected, minimal spanning tree, zero-latency edges, partitioning prevention.*
1. **T2.3.1 [Max Density Boundary]**: Topology is already fully connected; re-wiring attempts to add edges.
   - Command: `python main.py --topology fully-connected --trigger-rewire`
   - Assertion: `assert "No more re-wiring possible" in stdout` or `assert "optimal" in stdout`
   - Return Code: `0`
2. **T2.3.2 [Minimal Spanning Boundary]**: Topology is a minimal tree; re-wiring attempts to remove edges.
   - Command: `python main.py --topology minimal-tree --trigger-rewire-reduction`
   - Assertion: `assert "Cannot remove edges" in stdout` or `assert "minimal connectivity" in stdout`
   - Return Code: `0`
3. **T2.3.3 [Partition Prevention Boundary]**: Re-wiring would cause an isolated sub-graph.
   - Command: `python main.py --sim-partitioning-rewire`
   - Assertion: `assert "Re-wiring rejected: network partitioned" in stderr` or log.
   - Return Code: `0`
4. **T2.3.4 [Zero Latency/Distance Boundary]**: Node distance is exactly 0.
   - Command: `python main.py --sim-node-distance 0`
   - Assertion: `assert "distance=0" in stdout` (Should handle math smoothly without ZeroDivisionError).
   - Return Code: `0`
5. **T2.3.5 [Concurrent Rewiring Boundary]**: Multiple nodes attempt to re-wire simultaneously.
   - Command: `python main.py --sim-concurrent-rewire 10`
   - Assertion: `assert "Resolving concurrent re-wiring" in stdout` or no crash occurs.
   - Return Code: `0`

### Feature 4: Simulated Load Processing
*Boundary: Zero load, massive load, minimal possible load, malformed load, timeout.*
1. **T2.4.1 [Idle/Zero Load Boundary]**: System receives exactly 0 load requests.
   - Command: `python main.py --sim-load 0`
   - Assertion: `assert "Processing load: 0" in stdout` or system remains gracefully idle.
   - Return Code: `0`
2. **T2.4.2 [Minimum Load Boundary]**: System receives exactly 1 atomic unit of load.
   - Command: `python main.py --sim-load 1`
   - Assertion: `assert "Load complete" in stdout`
   - Return Code: `0`
3. **T2.4.3 [Capacity/Backpressure Boundary]**: System receives load drastically exceeding queue size.
   - Command: `python main.py --sim-load 1000000 --queue-size 100`
   - Assertion: `assert "Queue full" in stdout` or `assert "Backpressure applied" in stdout`
   - Return Code: `0`
4. **T2.4.4 [Timeout Boundary]**: Processing a single load takes longer than the max allowed TTL.
   - Command: `python main.py --sim-load 1 --sim-processing-delay 9999`
   - Assertion: `assert "Timeout processing load" in stderr` or `stdout`
   - Return Code: `0` or `>0` depending on strictness.
5. **T2.4.5 [Malformed Data Boundary]**: Load simulator is given corrupt or invalid format strings.
   - Command: `python main.py --sim-load-file invalid_format.bin`
   - Assertion: `assert "Error parsing load data" in stderr`
   - Return Code: `>0`

### Feature 5: Homeostasis / Recovery from Anomaly
*Boundary: Near-total failure, sub-threshold anomalies, cascading recoveries.*
1. **T2.5.1 [Near-Total Failure Boundary]**: Anomaly destroys N-1 nodes.
   - Command: `python main.py --sim-anomaly near-total`
   - Assertion: `assert "Critical anomaly: recovering from 1 node" in stdout`
   - Return Code: `0`
2. **T2.5.2 [Sub-Threshold Boundary]**: Anomaly occurs but is mathematically below the trigger threshold.
   - Command: `python main.py --sim-anomaly minor`
   - Assertion: `assert "Anomaly ignored: below threshold" in stdout`
   - Return Code: `0`
3. **T2.5.3 [Cumulative Threshold Boundary]**: A sequence of minor anomalies that cumulatively breach the threshold.
   - Command: `python main.py --sim-anomaly drip-feed`
   - Assertion: `assert "Homeostasis triggered by cumulative anomalies" in stdout`
   - Return Code: `0`
4. **T2.5.4 [Cascading Failure Boundary]**: A massive anomaly hits exactly while the system is already recovering.
   - Command: `python main.py --sim-anomaly double-strike`
   - Assertion: `assert "Recovery interrupted" in stdout` or `assert "restarting recovery" in stdout`
   - Return Code: `0`
5. **T2.5.5 [Resource-Starved Recovery Boundary]**: System attempts to recover nodes but free energy is exactly 0.
   - Command: `python main.py --sim-anomaly standard --sim-energy 0`
   - Assertion: `assert "Recovery delayed: insufficient free energy" in stdout`
   - Return Code: `0`

## 3. Caveats
- **Implementation Mismatch**: Since the implementation is not yet complete, the specific CLI flags (e.g., `--init-nodes`, `--sim-energy`) and precise error messages are predictive. The tests will fail when first run, which is intended, and the implementer will build `main.py` to satisfy these test contracts or adjust the test string assertions if their design differs slightly.
- **Log formats**: Exact string matching might be brittle. When implemented, tests might need to use regex matching or structural log parsing if the output uses JSON formatting.

## 4. Conclusion
We have successfully designed 25 Tier 2 (Boundary/Corner coverage) E2E tests covering all 5 core features outlined in `TEST_INFRA.md`. These tests utilize `subprocess` to call `main.py` and enforce system stability at the absolute boundaries of state, load, and failure.

## 5. Verification Method
- Ensure the `main.py` test harness (e.g. `tests/e2e/test_tier2_boundary.py`) executes these 25 commands using `subprocess.run()`.
- Validate that the `stdout`, `stderr`, and `returncode` matching logic mirrors the assertions outlined in Section 2.
- Execute `pytest tests/e2e/test_tier2_boundary.py`. Currently, all 25 tests should **fail**, verifying that the boundary constraints have not yet been implemented in `main.py`.
