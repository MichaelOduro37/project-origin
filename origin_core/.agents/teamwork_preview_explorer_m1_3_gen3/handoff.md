# Handoff Report: Iteration 2 Failure Fix Strategy (Milestone 1)

## 1. Observation
The codebase currently contains 6 specific vulnerabilities across `src/node.py` and `src/load_generator.py` that were identified during Iteration 2:
- **`src/node.py`**:
  - `receive_traffic` accumulates `amount` via `self.current_traffic += amount` without checking for overflow (lines 19-22).
  - `__init__` does not validate whether `surprise_threshold` and `surprise_ratio` are negative (lines 7-13).
  - `__init__` does not reject `NaN` or `Inf` for its state variables.
- **`src/load_generator.py`**:
  - `generate` applies `traffic *= self.anomaly_multiplier` before ensuring `traffic` is non-negative, allowing a negative `traffic` (from `random.gauss`) to combine with a negative `anomaly_multiplier` to create a massive positive spike (lines 18-24).
  - `__init__` accepts `anomaly_prob` outside the `[0.0, 1.0]` bound (lines 7-11).
  - `generate` passes `self.variance` as the second argument to `random.gauss(self.base_traffic, self.variance)`, which actually expects standard deviation ($\sigma$) (line 18).

## 2. Logic Chain
1. **Float Overflow Bricking**: When `self.current_traffic` reaches near `1.79e308` and more traffic is added, it overflows to `inf`. Subsequent calculations with `inf` (e.g., `abs(inf - expected_traffic)`) propagate `inf` through the model.
2. **Negative Threshold Exact-Match Bug**: If `surprise_threshold` and `surprise_ratio` are negative, `threshold` becomes negative. Even if `surprise == 0`, `0 > threshold` evaluates to `True`, inappropriately triggering a node action ("spawn" or "throttle").
3. **Negative Anomaly Sign Flip**: `random.gauss` can mathematically produce negative values. Multiplying a negative `traffic` by a negative `anomaly_multiplier` yields a positive anomaly spike, bypassing the intended noise logic. Clamping `traffic` to `0.0` *before* multiplication prevents this.
4. **Missing NaN/Inf Init Validation**: Initializing `Node` with `math.nan` or `math.inf` poisons the state immediately, leading to cascading failures during `step()`.
5. **Probability Bounds**: `random.random() < self.anomaly_prob` is nonsensical for bounds `< 0.0` or `> 1.0`. It should be constrained.
6. **Variance vs StdDev**: `random.gauss(mu, sigma)` expects $\sigma$. Using variance ($\sigma^2$) directly means the generated spread is massively exaggerated compared to what was configured.

## 3. Caveats
- No checks are currently made to limit `base_traffic` to `>= 0`, but the generated traffic is correctly clamped at the end. The fix simply applies a pre-clamp to prevent the sign-flip anomaly.
- Overflow checks use `math.isinf()`. Python floats do not throw `OverflowError` strictly upon addition unless checking `isinf()` manually.

## 4. Conclusion
The implementation needs to be fortified with early validation and correct math operations. 

**Code Fixes for `src/node.py`**:
```python
# In __init__:
import math
if any(math.isnan(x) or math.isinf(x) for x in [expected_traffic, surprise_threshold, surprise_ratio]):
    raise ValueError("State variables cannot be NaN or Inf")
if surprise_threshold < 0 or surprise_ratio < 0:
    raise ValueError("surprise_threshold and surprise_ratio must be >= 0")

# In receive_traffic:
new_traffic = self.current_traffic + amount
if math.isinf(new_traffic):
    raise OverflowError("Traffic accumulation exceeded maximum limit")
self.current_traffic = new_traffic
```

**Code Fixes for `src/load_generator.py`**:
```python
# In __init__:
if not (0.0 <= anomaly_prob <= 1.0):
    raise ValueError("anomaly_prob must be between 0.0 and 1.0")
if variance < 0.0:
    raise ValueError("variance cannot be negative")

# In generate():
import math
traffic = max(0.0, random.gauss(self.base_traffic, math.sqrt(self.variance)))
is_anomaly = random.random() < self.anomaly_prob
if is_anomaly:
    traffic *= self.anomaly_multiplier
return max(0.0, traffic)

# In generate_deterministic():
traffic = max(0.0, self.base_traffic)
if step == anomaly_step:
    traffic *= self.anomaly_multiplier
return max(0.0, traffic)
```

## 5. Verification Method
To guarantee these fixes, the edge cases must be explicitly covered in `tests/test_node.py` and `tests/test_load_generator.py`. After the implementer writes the fixes, run `pytest tests/` to confirm all tests pass.

**Proposed Test Cases for `test_node.py`**:
```python
def test_node_init_nan_inf():
    import math
    with pytest.raises(ValueError):
        Node("n1", expected_traffic=math.nan)

def test_node_negative_thresholds():
    with pytest.raises(ValueError):
        Node("n1", surprise_threshold=-5.0)

def test_node_overflow_traffic():
    node = Node("n1")
    node.receive_traffic("src_1", 1e308)
    with pytest.raises(OverflowError):
        node.receive_traffic("src_1", 1e308)
```

**Proposed Test Cases for `test_load_generator.py`**:
```python
def test_load_generator_prob_bounds():
    with pytest.raises(ValueError):
        LoadGenerator(anomaly_prob=-0.1)

def test_load_generator_negative_variance():
    with pytest.raises(ValueError):
        LoadGenerator(variance=-1.0)

def test_generate_negative_noise_flip():
    # Negative multiplier on negative noise should not create a spike
    gen = LoadGenerator(base_traffic=0.0, variance=100.0, anomaly_prob=1.0, anomaly_multiplier=-10.0)
    for _ in range(100):
        # The pre-clamp ensures noise is 0 before multiplier, yielding 0 traffic.
        assert gen.generate() == 0.0 
```
