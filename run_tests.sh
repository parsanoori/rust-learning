#!/usr/bin/env bash
# Test a Rust problem against its in/ and out/ test cases.
# Usage: ./run_tests.sh [problem ...]   (default: every problem that has an in/ folder)
# Output is compared ignoring trailing whitespace/newlines.

root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
problems=("$@")
if [ ${#problems[@]} -eq 0 ]; then
    for d in "$root"/*/; do
        [ -d "$d/in" ] && problems+=("$(basename "$d")")
    done
fi

failed=0
for p in "${problems[@]}"; do
    dir="$root/$p"
    [ -d "$dir/in" ] || { echo "$p: no in/ folder, skipping"; continue; }
    echo "== $p"
    if ! (cd "$dir" && cargo build --release -q 2>&1); then
        echo "build failed"; failed=1; continue
    fi
    bin="$dir/target/release/$p"
    pass=0; total=0
    for input in $(ls "$dir"/in/input*.txt | sort -V); do
        n="${input##*/input}"; n="${n%.txt}"
        expected="$dir/out/output$n.txt"
        total=$((total + 1))
        actual="$(timeout 1 "$bin" < "$input" 2>&1)"
        if [ "$actual" == "$(cat "$expected")" ]; then
            pass=$((pass + 1))
        else
            failed=1
            echo "  FAIL test $n"
            echo "    input:    $(tr '\n' ' ' < "$input")"
            echo "    expected: $(tr '\n' ' ' < "$expected")"
            echo "    got:      $(echo "$actual" | tr '\n' ' ')"
        fi
    done
    echo "  $pass/$total passed"
done
exit $failed
