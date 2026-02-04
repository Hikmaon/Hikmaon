#!/usr/bin/env python3
import argparse
import csv
import json
import os
import subprocess
import time
from collections import defaultdict
from urllib import request


def post_json(url, payload):
    data = json.dumps(payload).encode("utf-8")
    req = request.Request(url, data=data, headers={"Content-Type": "application/json"})
    start = time.time()
    with request.urlopen(req, timeout=10) as resp:
        resp.read()
    end = time.time()
    return end - start


def get_json(url):
    with request.urlopen(url, timeout=10) as resp:
        return json.loads(resp.read().decode("utf-8"))


def docker_stats(container_names):
    stats = {}
    try:
        output = subprocess.check_output(
            ["docker", "stats", "--no-stream", "--format", "{{.Name}},{{.CPUPerc}},{{.MemUsage}}"]
            + container_names,
            text=True,
        )
        for line in output.strip().splitlines():
            name, cpu, mem = line.split(",", 2)
            stats[name] = {"cpu": cpu, "mem": mem}
    except Exception:
        pass
    return stats


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-url", required=True)
    parser.add_argument("--duration", type=int, default=600)
    parser.add_argument("--rate", type=int, default=50)
    parser.add_argument("--output", default="bench/results")
    args = parser.parse_args()

    os.makedirs(args.output, exist_ok=True)

    latencies = []
    tx_count = 0
    start_time = time.time()
    next_mine = start_time
    mine_interval = 2

    accounts = [f"bench-{i}" for i in range(10)]
    for account in accounts:
        post_json(f"{args.base_url}/tokens/transfer", {"from": "admin", "to": account, "amount": 100})

    while time.time() - start_time < args.duration:
        for _ in range(args.rate):
            from_acct = accounts[tx_count % len(accounts)]
            to_acct = accounts[(tx_count + 1) % len(accounts)]
            latency = post_json(
                f"{args.base_url}/tokens/transfer",
                {"from": from_acct, "to": to_acct, "amount": 1},
            )
            latencies.append(latency)
            tx_count += 1
        if time.time() >= next_mine:
            try:
                post_json(f"{args.base_url}/mine", {})
            except Exception:
                pass
            next_mine = time.time() + mine_interval

    elapsed = time.time() - start_time
    tps = tx_count / elapsed if elapsed > 0 else 0
    avg_latency = sum(latencies) / len(latencies) if latencies else 0

    stats = get_json(f"{args.base_url}/blockchain/stats")
    metrics = get_json(f"{args.base_url}/metrics")

    docker_metrics = docker_stats(
        [
            "hikmalayer-bootnode",
            "hikmalayer-validator1",
            "hikmalayer-validator2",
            "hikmalayer-validator3",
            "hikmalayer-validator4",
        ]
    )

    report = {
        "duration_seconds": args.duration,
        "tx_count": tx_count,
        "tps": tps,
        "avg_latency_seconds": avg_latency,
        "finalized_height": stats.get("finalized_height"),
        "blocks_mined": metrics.get("blocks_mined"),
        "blocks_received": metrics.get("blocks_received"),
        "docker_stats": docker_metrics,
        "reorg_count": 0,
        "notes": "Reorg count requires chain fork instrumentation.",
    }

    with open(os.path.join(args.output, "benchmark_report.json"), "w") as fp:
        json.dump(report, fp, indent=2)

    with open(os.path.join(args.output, "benchmark_report.csv"), "w", newline="") as fp:
        writer = csv.writer(fp)
        writer.writerow(["duration_seconds", "tx_count", "tps", "avg_latency_seconds"])
        writer.writerow([args.duration, tx_count, tps, avg_latency])

    with open(os.path.join(args.output, "benchmark_report.md"), "w") as fp:
        fp.write("# Benchmark Report\n\n")
        fp.write(f"- Duration: {args.duration} seconds\n")
        fp.write(f"- Transactions: {tx_count}\n")
        fp.write(f"- TPS: {tps:.2f}\n")
        fp.write(f"- Avg latency: {avg_latency:.4f} s\n")
        fp.write(f"- Finalized height: {stats.get('finalized_height')}\n")
        fp.write(f"- Blocks mined: {metrics.get('blocks_mined')}\n")
        fp.write(f"- Blocks received: {metrics.get('blocks_received')}\n")
        fp.write(f"- Reorg count: 0 (not instrumented)\n")


if __name__ == "__main__":
    main()
