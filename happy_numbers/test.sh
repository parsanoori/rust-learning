#!/usr/bin/env bash
# Tests happy_numbers against in/inputN.txt and out/outputN.txt.
# Usage: ./test.sh   (can be run from anywhere)

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
name="happy_numbers"
limit=1   # time limit in seconds

cd "$dir" && cargo build --release -q || { echo "build failed"; exit 1; }
bin="$dir/target/release/$name"

pass=0; total=0
for input in $(ls "$dir"/in/input*.txt | sort -V); do
    n="${input##*/input}"; n="${n%.txt}"
    expected="$dir/out/output$n.txt"
    total=$((total + 1))
    actual="$(timeout "$limit" "$bin" < "$input" 2>&1)"
    # compare ignoring trailing whitespace/newlines
    if [ "$actual" == "$(cat "$expected")" ]; then
        pass=$((pass + 1))
    else
        echo "FAIL test $n"
        echo "  input:    $(tr "\n" " " < "$input")"
        echo "  expected: $(tr "\n" " " < "$expected")"
        echo "  got:      $(echo "$actual" | tr "\n" " ")"
    fi
done
echo "$name: $pass/$total passed"
[ "$pass" -eq "$total" ]
