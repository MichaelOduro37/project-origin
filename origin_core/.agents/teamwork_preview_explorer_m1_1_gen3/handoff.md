# Handoff Report: Iteration 2 Failure Fix Strategy (Milestone 1)

## 1. Observation
I reviewed the failure report `iteration_2_failure.md`, `src/node.py`, and `src/load_generator.py`. 
- **Vuln 1 (Float Overflow)**: `Node.receive_traffic` at `node.py:22` does `self.current_traffic += amount` without checking if the result exceeds float capacity, which can lead to `inf`.
- **Vuln 2 & 4 (Node Init Validation)**: `Node.__init__` does not validate `expected_traffic`, `surprise_threshold`, or `surprise_ratio`. It allows negative numbers, `NaN`, and `inf` which corrupt the state.
- **Vuln 3 (Negative Anomaly Sign Flip)**: `LoadGenerator.generate()` at `load_generator.py:22` multiplies `traffic` by `anomaly_multiplier`. If `traffic` (from `random.gauss`) and `anomaly_multiplier` are both negative, the result is a huge positive spike.
- **Vuln 5 (Probability Bounds)**: `LoadGenerator.__init__` lacks boundary checks for `anomaly_prob` (allows values outside `[0.0, 1.0]`).
- **Vuln 6 (Variance vs StdDev)**: `LoadGenerator.generate()` calls `random.gauss(self.base_traffic, self.variance)` at `load_generator.py:18`, which treats variance as standard deviation.

## 2. Logic Chain
1. **Node Initialization**: We must add explicit bounds checking in `Node.__init__` using `math.isnan()` and `math.isinf()` for all float parameters, and enforce `>= 0` for traffic, threshold, and ratio to fix Vulns 2 and 4.
2. **Node Traffic Accumulation**: To prevent float overflow (Vuln 1), `receive_traffic` should assign the sum to a temporary variable and check `math.isinf(new_traffic)` before persisting it to `self.current_traffic`.
3. **Probability & Variance Setup**: `LoadGenerator.__init__` must strictly enforce `0.0 <= anomaly_prob <= 1.0` and `variance >= 0.0`. It should also reject `NaN` and `inf` for all numeric inputs (Vuln 5).
4. **Traffic Generation Mathematics**:
   - `random.gauss` expects a standard deviation. We must pass `math.sqrt(self.variance)` to correct the statistical distribution (Vuln 6).
   - To fix the sign flip (Vuln 3), we must clamp `traffic` to a minimum of `0.0` *before* applying the `anomaly_multiplier`. A negative noise generation will become `0.0`, and `0.0 * negative_multiplier` remains `0.0`, averting the spike.

## 3. Caveats
- I assumed `anomaly_multiplier` can legitimately be negative to simulate a sudden drop in traffic. If it's meant to be strictly positive, an additional `anomaly_multiplier > 0` check should be added to `LoadGenerator.__init__`. 
- No code was implemented as per the read-only exploration scope boundaries.

## 4. Conclusion
The codebase lacks foundational mathematical validation, resulting in state corruption during boundary conditions. The fix strategy involves:
- **`src/node.py`**: Add bounds and NaN/Inf validation to `__init__`. Add an overflow check (`math.isinf()`) after traffic addition in `receive_traffic`.
- **`src/load_generator.py`**: Add `0.0 <= anomaly_prob <= 1.0` and `variance >= 0.0` validation to `__init__` alongside NaN/Inf checks. In `generate()`, pass `math.sqrt(self.variance)` to `random.gauss`, and clamp `traffic` with `max(0.0, traffic)` prior to multiplying by `anomaly_multiplier`.

## 5. Verification Method
1. Implement the changes described above.
2. Verify with unit tests in `tests/`:
   - Instantiate `Node` and `LoadGenerator` with `-1`, `float('inf')`, and `float('nan')` to assert `ValueError`.
   - Pass `sys.float_info.max` to `Node.receive_traffic` to trigger an `OverflowError`.
   - Set `base_traffic = -10.0`, `variance = 100.0`, `anomaly_prob = 1.0`, and `anomaly_multiplier = -5.0` in `LoadGenerator` and assert that `generate()` returns `0.0` (not a positive spike).
3. Run `pytest` to confirm all gate checks pass.
