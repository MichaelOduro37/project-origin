# MGCF Architecture Snapshot (archived)

Saved: 2026-06-08
Purpose: Preserve the latest MGCF v2/v3 architecture notes so we can iterate a low-cost variant while retaining the original design.

## Summary (archived)
- Core: WASM Cells executed in sandboxed runtimes, libp2p overlay for discovery and messaging, RocksDB-style stores for append-only event logs, BLS for threshold signatures.
- Key patterns: reversible event-sourcing, information-geometry telemetry, morphogenetic placement, mean-field control for autoscaling, reservoir forecasting, CRDT anti-entropy, proof-carrying manifests, and ZK/STARK-based verifiable compute.
- Business model: global marketplace for verifiable compute + data/model marketplaces, with auction-based pricing, gateway commissions, and enterprise premium SLAs.

## MGCF v2 highlights
- Placement: hybrid Optimal Transport (Sinkhorn) + VCG auctions for spot clearing.
- Observability: Information Bottleneck + compressed sensing + Fisher-weighted sampling.
- Trust: proof-carrying manifests and succinct proofs for verifiable compute.
- Phone support: WASM runtime for lightweight tasks, gateway-assisted heavy work, redundancy for correctness.

## MGCF v3 revision notes (short)
- Multi-tier provider model (phones → gateways → edge → cloud).
- Hybrid verification: on-phone quick checks, redundant execution, gateway-assisted succinct proofs.
- Adaptive placement: OT + mean-field + RL policies.
- Privacy-by-design: split-compute, encrypted pointers, threshold crypto for shared secrets.
- Phone-first optimizations: energy-aware scheduling, snapshotting, lightweight verification defaults.
- Economic model: continuous markets with VCG clearing for batches; reputation + staking.

## Where to find more
- Earlier additions and deep theory mapping are in `MASTER_COMPILATION.md` and `MASTER_COMPILATION_ADDITIONS.md` in this workspace.

---

Archive created by assistant to preserve architecture prior to designing MGCF-Lite (zero-cost variant).
