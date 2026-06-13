# Handoff Report: Tier 4 Workload Tests Strategy

## 1. Observation
- `TEST_INFRA.md` specifies an opaque-box test architecture using `pytest` to call `python main.py` and parsing its stdout/stderr JSON logs for specific behaviors (F1-F5).
- `TEST_INFRA.md` lines 27-34 define exactly 5 Tier 4 real-world application scenarios:
  1. Baseline steady traffic followed by gradual increase (F1, F4)
  2. Sudden traffic spike causing spawning (F1, F2, F4)
  3. Asymmetric load causing topology re-wiring (F1, F3, F4)
  4. Massive injected anomaly requiring full recovery (F1, F2, F3, F4, F5)
  5. Sustained high load with intermittent spikes (F1, F2, F3, F4, F5)
- `SCOPE.md` outlines that `tests/e2e/conftest.py` will expose `run_origin_core(args)` and `parse_logs(stdout)`.
- Examination of `tests/e2e/conftest.py` confirms these helpers exist and expect `main.py` to output JSON lines for parsing.

## 2. Logic Chain
- Because tests are opaque-box and must simulate workloads without knowing internal implementation, we must instruct `main.py` to execute these scenarios via CLI arguments.
- A logical API design is to pass a `--workload` flag (e.g., `--workload sudden_spike`) to the `main.py` script. 
- Each test case corresponds to one of the 5 scenarios. It invokes `run_origin_core(["--workload", "<scenario>"])` and extracts the JSON logs using `parse_logs()`.
- The tests then assert that the process completes successfully (`returncode == 0`) and that specific JSON log events exist indicating the required feature behaviors (e.g., `event: "node_spawned"`, `event: "topology_rewired"`, `event: "homeostasis_recovered"`).

## 3. Caveats
- The exact CLI argument structure for `main.py` is not yet strictly defined. I have proposed `--workload <scenario>`. The implementer or `main.py` developers will need to align on this or adapt the tests.
- The exact JSON schema for logs is not defined yet. I assumed a simple `{"event": "some_event", ...}` schema. This will need to be matched by the core implementation.

## 4. Conclusion
I have proposed an implementation strategy where 5 pytest functions invoke `run_origin_core` with distinct `--workload` scenarios and verify JSON event logs. The proposed code for `tests/e2e/test_tier4_workload.py` has been drafted and is available at `c:\Users\ahmad ali\OneDrive\Desktop\Project Origin\origin_core\.agents\tier4_explorer\proposed_test_tier4_workload.py`.

## 5. Verification Method
- **Syntax Check**: Run `pytest --collect-only tests/e2e/test_tier4_workload.py` after the implementer copies the file. It should successfully collect 5 tests without syntax errors.
- **Test Execution**: Run `pytest tests/e2e/test_tier4_workload.py`. The tests should fail as expected, since `main.py` and its workload configurations do not exist yet.
