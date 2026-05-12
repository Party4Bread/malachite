#!/usr/bin/env bash
# Compare two RUSTFLAGS configurations side-by-side for the same bench.
#
# Usage:
#   scripts/compare-rustflags.sh <bench> <baseline-name> <baseline-rustflags> <candidate-name> <candidate-rustflags> [filter]
#
# Example:
#   scripts/compare-rustflags.sh mpn_primitives stock "" bmi2_adx "-C target-feature=+bmi2,+adx"
#
# Saves two criterion baselines and prints a comparison table.

set -euo pipefail

if [[ $# -lt 5 ]]; then
    cat >&2 <<'USAGE'
usage: scripts/compare-rustflags.sh BENCH BASE_NAME BASE_RUSTFLAGS CAND_NAME CAND_RUSTFLAGS [FILTER]

Each <name> is a criterion baseline label. Each <RUSTFLAGS> is the value passed
as RUSTFLAGS for that run (may be empty string for stock).
USAGE
    exit 2
fi

BENCH="$1"
BASE_NAME="$2"
BASE_FLAGS="$3"
CAND_NAME="$4"
CAND_FLAGS="$5"
FILTER="${6:-}"

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$REPO_ROOT"

PIN=()
command -v taskset >/dev/null 2>&1 && PIN=(taskset -c 0)

# Tighter timing so two passes finish in reasonable wall time.
CRIT_TIME=(--sample-size 15 --warm-up-time 1 --measurement-time 2)

run_one() {
    local name="$1" flags="$2"
    echo
    echo "=== Run: $name  (RUSTFLAGS='$flags') ==="
    RUSTFLAGS="$flags" "${PIN[@]}" cargo bench -p malachite-criterion-bench --bench "$BENCH" -- \
        --save-baseline "$name" "${CRIT_TIME[@]}" $FILTER
}

run_one "$BASE_NAME" "$BASE_FLAGS"
run_one "$CAND_NAME" "$CAND_FLAGS"

echo
echo "=== Comparison: $BASE_NAME vs $CAND_NAME ==="
python3 - "$REPO_ROOT/target/criterion" "$BASE_NAME" "$CAND_NAME" <<'PY'
import json, os, sys
root, base, cand = sys.argv[1], sys.argv[2], sys.argv[3]
rows = []
for group in sorted(os.listdir(root)):
    gdir = os.path.join(root, group)
    if not os.path.isdir(gdir) or group == "report":
        continue
    # walk to leaves containing both baselines
    for dirpath, dirnames, _ in os.walk(gdir):
        if base in dirnames and cand in dirnames:
            with open(os.path.join(dirpath, base, "estimates.json")) as f:
                b = json.load(f)["median"]["point_estimate"]
            with open(os.path.join(dirpath, cand, "estimates.json")) as f:
                c = json.load(f)["median"]["point_estimate"]
            label = os.path.relpath(dirpath, root)
            rows.append((label, b, c))

rows.sort(key=lambda r: r[0])
print(f"{'BENCH':<78} {'BASE_ns':>12} {'CAND_ns':>12} {'SPEEDUP':>9}")
for label, b, c in rows:
    speedup = b / c if c > 0 else float('inf')
    print(f"{label:<78} {b:>12.1f} {c:>12.1f} {speedup:>8.2f}x")
PY
