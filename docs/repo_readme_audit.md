# README Consistency Audit

This document verifies whether repository contents align with the updated `README.md` benchmark claims and referenced artifact paths.

## Scope
- Checked tracked files from current branch using `git ls-files`.
- Reviewed `README.md` sections for:
  - Phase-4 benchmark metrics
  - Benchmark artifact path (`bench/results/run_10min/`)
  - Project directory tree

## Findings

### 1) Phase-4 benchmark metrics: ✅ Consistent
README states:
- Duration: **600s**
- Total transactions: **8,940**
- Throughput: **14.88 TPS**
- Latency: **~67 ms**
- Reorg count: **0**

`bench/results/run_10min/benchmark_report.json` confirms:
- `duration_seconds = 600`
- `tx_count = 8940`
- `tps = 14.881899...` (rounds to 14.88)
- `avg_latency_seconds = 0.066955...` (~67 ms)
- `reorg_count = 0`

`bench/results/run_10min/benchmark_report.md` also matches these rounded values.

### 2) Referenced benchmark artifact folder: ✅ Present
README points to `bench/results/run_10min/`, and the repository contains:
- `benchmark_report.json`
- `benchmark_report.csv`
- `benchmark_report.md`

### 3) README project tree block: ⚠️ Partially outdated
The tree is mostly representative, but not exact.

Examples of tracked files present in repo but not listed in the README tree:
- `docker-compose.yml`
- `Dockerfile`
- `dir` (tracked empty file)
- `docs/API.md`
- `docs/Whitepaper.md`
- `docs/whitepaper_short_version.md`

Examples listed in README tree as environment/generated directories (typically untracked):
- `node_modules/`
- `target/`
- `data/`

## Conclusion
- Benchmark data uploaded in the repository is present and aligned with the updated README metrics.
- If strict file-by-file parity is desired, refresh the README "Project directory" section to include all currently tracked docs/top-level files.
