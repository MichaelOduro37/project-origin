import pytest
import subprocess
import sys
import json
import re

@pytest.fixture
def run_origin_core():
    """Returns a function to run the main.py script with given arguments."""
    def _run_origin_core(args: list[str]) -> subprocess.CompletedProcess:
        return subprocess.run(
            [sys.executable, "src/main.py"] + args,
            capture_output=True,
            text=True
        )
    return _run_origin_core

@pytest.fixture
def parse_logs():
    """Returns a function to parse logs from stdout."""
    def _parse_logs(stdout: str) -> list[dict]:
        logs = []
        for line in stdout.splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                logs.append(json.loads(line))
            except json.JSONDecodeError:
                entry = {"raw": line}
                for match in re.finditer(r'(\w+)=([^\s]+)', line):
                    entry[match.group(1)] = match.group(2)
                logs.append(entry)
        return logs
    return _parse_logs
