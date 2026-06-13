# Handoff Report: Origin Core Boundary & Threshold Fix Strategy

## Observation
1. **Fixed Surprise Threshold**: In `src/node.py` line 29, the condition is `if self.surprise > 10.0:`. This hardcoded absolute threshold causes hyperactivity at high load scales (e.g., a fluctuation of 11 on an expected traffic of 10,000 is a tiny percentage but will trigger an action).
2. **Invalid Traffic Acceptance**: In `src/node.py` line 13 (`receive_traffic`), there is no validation on the `amount` parameter. It blindly executes `self.current_traffic += amount`, allowing negative values, `float('nan')`, and `float('inf')` to poison the node's state variables (`current_traffic`, `expected_traffic`, and `surprise`).
3. **LoadGenerator Generate Flaw**: In `src/load_generator.py` lines 23-25 (`generate`), the anomaly multiplier is applied *after* the `traffic = max(0.0, traffic)` check. If `anomaly_multiplier` is negative, the method will return a negative traffic value.
4. **LoadGenerator Deterministic Flaw**: In `src/load_generator.py` lines 32-36 (`generate_deterministic`), the generated traffic is never bounded. If `base_traffic` is negative or `anomaly_multiplier` is negative, it returns negative traffic.

## Logic Chain
1. **Addressing Surprise Threshold**: To fix the hyper-reactivity at scale while maintaining sensitivity at low loads, the node should use a combination of absolute and relative thresholds. If we introduce `surprise_threshold_absolute` and `surprise_threshold_relative`, the node can trigger actions only when `self.surprise > max(surprise_threshold_absolute, self.expected_traffic * surprise_threshold_relative)`.
2. **Addressing Invalid Traffic**: To prevent state poisoning, `Node.receive_traffic` must actively reject invalid data. We should `import math` and add an explicit guard: `if amount < 0 or math.isnan(amount) or math.isinf(amount): raise ValueError(...)`.
3. **Addressing LoadGenerator Bounds**: To guarantee non-negative traffic emission, the zero-bound clamping (`traffic = max(0.0, traffic)`) must be the *final* operation performed in both `generate()` and `generate_deterministic()` before returning the value.

## Caveats
- Raising `ValueError` in `Node.receive_traffic` will cause the caller to fail if it sends invalid traffic. This is a strict "fail-fast" rejection approach; if the system prefers resilience over failing, it could alternatively log a warning and ignore the invalid traffic. However, strict rejection is standard for boundary validation.
- The relative threshold approach requires sensible defaults (e.g., absolute `10.0` and relative `0.1` or 10%). The implementer should make these configurable via `__init__`.

## Conclusion
The recommended fix strategy is:
1. **In `src/node.py`**:
   - Add `import math`.
   - Update `receive_traffic(self, source_id: str, amount: float)` to raise `ValueError` if `amount < 0` or `math.isnan(amount)` or `math.isinf(amount)`.
   - Update `__init__` to accept `surprise_threshold_absolute=10.0` and `surprise_threshold_relative=0.1`.
   - Update `step()` to use `if self.surprise > max(self.surprise_threshold_absolute, self.expected_traffic * self.surprise_threshold_relative):`.
2. **In `src/load_generator.py`**:
   - In `generate()`, move `traffic = max(0.0, traffic)` to the very end, right before `return traffic`.
   - In `generate_deterministic()`, add `traffic = max(0.0, traffic)` right before `return traffic`.

## Verification Method
1. **Tests to Implement**:
   - `test_node_rejects_negative_traffic`: Assert `Node.receive_traffic(-1.0)` raises `ValueError`.
   - `test_node_rejects_nan_inf_traffic`: Assert `Node.receive_traffic(float('nan'))` and `float('inf')` raise `ValueError`.
   - `test_node_relative_threshold`: Set expected traffic to `1000`. Provide `1005` traffic. Assert no action is triggered (surprise `5` < relative threshold `100`).
   - `test_load_generator_never_negative`: Initialize `LoadGenerator` with a negative base traffic or negative anomaly multiplier and assert both `generate()` and `generate_deterministic()` return `0.0`.
2. **Commands**:
   - Run `pytest tests/` after implementing the fixes.
