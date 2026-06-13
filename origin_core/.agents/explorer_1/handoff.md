# Handoff: Milestone 1 Test Plan

## 1. Observation
- `ORIGINAL_REQUEST.md` provides 5 core requirements based on natural laws (Free Energy Principle, Cellular Biology, Constructal Law). The acceptance criteria dictate that terminal output will "definitively log" specific behaviors like "high surprise", "spawning/throttling", "re-wiring", and "homeostasis".
- `TEST_INFRA.md` explicitly calls for "opaque-box, requirement-driven" testing via `subprocess` against `python main.py`. Tier 1 needs exactly 5 tests per feature (25 total tests).
- `SCOPE.md` defines the expected interface contracts for `conftest.py`: `run_origin_core(args: List[str]) -> subprocess.CompletedProcess` and `parse_logs(stdout: str) -> List[Dict[str, Any]]`.

## 2. Logic Chain
1. **Opaque-Box Constraints**: Since the test suite cannot access internal Python state and the application code does not yet exist, we must interact entirely through CLI arguments and standard output. 
2. **CLI Contract**: To test specific autonomous behaviors (e.g., handling traffic spikes or anomalies), the tests must inject specific conditions. We will assume `main.py` will accept `--scenario` flags (e.g., `--scenario spike`, `--scenario central-load`, `--scenario massive-anomaly`) or similar simulation flags to trigger these conditions.
3. **Log Parsing Contract**: The tests will rely on searching the captured standard output for specific substring signatures or structured log data matching the acceptance criteria.
4. **Test Distribution**: We will create 5 distinct assertions/scenarios per feature, focusing on the expected sequence of logs, boundary numbers (e.g., exactly 5 nodes), or latency metric differences.

## 3. Caveats
- The application implementation must conform to the CLI arguments (`--scenario`) and log phrases assumed by these tests. The implementer of `main.py` will need to read these tests to know what CLI interface and log outputs to build.
- We assume logs are either parseable as JSON dicts or predictable text lines.
- Timeouts will need to be configured on the `subprocess.run` calls, as the simulation might run indefinitely.

## 4. Conclusion
We will define the testing infrastructure and the 25 Tier 1 tests as follows:

### `tests/e2e/conftest.py`
Provides two primary fixtures:
- `run_origin_core`: A function taking `args: List[str]`, executing `subprocess.run(["python", "main.py"] + args, capture_output=True, text=True, timeout=10)`.
- `parse_logs`: A function taking stdout, extracting logging information (e.g., looking for `INFO:`, `WARNING:`, `ERROR:` signatures or parsing JSON objects).

### `tests/e2e/test_tier1_feature.py`

#### Feature 1: Network Initialization (>= 5 nodes)
1. `test_f1_initialization_count`: Assert exactly (or at least) 5 node initializations are logged.
2. `test_f1_unique_node_ids`: Parse node IDs from initialization logs, assert all are unique.
3. `test_f1_markov_blanket_creation`: Assert each initialized node logs the creation of its state boundary/Markov Blanket.
4. `test_f1_generative_model_init`: Assert logs show each node instantiating its generative model for traffic prediction.
5. `test_f1_startup_latency`: Measure timestamp delta from start to the final network-ready log, asserting it falls within expected fast startup bounds.

#### Feature 2: Free Energy Minimization (Spawn/Throttle)
1. `test_f2_spike_high_surprise`: Pass `--scenario spike`. Assert logs contain "high surprise" / "high free energy".
2. `test_f2_spike_spawn_subnode`: Pass `--scenario spike`. Assert "spawning sub-node" is logged after the surprise log.
3. `test_f2_drop_throttle_connection`: Pass `--scenario drop`. Assert logs indicate "low traffic / dropping prediction" and "throttling connection".
4. `test_f2_steady_no_action`: Pass `--scenario steady`. Assert no "spawning" or "throttling" logs occur.
5. `test_f2_spawn_registration`: Pass `--scenario spike`. Assert that the spawned sub-node is subsequently logged as receiving and processing its own traffic.

#### Feature 3: Constructal Topology Morphing (Re-wiring)
1. `test_f3_rewiring_triggered`: Pass `--scenario asymmetric-load`. Assert logs explicitly indicate "re-evaluating topology" or "re-wiring".
2. `test_f3_hub_and_spoke_formation`: Pass `--scenario central-load`. Assert logs show topology morphed to a "hub-and-spoke" pattern.
3. `test_f3_mesh_formation`: Pass `--scenario distributed-load`. Assert logs show topology morphed to a "mesh" pattern.
4. `test_f3_direct_connection_established`: Assert logs detail a specific direct link creation (e.g., "node A connects directly to node C").
5. `test_f3_latency_resistance_reduced`: Parse system latency/resistance logs before and after rewiring; assert the value decreases post-rewire.

#### Feature 4: Simulated Load Processing
1. `test_f4_load_generator_starts`: Assert logs contain "simulated load generator started".
2. `test_f4_traffic_pattern_varying`: Pass `--scenario varying-load`. Assert logs reflect receipt of different traffic shapes.
3. `test_f4_nodes_process_payloads`: Parse logs for "processed N requests" to verify traffic is actually hitting the leaf nodes.
4. `test_f4_traffic_routing_path`: Trace a specific request ID or batch ID in the logs to ensure it is forwarded across multiple nodes.
5. `test_f4_no_packet_loss_under_normal_load`: Pass `--scenario normal`. Assert zero "packet dropped" or "queue full" error logs.

#### Feature 5: Homeostasis / Recovery from Anomaly
1. `test_f5_anomaly_injection_logged`: Pass `--scenario anomaly`. Assert logs contain "massive traffic anomaly injected".
2. `test_f5_system_destabilizes`: Assert logs show temporary spikes in "surprise", error rates, or latency immediately post-anomaly.
3. `test_f5_cascade_recovery_actions`: Assert logs show a flurry of concurrent spawn, throttle, and rewire actions following the anomaly.
4. `test_f5_homeostasis_achieved`: Assert logs explicitly state "system recovered stability" or "homeostasis achieved" within the timeout period.
5. `test_f5_post_recovery_processing`: Assert that after homeostasis is achieved, subsequent normal load is processed without errors.

## 5. Verification Method
- **Review**: The orchestrator can review `handoff.md` to ensure exactly 25 test cases are planned and align with the constraints.
- **Next Steps**: A test implementer agent will create `conftest.py` and `test_tier1_feature.py` based on this plan. They will run `pytest tests/e2e/test_tier1_feature.py` (which will fail initially) and commit the test files.
