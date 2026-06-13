# E2E Test Infra: Project Origin Core

## Test Philosophy
- Opaque-box, requirement-driven. No dependency on implementation design.
- Methodology: Category-Partition + BVA + Pairwise + Workload Testing.

## Feature Inventory
| # | Feature | Source (requirement) | Tier 1 | Tier 2 | Tier 3 |
|---|---------|---------------------|:------:|:------:|:------:|
| 1 | Network Initialization (>= 5 nodes) | ORIGINAL_REQUEST §R1, Acceptance | 5      | 5      | ✓      |
| 2 | Free Energy Minimization (Spawn/Throttle) | ORIGINAL_REQUEST §R2, Acceptance | 5      | 5      | ✓      |
| 3 | Constructal Topology Morphing (Re-wiring) | ORIGINAL_REQUEST §R3, Acceptance | 5      | 5      | ✓      |
| 4 | Simulated Load Processing | ORIGINAL_REQUEST Acceptance        | 5      | 5      | ✓      |
| 5 | Homeostasis / Recovery from Anomaly | ORIGINAL_REQUEST Acceptance        | 5      | 5      | ✓      |

## Test Architecture
- Test runner: `pytest`
- Test case format: Python `subprocess` calls to `main.py` (or importing its CLI wrapper), verifying stdout/logs and return codes.
- Expected to run: `python main.py` with possible CLI arguments if provided, or capturing standard default output. Since the problem explicitly mentions "The terminal output definitively logs...", tests will capture and parse stdout/stderr for specific log signatures indicating required behaviors.
- Directory layout:
  - `tests/e2e/conftest.py` (fixtures and test runner utilities)
  - `tests/e2e/test_tier1_feature.py`
  - `tests/e2e/test_tier2_boundary.py`
  - `tests/e2e/test_tier3_pairwise.py`
  - `tests/e2e/test_tier4_workload.py`

## Real-World Application Scenarios (Tier 4)
| # | Scenario | Features Exercised | Complexity |
|---|----------|--------------------|------------|
| 1 | Baseline steady traffic followed by gradual increase | F1, F4 | Low |
| 2 | Sudden traffic spike causing spawning | F1, F2, F4 | Medium |
| 3 | Asymmetric load causing topology re-wiring | F1, F3, F4 | Medium |
| 4 | Massive injected anomaly requiring full recovery | F1, F2, F3, F4, F5 | High |
| 5 | Sustained high load with intermittent spikes | F1, F2, F3, F4, F5 | High |

## Coverage Thresholds
- Tier 1: ≥5 per feature
- Tier 2: ≥5 per feature (where boundaries exist)
- Tier 3: pairwise coverage of major feature interactions
- Tier 4: ≥5 realistic application scenarios
