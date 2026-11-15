#!/usr/bin/env bash
# Test a Rust problem against its in/ and out/ test cases.
# Usage: ./run_tests.sh [problem ...]   (default: every problem that has an in/ folder)
# Output is compared ignoring trailing whitespace/newlines, unless the problem
# ships a tester.cpp (called as: tester input expected actual, exit 0 = pass).
# Per-problem time limits (seconds) default to 1; see TIME_LIMITS below.

root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
problems=("$@")
if [ ${#problems[@]} -eq 0 ]; then
    for d in "$root"/*/; do
        [ -d "$d/in" ] && problems+=("$(basename "$d")")
    done
fi

declare -A TIME_LIMITS=( [random_walk]=10 )

failed=0
for p in "${problems[@]}"; do
    dir="$root/$p"
    [ -d "$dir/in" ] || { echo "$p: no in/ folder, skipping"; continue; }
    echo "== $p"
    if ! (cd "$dir" && cargo build --release -q 2>&1); then
        echo "build failed"; failed=1; continue
    fi
    bin="$dir/target/release/$p"
    limit="${TIME_LIMITS[$p]:-1}"
    tester=""
    if [ -f "$dir/tester.cpp" ]; then
        tester="$dir/target/tester"
        if ! g++ -O2 -o "$tester" "$dir/tester.cpp"; then
            echo "tester build failed"; failed=1; continue
        fi
    fi
    pass=0; total=0
    for input in $(ls "$dir"/in/input*.txt | sort -V); do
        n="${input##*/input}"; n="${n%.txt}"
        expected="$dir/out/output$n.txt"
        total=$((total + 1))
        actual="$(timeout "$limit" "$bin" < "$input" 2>&1)"
        if [ -n "$tester" ]; then
            printf '%s\n' "$actual" > "$dir/target/actual.txt"
            "$tester" "$input" "$expected" "$dir/target/actual.txt"; ok=$?
        else
            [ "$actual" == "$(cat "$expected")" ]; ok=$?
        fi
        if [ $ok -eq 0 ]; then
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
