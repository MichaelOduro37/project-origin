import pytest
import subprocess
from typing import List, Dict, Any
import json

@pytest.fixture
def run_origin_core():
    def _run_origin_core(args: List[str]) -> subprocess.CompletedProcess:
        cmd = ["python", "origin_core/src/main.py"] + args
        return subprocess.run(
            cmd,
            capture_output=True,
            text=True
        )
    return _run_origin_core

@pytest.fixture
def parse_logs():
    def _parse_logs(stdout: str) -> List[Dict[str, Any]]:
        logs = []
        for line in stdout.splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                logs.append(json.loads(line))
            except json.JSONDecodeError:
                logs.append({"message": line})
        return logs
    return _parse_logs
