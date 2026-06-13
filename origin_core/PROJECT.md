# Project: Origin Core

## Architecture
- **Node (`src/node.py`)**: Implements Cellular Biology (Markov blankets) and Free Energy Principle (minimizing traffic surprise). Each node models expected traffic, calculates "surprise" (free energy) on deviation, and takes autonomous action (throttle/spawn).
- **Network (`src/network.py`)**: Implements Constructal Law. Manages connections between nodes, assesses data flow and latency/resistance, and dynamically rewires topology to optimize flow.
- **Load Generator (`src/load_generator.py`)**: Generates and injects varying traffic patterns and anomalies.
- **Main (`src/main.py`)**: Orchestrates the 5-node setup, simulation loop, anomaly injection, and verifiable logging.

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | Core Node & Traffic Sim | `Node` class with traffic prediction, "surprise" detection, and autonomous actions. `LoadGenerator` basic implementation. | none | IN_PROGRESS |
| 2 | Network Topology Morphing | `Network` class managing nodes, calculating resistance/latency, and dynamically re-wiring topology. | M1 | PLANNED |
| 3 | Integration & Main | `main.py` setup with 5 nodes, varying traffic, massive anomaly injection, and homeostasis recovery logging. | M1, M2 | PLANNED |

## Interface Contracts
### `node.py` ↔ `network.py`
- `Node` exposes methods to receive traffic: `receive_traffic(source_id, amount)`.
- `Node` exposes its prediction error (free energy): `get_surprise()`.
- `Network` maintains an adjacency list/graph of connections and routes traffic between connected nodes.
- `Network` evaluates edge "resistance" based on traffic volume and dynamically adds/removes edges.

### `main.py` ↔ `network.py`, `load_generator.py`
- `Main` initializes `Network(num_nodes=5)`.
- `Main` uses `LoadGenerator` to apply traffic to specific nodes in the `Network`.
- `Main` triggers simulation steps: `network.step()`, which internally calls `node.step()`.

## Code Layout
- `src/node.py`
- `src/network.py`
- `src/load_generator.py`
- `src/main.py`
- `tests/`
