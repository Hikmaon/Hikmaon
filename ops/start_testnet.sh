#!/usr/bin/env bash
set -euo pipefail

COMPOSE_FILE="${COMPOSE_FILE:-docker-compose.yml}"

echo "Starting Hikmalayer testnet..."
docker compose -f "${COMPOSE_FILE}" up -d --build

echo "Waiting for nodes to boot..."
sleep 5

ADMIN_TOKEN="${ADMIN_TOKEN:-local-admin}"
P2P_TOKEN="${P2P_TOKEN:-local-testnet}"

VALIDATOR_KEY_PRIV="0000000000000000000000000000000000000000000000000000000000000001"
VALIDATOR_KEY_PUB="0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"

function seed_validator() {
  local node_url=$1
  local address=$2

  curl -s -X POST "${node_url}/tokens/transfer" \
    -H "Content-Type: application/json" \
    -d "{\"from\":\"admin\",\"to\":\"${address}\",\"amount\":100}" >/dev/null

  curl -s -X POST "${node_url}/staking/deposit" \
    -H "Content-Type: application/json" \
    -d "{\"address\":\"${address}\",\"amount\":100,\"public_key\":\"${VALIDATOR_KEY_PUB}\",\"private_key\":\"${VALIDATOR_KEY_PRIV}\"}" >/dev/null

  curl -s -X POST "${node_url}/p2p/peers/register" \
    -H "Content-Type: application/json" \
    -H "x-p2p-token: ${P2P_TOKEN}" \
    -d "{\"address\":\"http://bootnode:3000\"}" >/dev/null || true
}

seed_validator "http://localhost:3000" "validator-boot"
seed_validator "http://localhost:3001" "validator-1"
seed_validator "http://localhost:3002" "validator-2"
seed_validator "http://localhost:3003" "validator-3"
seed_validator "http://localhost:3004" "validator-4"

echo "Testnet started."
