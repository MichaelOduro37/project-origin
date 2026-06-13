# Milestone 1 Review Handoff

## 1. Observation
- `src/node.py` implements `Node` with `receive_traffic`, `step`, and `get_surprise`.
- `receive_traffic` accumulates traffic via `self.current_traffic += amount` without validation.
- `src/load_generator.py` implements `LoadGenerator` with `generate` and `generate_deterministic`.
- `generate` clamps traffic to `0.0` but does so *before* applying `anomaly_multiplier`. Thus, if `anomaly_multiplier` is negative, the resulting traffic is negative.
- `generate_deterministic` does not clamp `base_traffic` to `0.0` and can also yield negative traffic with a negative `anomaly_multiplier`.
- `tests/` successfully execute standard scenarios.
- I wrote a stress test (`.agents/teamwork_preview_reviewer_m1_2/stress_test.py`) injecting `-50.0` traffic into a Node, which resulted in the Node's expected traffic (EMA) becoming negative.

## 2. Logic Chain
1. The project simulates network traffic which inherently represents a non-negative volume or rate.
2. `LoadGenerator` contains a logic bug where applying `anomaly_multiplier` after the `max(0.0, traffic)` check allows the generation of negative traffic if configured with a negative multiplier.
3. `generate_deterministic` does not validate or clamp `base_traffic` or its output, allowing negative values.
4. `Node.receive_traffic` lacks boundary checks. When fed negative traffic, it reduces the `current_traffic` sum.
5. Consequently, `Node.step()` updates the Exponential Moving Average (`expected_traffic`) to negative values and misinterprets the negative difference as a reason to "throttle".
6. To ensure system robustness for M2 and beyond, these bounds must be explicitly enforced.

## 3. Caveats
- No integrity violations were found. The implementations are genuine and not hardcoded dummies.
- The `alpha` value in `Node.step` is hardcoded to `0.2` and the `surprise` threshold is hardcoded to `10.0`. While not inherently buggy, these might need parameterization in future milestones for easier tuning.

## 4. Conclusion
**Verdict**: REQUEST_CHANGES

**Major Findings:**
1. **Missing bounds on `Node.receive_traffic`**: It must raise a `ValueError` or clamp if `amount < 0.0`.
2. **Bug in `LoadGenerator.generate`**: Clamping `max(0.0, traffic)` must happen *after* applying `anomaly_multiplier`, or `anomaly_multiplier` must be validated as `>= 0`.
3. **Bug in `LoadGenerator.generate_deterministic`**: Needs to enforce `traffic >= 0.0` before returning.

Please add validation and clamping logic to ensure traffic is strictly non-negative across the system.

## 5. Verification Method
- Execute the stress test created in `.agents/teamwork_preview_reviewer_m1_2/stress_test.py` via `$env:PYTHONPATH="."; pytest .agents/teamwork_preview_reviewer_m1_2/stress_test.py`. It currently passes, proving the negative traffic vulnerabilities exist.
- After fixing, the `Node` test should fail (e.g., throwing a `ValueError`), and the `LoadGenerator` test should yield clamped `0.0` traffic instead of `-50.0`.
- Standard project tests can be run via `$env:PYTHONPATH="."; pytest tests/`.
