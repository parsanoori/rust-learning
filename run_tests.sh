#!/usr/bin/env bash
# Runs the test.sh of every problem folder (or only the given ones).
# Usage: ./run_tests.sh [problem ...]

root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
problems=("$@")
if [ ${#problems[@]} -eq 0 ]; then
    for d in "$root"/*/; do
        [ -x "$d/test.sh" ] && problems+=("$(basename "$d")")
    done
fi

failed=0
for p in "${problems[@]}"; do
    echo "== $p"
    "$root/$p/test.sh" || failed=1
done
exit $failed
