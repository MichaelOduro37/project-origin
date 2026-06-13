# Scope: Milestone 1 - Core Node & Traffic Sim

## Architecture
- **Node (`src/node.py`)**: Implements Cellular Biology (Markov blankets) and Free Energy Principle (minimizing traffic surprise). Each node models expected traffic, calculates "surprise" (free energy) on deviation, and takes autonomous action (throttle/spawn).
- **Load Generator (`src/load_generator.py`)**: Generates and injects varying traffic patterns and anomalies.

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | Core Node & Traffic Sim | `Node` class with traffic prediction, "surprise" detection, and autonomous actions. `LoadGenerator` basic implementation. | none | IN_PROGRESS |

## Interface Contracts
### `node.py` ↔ `network.py`
- `Node` exposes methods to receive traffic: `receive_traffic(source_id, amount)`.
- `Node` exposes its prediction error (free energy): `get_surprise()`.

### `main.py` ↔ `network.py`, `load_generator.py`
- `LoadGenerator` basic implementation for applying traffic patterns.

## Code Layout
- `src/node.py`
- `src/load_generator.py`
- `tests/test_node.py`
- `tests/test_load_generator.py`
