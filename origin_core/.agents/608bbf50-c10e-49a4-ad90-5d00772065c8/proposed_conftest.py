import pytest
import subprocess
import sys
import json
import re

@pytest.fixture
def run_origin_core():
    def _run_origin_core(args: list[str]) -> subprocess.CompletedProcess:
        """Runs the main.py script with the given arguments."""
        return subprocess.run(
            [sys.executable, "src/main.py"] + args,
            capture_output=True,
            text=True
        )
    return _run_origin_core

@pytest.fixture
def parse_logs():
    def _parse_logs(stdout: str) -> list[dict]:
        """Parses logs from stdout. Handles JSON or key=value regex."""
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
