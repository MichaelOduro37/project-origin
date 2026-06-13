# Scope: E2E Test Suite Creation

## Architecture
- Module boundaries: test suite is structured by tiers under `tests/e2e/`.
- `conftest.py` will contain test harnesses for running `main.py` via `subprocess` and parsing logs.

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | Tier 1 Tests | 25 feature coverage tests (F1-F5) | none | DONE |
| 2 | Tier 2 Tests | 25 boundary/corner tests (F1-F5) | none | DONE |
| 3 | Tier 3 Tests | 5 cross-feature tests | none | DONE |
| 4 | Tier 4 Tests | 5 real-world workload tests | none | DONE |

## Interface Contracts
### conftest.py ↔ test scripts
- `run_origin_core(args: List[str]) -> subprocess.CompletedProcess`
- `parse_logs(stdout: str) -> List[Dict[str, Any]]`
