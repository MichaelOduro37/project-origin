# Synthesis of Explorer Findings for Milestone 1

## Consensus
All 3 Explorers agree that the required files (`src/node.py`, `src/load_generator.py`, `tests/test_node.py`, `tests/test_load_generator.py`) must be created from scratch. They agree on the API and state requirements for both classes.

## Action Plan for Implementation
1. Scaffold directories: Ensure `src/` and `tests/` exist. Create `src/__init__.py` and `tests/__init__.py`.
2. Implement `src/node.py` with `Node` class:
   - State: `node_id`, `expected_traffic`, `current_traffic` (to accumulate per step), `surprise`.
   - `receive_traffic(self, source_id, amount)`: Accumulates traffic.
   - `step(self)`: 
     - Calculates `surprise = abs(current_traffic - expected_traffic)`.
     - Logs/triggers an autonomous action (e.g., "spawn" or "throttle") if `surprise` exceeds a threshold (e.g., > 10.0 or configured).
     - Updates generative model for `expected_traffic` (Exponential Moving Average or Simple Moving Average).
     - Resets `current_traffic` for the next step.
   - `get_surprise(self)`: Returns the calculated surprise.
3. Implement `src/load_generator.py` with `LoadGenerator` class:
   - Generates baseline traffic with normal variance.
   - Has a mechanism to inject anomalies (e.g., spike multiplier based on probability or specific step).
4. Implement `pytest` unit tests:
   - `tests/test_node.py`: verify traffic accumulation, surprise math, moving average update, and threshold triggering.
   - `tests/test_load_generator.py`: verify baseline and anomaly generation.
5. Verification:
   - Run `pytest`. All tests must pass.
