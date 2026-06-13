# Handoff Report: Tier 2 Boundary/Corner E2E Test Design

## 1. Observation
- `TEST_INFRA.md` requires 25 Tier 2 (Boundary/Corner coverage) E2E tests, 5 for each of the 5 core features.
- The 5 features are: 
  1. Network Initialization (>= 5 nodes)
  2. Free Energy Minimization (Spawn/Throttle)
  3. Constructal Topology Morphing (Re-wiring)
  4. Simulated Load Processing
  5. Homeostasis / Recovery from Anomaly
- Tests must be `pytest` based, use `subprocess` to call `python src/main.py` (or `main.py`), and verify behavior via stdout/stderr/returncodes.
- `PROJECT.md` states the codebase layout includes `src/main.py`, `src/node.py`, `src/network.py`, and `src/load_generator.py`.
- `ORIGINAL_REQUEST.md` specifies output must definitively log nodes detecting high "free energy/surprise", autonomously spawning/throttling, successfully re-wiring, and recovering stability.

## 2. Logic Chain
- Since the implementation is incomplete, tests will execute `subprocess.run(["python", "src/main.py", ...], capture_output=True, text=True)`.
- Because the system uses "verifiable logging", tests should assert against the process's standard output using string matching or regular expressions (e.g., searching for `SPAWN`, `THROTTLE`, `RE-WIRE`, `HOMEOSTASIS`).
- For Tier 2 tests, inputs must stress boundaries (0, negative, extremely large, exactly at threshold, etc.). Since CLI arguments aren't fully defined yet, the tests will assume passing standard environment variables or flags (like `--nodes=5`, `--inject-load=0`) that `main.py` will later be built to parse.

## 3. Caveats
- Exact CLI parameters for `main.py` (like `--nodes=X` or `--anomaly=Y`) have not been implemented. Test design assumes these boundary variables can be injected.
- The exact log formats are not yet locked in; regex assertions are proposed conceptually (e.g., `r"Node.*action=SPAWN"`) and will need adjustment once `main.py` logging is solidified.
- Performance and stability boundaries (like infinite re-wiring damping) assume the implementation will eventually support a mechanism to prevent flapping, which is what is being tested.

## 4. Conclusion

**Test Strategy:**
Use `subprocess.run` to execute `main.py` with boundary-inducing arguments. Capture `stdout` and `stderr`. Assert return code is 0 (or >0 for intentional invalid boundaries), and use regex on `stdout` to verify the presence of required semantic logs (e.g., `Node X free_energy=...`).

### Feature 1: Network Initialization (>= 5 nodes)
1. **Boundary (Minimum valid):** Run `main.py` with `--nodes=5`.
   *Assertion:* Process exits 0. Regex `r"Network initialized with 5 nodes"` found in stdout.
2. **Boundary (Zero nodes):** Run `main.py` with `--nodes=0`.
   *Assertion:* Process exits >0. Regex `r"Error.*minimum nodes"` in stderr.
3. **Corner (Negative nodes):** Run `main.py` with `--nodes=-1`.
   *Assertion:* Process exits >0. Regex `r"Error.*invalid node count"` in stderr.
4. **Boundary (Maximum/Stress):** Run `main.py` with `--nodes=1000`.
   *Assertion:* Process exits 0. Network initialization completes without `MemoryError` and logs `1000 nodes`.
5. **Corner (Invalid type):** Run `main.py` with `--nodes=five`.
   *Assertion:* Process exits >0 with graceful parsing failure log, not an unhandled Python traceback.

### Feature 2: Free Energy Minimization (Spawn/Throttle)
6. **Boundary (Zero Surprise):** Inject traffic exactly matching node's generative model.
   *Assertion:* Stdout does NOT contain `action=SPAWN` or `action=THROTTLE`.
7. **Boundary (Threshold - 1):** Inject traffic strictly one unit below the surprise threshold.
   *Assertion:* Surprise is logged as high, but NO action is triggered.
8. **Boundary (Threshold exact):** Inject traffic exactly at the threshold of surprise.
   *Assertion:* Stdout matches `r"Surprise threshold met.*action=(SPAWN|THROTTLE)"`.
