# Handoff Report: Iteration 2 Failures (Milestone 1)

## 1. Observation
- `src/node.py` (line 7-11): The `Node.__init__` method accepts state variables (`expected_traffic`, `surprise_threshold`, `surprise_ratio`) without validating if they are `NaN`, `Inf`, or negative.
- `src/node.py` (line 22): The `receive_traffic` method executes `self.current_traffic += amount` without checking for float overflow, which can result in `inf`.
- `src/load_generator.py` (line 7-11): The `LoadGenerator.__init__` method lacks NaN/Inf checks and does not validate numerical boundaries for `variance` (must be $\ge 0$) or `anomaly_prob` (must be between 0.0 and 1.0).
- `src/load_generator.py` (line 18): `random.gauss(self.base_traffic, self.variance)` incorrectly receives variance as the second argument instead of the standard deviation.
- `src/load_generator.py` (lines 18-24): The output of `random.gauss` can be negative. If `anomaly_multiplier` is also negative, multiplying them together (line 22) results in a massive positive spike (sign flip). The value is only clamped to 0 at the very end.

## 2. Logic Chain
1. **Float Overflow**: Since Python floats overflow to `inf` at $\sim 1.8 \times 10^{308}$, an adversary sending massive traffic amounts will cause `self.current_traffic` to become `inf`. This cascades into `expected_traffic` and `surprise`, permanently bricking the node. A check is needed after the addition to raise an `OverflowError`.
2. **Negative Threshold Exact-Match Bug**: Because `surprise_threshold` and `surprise_ratio` aren't clamped or validated, initializing them with negative values means that a perfectly matching traffic (`surprise = 0`) will strictly exceed the negative threshold, erroneously triggering autonomous actions.
3. **Negative Anomaly Sign Flip**: If base traffic combined with Gaussian noise produces a negative value, multiplying it by a negative `anomaly_multiplier` yields a positive anomaly. To fix this, the traffic must be clamped with `max(0.0, traffic)` *before* applying the anomaly multiplier.
4. **Missing NaN/Inf Init Validation**: Without validation, state variables initialized to `NaN` will propagate silently, making `self.surprise > threshold` evaluate to false indefinitely.
5. **Probability Bounds**: `anomaly_prob` must be constrained to `[0.0, 1.0]` to ensure mathematically sound random distribution logic.
6. **Variance vs StdDev**: `random.gauss(mu, sigma)` expects standard deviation (`sigma`). By passing `variance`, the generator acts with a variance of $V^2$. Taking the square root of `variance` ensures correctness.

## 3. Caveats
- Raising `ValueError` and `OverflowError` alters the control flow during edge cases, which means callers must be prepared to catch these exceptions.
- Assuming `anomaly_multiplier` can legitimately be negative (e.g., to simulate a sudden drop in traffic), the final return statement in `LoadGenerator.generate` must remain `max(0.0, traffic)` to prevent returning negative traffic after the anomaly logic.

## 4. Conclusion
The codebase needs input validation and mathematical corrections. Recommended Fixes:

**In `src/node.py`**:
- In `__init__`, add validation for `expected_traffic`, `surprise_threshold`, and `surprise_ratio` to reject `NaN`, `Inf`, and negative values (raise `ValueError`).
- In `receive_traffic`, modify accumulation to check for overflow:
  ```python
  new_traffic = self.current_traffic + amount
  if math.isinf(new_traffic):
      raise OverflowError("Traffic accumulation resulted in float overflow")
  self.current_traffic = new_traffic
  ```

**In `src/load_generator.py`**:
- In `__init__`, add validation for all numeric parameters to reject `NaN` and `Inf`.
- Enforce `variance >= 0.0` and `0.0 <= anomaly_prob <= 1.0` (raise `ValueError`).
- In `generate`, correctly calculate standard deviation: `import math`, then `traffic = random.gauss(self.base_traffic, math.sqrt(self.variance))`.
- In `generate`, clamp noise *before* anomaly application to prevent sign flip:
  ```python
  traffic = max(0.0, traffic) # Fixes negative noise sign flip
  is_anomaly = random.random() < self.anomaly_prob
  if is_anomaly:
      traffic *= self.anomaly_multiplier
  return max(0.0, traffic)
  ```

## 5. Verification Method
- Execute the project test suite (`pytest` or equivalent).
- Add unit tests verifying `ValueError` is raised for: `Node(node_id="1", surprise_threshold=-1.0)`, `Node(..., expected_traffic=math.nan)`.
- Add unit tests verifying `OverflowError` is raised in `receive_traffic` when `sys.float_info.max` is added repeatedly.
- Add unit tests verifying `ValueError` is raised for `LoadGenerator(anomaly_prob=1.5)` or `variance=-1`.
- Verify the sign flip bug is absent by mocking `random.gauss` to return a negative value and confirming the output remains `0.0` when a negative anomaly multiplier is used.
