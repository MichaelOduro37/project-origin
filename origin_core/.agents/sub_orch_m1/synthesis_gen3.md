# Iteration 3 Synthesis Plan: Milestone 1 Failures

## Overview
Based on the analysis from Gen 3 Explorers, the Iteration 2 failures in Milestone 1 (Core Node & Traffic Sim) stem from a lack of mathematical boundary checks and input validation in `src/node.py` and `src/load_generator.py`. 

## 1. Code Fixes Needed

### `src/node.py`
1. **Init Validation**: Modify `__init__` to validate `expected_traffic`, `surprise_threshold`, and `surprise_ratio`.
   - Reject `math.isnan` or `math.isinf` values (raise `ValueError`).
   - Ensure `surprise_threshold >= 0` and `surprise_ratio >= 0` (raise `ValueError`).
2. **Float Overflow**: In `receive_traffic`, calculate `new_traffic = self.current_traffic + amount`. Check if `math.isinf(new_traffic)`. If so, raise an `OverflowError`. Otherwise, assign it to `self.current_traffic`.

### `src/load_generator.py`
1. **Init Validation**: Modify `__init__` to ensure:
   - `0.0 <= anomaly_prob <= 1.0` (raise `ValueError`).
   - `variance >= 0.0` (raise `ValueError`).
   - Reject `math.isnan` or `math.isinf` for numeric parameters.
2. **Variance vs StdDev**: In `generate()`, `random.gauss` expects the standard deviation. Import `math` and pass `math.sqrt(self.variance)` instead of `self.variance`.
3. **Negative Anomaly Sign Flip**: In `generate()`, clamp the output of `random.gauss` to `0.0` *before* multiplying by `anomaly_multiplier`. 
   `traffic = max(0.0, random.gauss(self.base_traffic, math.sqrt(self.variance)))`

## 2. Test Requirements
You must update or add tests in the `tests/` directory to explicitly cover these edge cases:
- `Node` initialization with `NaN`, `inf`, and negative thresholds raises `ValueError`.
- `Node.receive_traffic` raises `OverflowError` when adding extremely large values (e.g. `1e308`).
- `LoadGenerator` initialization with negative variance or out-of-bounds probabilities raises `ValueError`.
- `LoadGenerator.generate` returns `0.0` (and not a massive positive spike) when a negative noise combined with a negative `anomaly_multiplier` occurs.

## 3. Verification
Run the tests (e.g., `pytest`) and ensure they all pass before delivering your handoff.
