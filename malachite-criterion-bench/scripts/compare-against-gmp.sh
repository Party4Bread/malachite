#!/usr/bin/env bash
# Extract per-size malachite-vs-rug (GMP) ratios from criterion JSON output.
#
# Run a bench first (e.g. `scripts/bench.sh natural_mul`), then this script
# walks target/criterion/ and prints a table:
#
#   bench / size / malachite ns / rug ns / ratio (malachite / rug)
#
# Requires `jq`.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
CRITERION_DIR="$REPO_ROOT/target/criterion"

if ! command -v jq >/dev/null 2>&1; then
    echo "error: jq not installed" >&2
    exit 1
fi
if [[ ! -d "$CRITERION_DIR" ]]; then
    echo "error: no criterion results at $CRITERION_DIR. Run scripts/bench.sh first." >&2
    exit 1
fi

printf '%-44s %-10s %14s %14s %10s\n' "GROUP" "SIZE" "MALACHITE_NS" "RUG_NS" "RATIO"
printf '%-44s %-10s %14s %14s %10s\n' "-----" "----" "------------" "------" "-----"

# Each group directory has one subdir per (impl, size) like "malachite/1024".
while IFS= read -r est_file; do
    impl_size="${est_file#$CRITERION_DIR/}"
    impl_size="${impl_size%/new/estimates.json}"
    group="$(dirname "$impl_size")"
    leaf="$(basename "$impl_size")"
    case "$leaf" in
        malachite|num|rug) continue ;;
    esac
done < /dev/null

# Iterate groups, find malachite & rug entries at the same size.
for group_dir in "$CRITERION_DIR"/*/; do
    group="$(basename "$group_dir")"
    [[ "$group" == "report" ]] && continue
    # gather sizes that exist for both malachite and rug
    mapfile -t mal_dirs < <(find "$group_dir" -maxdepth 2 -type d -name "malachite" 2>/dev/null || true)
    for mal_dir in "${mal_dirs[@]:-}"; do
        [[ -z "$mal_dir" ]] && continue
        for size_dir in "$mal_dir"/*/; do
            size="$(basename "$size_dir")"
            mal_est="$size_dir/new/estimates.json"
            rug_est="$group_dir/rug/$size/new/estimates.json"
            if [[ -f "$mal_est" && -f "$rug_est" ]]; then
                mal_ns=$(jq -r '.median.point_estimate' "$mal_est")
                rug_ns=$(jq -r '.median.point_estimate' "$rug_est")
                ratio=$(awk "BEGIN{printf \"%.3f\", $mal_ns/$rug_ns}")
                printf '%-44s %-10s %14.0f %14.0f %10s\n' "$group" "$size" "$mal_ns" "$rug_ns" "$ratio"
            fi
        done
    done
done
