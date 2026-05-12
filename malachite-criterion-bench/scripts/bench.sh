#!/usr/bin/env bash
# Run malachite-criterion-bench benchmarks reproducibly.
#
# Usage:
#   scripts/bench.sh                  # run all benches
#   scripts/bench.sh natural_mul      # run a single bench
#   scripts/bench.sh mpn_primitives addmul_1   # filter inside one bench
#
# Environment variables:
#   RUSTFLAGS_EXTRA  - appended to RUSTFLAGS (e.g. "-C target-cpu=native")
#   BASELINE         - if set, runs `--save-baseline $BASELINE`
#   COMPARE_TO       - if set, runs `--baseline $COMPARE_TO`

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$REPO_ROOT"

# Default to native CPU codegen for the benchmark binary so we measure what
# the hardware can actually do, not a generic x86-64-v1 baseline.
: "${RUSTFLAGS_EXTRA:=-C target-cpu=native}"
export RUSTFLAGS="${RUSTFLAGS:-} ${RUSTFLAGS_EXTRA}"

CARGO_ARGS=(bench -p malachite-criterion-bench)
CRITERION_ARGS=()

if [[ -n "${BASELINE:-}" ]]; then
    CRITERION_ARGS+=(--save-baseline "$BASELINE")
fi
if [[ -n "${COMPARE_TO:-}" ]]; then
    CRITERION_ARGS+=(--baseline "$COMPARE_TO")
fi

if [[ $# -ge 1 ]]; then
    CARGO_ARGS+=(--bench "$1")
    shift
fi

# Disable parallel benching: shared CPU caches and thread migration distort
# measurements. Pin to a single core if `taskset` is available.
PIN=()
if command -v taskset >/dev/null 2>&1; then
    PIN=(taskset -c 0)
fi

echo "RUSTFLAGS=$RUSTFLAGS"
echo "Running: ${PIN[*]} cargo ${CARGO_ARGS[*]} -- ${CRITERION_ARGS[*]} $*"
"${PIN[@]}" cargo "${CARGO_ARGS[@]}" -- "${CRITERION_ARGS[@]}" "$@"
