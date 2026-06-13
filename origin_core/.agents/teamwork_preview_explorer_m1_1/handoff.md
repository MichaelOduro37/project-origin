# Milestone 1: Core Node & Traffic Sim Handoff Report

## 1. Observation
- Inspected the repository at `c:\Users\ahmad ali\OneDrive\Desktop\Project Origin\origin_core`.
- The `src/` directory and `tests/` unit test files (`test_node.py` and `test_load_generator.py`) do not currently exist. 
- `PROJECT.md` and `SCOPE.md` require the creation of a `Node` class (`src/node.py`) with `receive_traffic(source_id, amount)` and `get_surprise()` methods. It must model expected traffic, calculate surprise (free energy) on deviation, and take actions (throttle/spawn).
- `PROJECT.md` requires `LoadGenerator` (`src/load_generator.py`) to generate varying traffic patterns and anomalies.
- `TEST_INFRA.md` specifies `pytest` as the project test runner.

## 2. Logic Chain
- Because the implementation is missing, the implementer needs to create `src/node.py`, `src/load_generator.py`, and their respective `pytest` test files in the `tests/` directory.
- **Node Implementation**:
  - Requires internal state for `expected_traffic`, `current_traffic`, and `surprise`.
  - `receive_traffic` accumulates the traffic for the current step.
  - A `step()` method is needed to compare `current_traffic` against `expected_traffic` to compute the `surprise` metric (e.g., absolute difference), decide on an action (e.g., "spawn" if surprise is above a high threshold, "throttle" if needed), and update `expected_traffic` (e.g., using an exponential moving average).
  - `get_surprise()` will simply return the last calculated surprise.
- **LoadGenerator Implementation**:
  - Requires a method (e.g., `generate_traffic`) to produce traffic based on different patterns (e.g., steady, sine wave, random spikes).
  - Anomaly injection can be supported via an `anomaly_chance` parameter to introduce random multipliers to the baseline traffic.
- **Test Implementation**:
  - Tests should be written using `pytest`.
  - `tests/test_node.py` should instantiate `Node`, call `receive_traffic`, invoke `step()`, and verify that `get_surprise()` returns >0 values when traffic deviates, and that actions are returned correctly.
  - `tests/test_load_generator.py` should ensure that the generator creates varying outputs based on the configuration.

## 3. Caveats
- The exact thresholds for "spawn" and "throttle" actions are not defined in `PROJECT.md`; the implementer should introduce reasonable defaults (e.g., configuration variables).
- The "spawn" and "throttle" actions currently do not have a defined downstream consumer in `node.py`, so they should either be returned by `step()` or stored in a state variable for `network.py` (M2) to query.
- Make sure to add `__init__.py` to the `src/` and `tests/` directories to ensure Python can resolve imports.

## 4. Conclusion
The implementation of Milestone 1 should proceed by scaffolding the `src` and `tests` directories. 
- Create `src/node.py` implementing the Cellular Biology/Free Energy principle with `receive_traffic`, `get_surprise`, and a `step` method for internal state updates.
- Create `src/load_generator.py` to output deterministic and random traffic data.
- Create `tests/test_node.py` and `tests/test_load_generator.py` using `pytest`.

## 5. Verification Method
- Execute `pytest tests/test_node.py tests/test_load_generator.py` from the root directory.
- Verify that tests pass successfully.
- Manually inspect `src/node.py` and `src/load_generator.py` to ensure they strictly implement the required APIs (`receive_traffic`, `get_surprise`, etc.) defined in `SCOPE.md`.
