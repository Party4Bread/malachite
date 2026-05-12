#!/usr/bin/env bash
# Capture a criterion baseline against which future runs can be compared.
#
# Usage:
#   scripts/save-baseline.sh <baseline-name>
#
# The recommended workflow is:
#   1. On main/before changes:   scripts/save-baseline.sh main
#   2. After changes:            COMPARE_TO=main scripts/bench.sh
#
# criterion stores baselines under target/criterion/<bench>/<group>/<id>/<baseline>/.

set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "usage: $0 <baseline-name> [bench-name [filter]]" >&2
    exit 2
fi

BASELINE="$1"
shift

BASELINE="$BASELINE" exec "$(dirname "$0")/bench.sh" "$@"
