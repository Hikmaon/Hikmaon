#!/usr/bin/env bash
set -euo pipefail

DURATION_SECONDS="${1:-600}"
RATE="${RATE:-50}"
OUTPUT_DIR="${OUTPUT_DIR:-bench/results}"

python3 bench/benchmark.py \
  --base-url "http://localhost:3000" \
  --duration "${DURATION_SECONDS}" \
  --rate "${RATE}" \
  --output "${OUTPUT_DIR}"
