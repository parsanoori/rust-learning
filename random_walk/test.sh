#!/usr/bin/env bash
# Tests random_walk against in/inputN.txt and out/outputN.txt.
# Usage: ./test.sh   (can be run from anywhere)

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
name="random_walk"
limit=10   # time limit in seconds

# Output is random, so tester.cpp (exit 0 = pass) judges it instead of an exact diff.
cd "$dir" && cargo build --release -q || { echo "build failed"; exit 1; }
mkdir -p "$dir/target"
g++ -O2 -o "$dir/target/tester" "$dir/tester.cpp" || { echo "tester build failed"; exit 1; }
bin="$dir/target/release/$name"

pass=0; total=0
for input in $(ls "$dir"/in/input*.txt | sort -V); do
    n="${input##*/input}"; n="${n%.txt}"
    expected="$dir/out/output$n.txt"
    total=$((total + 1))
    timeout "$limit" "$bin" < "$input" > "$dir/target/actual.txt" 2>&1
    if "$dir/target/tester" "$input" "$expected" "$dir/target/actual.txt"; then
        pass=$((pass + 1))
    else
        echo "FAIL test $n"
        echo "  input:    $(tr "\n" " " < "$input")"
        echo "  expected: $(tr "\n" " " < "$expected")"
        echo "  got:      $(tr "\n" " " < "$dir/target/actual.txt")"
    fi
done
echo "$name: $pass/$total passed"
[ "$pass" -eq "$total" ]
