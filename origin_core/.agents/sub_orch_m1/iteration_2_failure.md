# Iteration 2 Failure Report

## Verdict
Iteration 2 failed the gate check.

- **Auditor**: CLEAN.
- **Reviewers 1 & 2**: APPROVE.
- **Challengers 1 & 2**: FOUND VULNERABILITIES.

### Vulnerabilities Found:
1. **Float Overflow Bricking**: `Node` overflows to `inf` during massive traffic accumulation.
2. **Negative Threshold Exact-Match Bug**: `Node` lacks bounds checking on `surprise_threshold` and `ratio`. If negative, perfectly matching traffic (surprise=0) triggers an action.
3. **Negative Anomaly Sign Flip**: `LoadGenerator` applies `anomaly_multiplier` incorrectly. A negative noise combined with negative multiplier flips to a huge positive spike.
4. **Missing NaN/Inf Init Validation**: `Node` accepts NaN for state variables upon initialization.
5. **Probability Bounds**: `LoadGenerator` accepts `anomaly_prob` > 1.0 or < 0.0 without validation.
6. **Variance vs StdDev**: `LoadGenerator` passes `variance` directly to `random.gauss()` instead of standard deviation (sigma), making actual variance $V^2$.

## Objective for Iteration 3
Address the above 6 vulnerabilities in `src/node.py` and `src/load_generator.py`. Ensure that `tests/` covers these extreme edge cases.
