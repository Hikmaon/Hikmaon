# Benchmarking Hikmalayer (Phase 4)

This guide provides benchmark commands for local testnet runs using Docker Compose and the
benchmark harness in `/bench`.

## Prerequisites

- Docker + Docker Compose
- Python 3

## Start the testnet

```bash
./ops/start_testnet.sh
```

## Quick 10‑minute run

```bash
./ops/run_benchmark.sh 600
```

## 1‑hour run

```bash
./ops/run_benchmark.sh 3600
```

## 72‑hour endurance run

```bash
./ops/run_benchmark.sh 259200
```

## Stop or reset the testnet

```bash
./ops/stop_testnet.sh
```

```bash
./ops/reset_chain.sh
```

Benchmark outputs are saved under `bench/results/`:

- `benchmark_report.json`
- `benchmark_report.csv`
- `benchmark_report.md`
