# Handoff Report: Milestone 1 (Core Node & Traffic Sim)

## 1. Observation
- The target files (`src/node.py`, `src/load_generator.py`, `tests/test_node.py`, `tests/test_load_generator.py`) do not yet exist in the repository.
- `PROJECT.md` and `SCOPE.md` mandate that `Node` must implement `receive_traffic(source_id, amount)` and `get_surprise()`. It must model expected traffic and minimize "free energy" (surprise) by taking autonomous actions like spawning or throttling when actual traffic deviates from expected.
- `ORIGINAL_REQUEST.md` specifies that the terminal output must explicitly log nodes detecting high "free energy/surprise" and autonomously spawning/throttling.
- `TEST_INFRA.md` specifies that testing uses `pytest`.

## 2. Logic Chain
- Because the `Node` needs to model expected traffic, an Exponential Moving Average (EMA) or Simple Moving Average of received traffic per step is an appropriate, lightweight generative model.
- Because `Node` must calculate "surprise", it should compute the absolute difference between the total received traffic in the current step and the expected traffic.
- If the calculated "surprise" exceeds a predefined threshold, the `Node` should trigger an autonomous action (e.g., logging "High surprise detected. Spawning sub-node to handle load") and potentially return an action payload to the simulation loop (which will be implemented in M2/M3).
- `LoadGenerator` needs to inject varying traffic and anomalies. It should expose a method, like `get_traffic(step_num)`, which returns traffic assignments for nodes. It should support injecting a sudden spike (anomaly) at a specific step to trigger the high-surprise conditions in the nodes.
- To meet code layout constraints and pass tests, the implementation must be cleanly separated into `src/` and `tests/`.

## 3. Caveats
- The exact signature for the autonomous action's feedback to the network is not fully specified yet. For M1, simply logging the action and updating internal node state is sufficient to meet the acceptance criteria ("The terminal output definitively logs...").
- Spawning a sub-node actually modifying the network topology is an M2 concern, so M1's `Node.step()` should just simulate the action (e.g., throttle internal capacity, return an intention, log the event).

## 4. Conclusion
We are ready to implement Milestone 1. The implementer should:
1. Create `src/node.py` containing a `Node` class with:
   - State: `id`, `expected_traffic` (float), `current_traffic` (dict/buffer), `surprise` (float).
   - Methods: `receive_traffic(source_id, amount)`, `get_surprise()`, and `step()`.
   - `step()` logic: Compute total traffic, calculate `surprise` as `abs(total - expected)`, update `expected` using EMA, and if `surprise > threshold`, log an action and clear `current_traffic`.
2. Create `src/load_generator.py` containing a `LoadGenerator` class capable of generating normal step-based traffic and an anomaly spike.
3. Create unit tests `tests/test_node.py` and `tests/test_load_generator.py` using `pytest` to verify the mathematical logic (EMA updates, surprise calculation) and ensure threshold logging works.

## 5. Verification Method
- **Tests**: Run `pytest tests/test_node.py tests/test_load_generator.py` and verify all pass.
- **Manual Check**: Instantiate a `Node`, send traffic over a few steps, verify `expected_traffic` converges. Send a massive spike and verify `get_surprise()` returns a high value and the node prints an action log.
