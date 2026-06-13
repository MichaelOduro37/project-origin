# Handoff Report: Milestone 1 Implementation Strategy

## 1. Observation
- The target implementation files `src/node.py` and `src/load_generator.py` do not currently exist in the codebase, nor does the `src` directory.
- The target test files `tests/test_node.py` and `tests/test_load_generator.py` also do not currently exist.
- `PROJECT.md` and `SCOPE.md` define Milestone 1: "Core Node & Traffic Sim".
- `node.py` must define a `Node` class that predicts expected traffic, calculates "surprise" (free energy) on deviation, takes autonomous action (throttle/spawn), and exposes `receive_traffic(source_id, amount)` and `get_surprise()`.
- `load_generator.py` must define a `LoadGenerator` class that generates and injects varying traffic patterns and anomalies.
- `TEST_INFRA.md` mandates `pytest` as the test runner. 

## 2. Logic Chain
1. Since the source and test files do not exist, Milestone 1 requires a completely new implementation from scratch following the layout specified in `PROJECT.md`.
2. To satisfy the `Node` requirements:
   - `Node` needs an initialization parameter for its identity (`node_id`).
   - It requires internal state to track a history of received traffic to compute an expected baseline (e.g., via a simple moving average).
   - `receive_traffic` must accumulate incoming traffic for the current step.
   - A `step()` method is needed to finalize the current step's traffic, update the moving average, calculate `surprise` (absolute difference between expected and actual traffic), reset the step's traffic counter, and trigger autonomous actions (like logging a throttle or spawn event when surprise exceeds a threshold).
   - `get_surprise()` must return the most recently calculated surprise value.
3. To satisfy the `LoadGenerator` requirements:
   - `LoadGenerator` needs a method (e.g., `generate(step)`) that produces baseline traffic patterns (e.g., sine wave or random noise around a mean).
   - It also needs a mechanism to inject anomalies (e.g., sudden spikes at specific steps or random intervals).
4. For tests (`tests/test_node.py` and `tests/test_load_generator.py`):
   - Unit tests using `pytest` should verify the mathematical correctness of surprise calculation, accumulation of traffic, and correct anomaly generation.

## 3. Caveats
- Since the exact mathematical model for "Cellular Biology (Markov blankets) and Free Energy Principle" is not strictly defined in the requirements beyond "expected traffic" and "minimizing traffic surprise", a basic moving-average-based expectation model is assumed to be sufficient for Milestone 1.
- The interface `receive_traffic(source_id, amount)` includes `source_id`, suggesting the node might eventually track surprise per neighbor, but for M1, a global surprise per node is sufficient.

## 4. Conclusion
Implement `src/node.py` and `src/load_generator.py` with the following proposed structures:

**`src/node.py`**:
```python
class Node:
    def __init__(self, node_id):
        self.node_id = node_id
        self.expected_traffic = 0.0
        self.current_traffic = 0.0
        self.surprise = 0.0
        self.history = []

    def receive_traffic(self, source_id, amount):
        self.current_traffic += amount

    def step(self):
        # Calculate surprise (Free Energy)
        self.surprise = abs(self.current_traffic - self.expected_traffic)
        
        # Autonomous action based on surprise
        if self.surprise > 10.0:  # arbitrary threshold for spawning/throttling
            pass # trigger action
            
        # Update expectation (e.g., Simple Moving Average)
        self.history.append(self.current_traffic)
        if len(self.history) > 5:
            self.history.pop(0)
        self.expected_traffic = sum(self.history) / len(self.history)
        
        # Reset for next step
        self.current_traffic = 0.0

    def get_surprise(self):
        return self.surprise
```

**`src/load_generator.py`**:
```python
import random

class LoadGenerator:
    def __init__(self, base_load=10, anomaly_chance=0.05, anomaly_multiplier=5):
        self.base_load = base_load
        self.anomaly_chance = anomaly_chance
        self.anomaly_multiplier = anomaly_multiplier

    def generate(self):
        load = self.base_load + random.uniform(-2, 2)
        if random.random() < self.anomaly_chance:
            load *= self.anomaly_multiplier
        return max(0, load)
```

Also, create corresponding `pytest` files in `tests/test_node.py` and `tests/test_load_generator.py`. Ensure the `src` directory is created.

## 5. Verification Method
- Run `pytest tests/test_node.py tests/test_load_generator.py` to ensure unit tests pass.
- Verify `src/node.py` and `src/load_generator.py` exist and adhere to the project layout specified in `PROJECT.md`.
