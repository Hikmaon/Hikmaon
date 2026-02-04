#!/usr/bin/env bash
set -euo pipefail

COMPOSE_FILE="${COMPOSE_FILE:-docker-compose.yml}"

docker compose -f "${COMPOSE_FILE}" down -v
rm -rf data/state.json
echo "Chain state reset."