9. **Corner (Instant drop to 0):** Traffic goes from high load to exactly 0 instantaneously.
   *Assertion:* Stdout logs a massive negative prediction error and triggers `action=HIBERNATE` or maximum `action=THROTTLE`.
10. **Corner (Integer Overflow Load):** Inject an impossibly large traffic payload (`sys.maxsize`).
    *Assertion:* Node caps prediction error and safely logs `action=SPAWN_MAX` without math overflow crashes.

### Feature 3: Constructal Topology Morphing (Re-wiring)
11. **Boundary (Perfect Balance):** Distribute load perfectly evenly across all 5 nodes.
    *Assertion:* `r"Topology re-wire"` is NOT found in stdout over the simulation step.
12. **Corner (Extreme Asymmetry):** Route 100% of load generator traffic to exactly one peripheral node.
    *Assertion:* Stdout logs `r"Re-wiring.*hub-and-spoke"` centered on the over-loaded node.
13. **Corner (Fully Mesh Boundary):** Start with nodes fully connected and inject uniformly high load everywhere.
    *Assertion:* Network maintains fully connected state; does not flap or attempt to add existing edges.
14. **Corner (Line Graph Bypass):** Start with 5 nodes in a line (A-B-C-D-E), load purely between A and E.
    *Assertion:* Stdout logs `r"Adding edge A <-> E"` (direct bypass created to minimize latency).
15. **Corner (Flapping Dampening):** Inject highly oscillating load alternating rapidly between node A and B.
    *Assertion:* Network changes topology once or twice, then logs `r"Re-wire damped"` or stabilizes, avoiding infinite loop.

### Feature 4: Simulated Load Processing
16. **Boundary (Zero Load):** Run load generator with 0 volume.
    *Assertion:* Nodes log `load=0`, no errors occur, prediction models slowly decay to 0.
17. **Corner (Invalid Target):** Send load to `Node ID 999` in a 5-node network.
    *Assertion:* Load generator logs `r"Dropped load.*invalid node 999"`, process does not crash.
18. **Corner (High Frequency Updates):** Send micro-loads every 0.001 simulation seconds (faster than model update tick).
    *Assertion:* Node buffers the micro-loads and processes them cumulatively at the tick, logging successful aggregation.
19. **Boundary (Single Massive Spike):** Inject 1,000,000 load in a single tick, followed by 0.
    *Assertion:* System processes the spike in one step without segment fault, updating free energy drastically.
20. **Corner (Negative Load):** Inject a load of `-50`.
    *Assertion:* Load generator rejects the command or clamps to 0; stderr logs `r"Invalid load value"`.

### Feature 5: Homeostasis / Recovery from Anomaly
21. **Boundary (Impulse Anomaly):** Inject a massive anomaly that lasts exactly 1 simulation tick.
    *Assertion:* System reacts, then logs `r"Homeostasis restored"` within a defined low number of recovery ticks.
22. **Corner (Sustained Permanent Anomaly):** Inject anomaly that never ends.
    *Assertion:* Instead of endlessly trying to "recover", the network stabilizes into a new baseline and logs `r"New homeostasis baseline established"`.
23. **Corner (Multiple Simultaneous Anomalies):** Inject anomalies into 3 different nodes at the exact same tick.
    *Assertion:* Network does not deadlock; logs show global recovery and re-wiring resolving all 3 zones.
24. **Boundary (No Anomaly):** Run baseline simulation with no anomaly injection.
    *Assertion:* Stability metric / free energy average remains consistently low and logged as `stable`.
25. **Corner (Max Throttle Deadlock Check):** Anomaly is so large it forces all nodes to throttle to 0.
    *Assertion:* Network logs `r"Global throttle"`, but successfully re-admits traffic when anomaly ends, proving no deadlock.

## 5. Verification Method
To independently verify this strategy:
1. Examine `tests/e2e/test_tier2_boundary.py` (to be created by an implementer based on this report).
2. Run `pytest tests/e2e/test_tier2_boundary.py`.
3. The tests will currently `FAILED` because `main.py` is missing or incomplete, but the tests themselves should compile and assert `subprocess.run` calls.
4. Review the regular expressions in the tests against `PROJECT.md` logging requirements.
