# To make executable: chmod +x program2_tester.sh
# To run: ./program2_tester.sh program2.c program2.rs

#!/bin/bash

set -e

N=1000000
SEED=42
RUNS=50

INPUT_FILE="prog2_input.txt"

C_FILE=$1
RUST_FILE=$2

# -----------------------------
# Generate deterministic input
# -----------------------------
echo "Generating input file..."

awk -v seed="$SEED" -v n="$N" '
BEGIN {
    srand(seed);
    print n;
    for (i = 1; i <= n; i++) {
        print int(rand() * 1000000);
    }
}
' > "$INPUT_FILE"

echo "Input written to $INPUT_FILE ($N values)"

# -----------------------------
# Compile programs
# -----------------------------
echo "Compiling C program..."
gcc -O3 "$C_FILE" -o c_prog

echo "Compiling Rust program..."
rustc -O "$RUST_FILE" -o rust_prog

# # -----------------------------
# # Warmup run (optional but useful)
# # -----------------------------
# echo "Warmup run (not measured)..."
# ./c_prog < "$INPUT_FILE" > /dev/null
# ./rust_prog "$INPUT_FILE" > /dev/null

# -----------------------------
# Benchmark function
# -----------------------------
run_bench() {
    local label=$1
    local cmd=$2

    echo ""
    echo "$label:"

    total=0

    for ((i=1; i<=RUNS; i++)); do
        start=$(date +%s%N)

        eval "$cmd" > /dev/null

        end=$(date +%s%N)

        elapsed=$((end - start))
        elapsed_ms=$((elapsed / 1000000))

        echo "Run $i: ${elapsed_ms} ms"

        total=$((total + elapsed_ms))
    done

    avg=$((total / RUNS))
    echo "Average: ${avg} ms"
}

# -----------------------------
# Run benchmarks
# -----------------------------
run_bench "C program" "./c_prog $INPUT_FILE"
run_bench "Rust program" "./rust_prog $INPUT_FILE"