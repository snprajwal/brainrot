#!/usr/bin/env bash

# Path to the brainfuck binary (first argument)
BINARY="$1"

# Hardcoded paths relative to the bench directory
BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXAMPLES_DIR="$BASE_DIR/../examples"
INPUT_DIR="$BASE_DIR/data"
EXPECTED_DIR="$BASE_DIR/expected"

# Check if the binary path is provided
if [[ -z "$BINARY" ]]; then
    echo "Usage: $0 /path/to/brainfuck"
    exit 1
fi

# Check if the binary exists
if [[ ! -x "$BINARY" ]]; then
    echo "Error: Binary '$BINARY' does not exist or is not executable."
    exit 1
fi

# Function to run a single test
run_test() {
    local example_file="$1"
    local filename=$(basename "$example_file" .b)
    local input_file="$INPUT_DIR/$filename.in"
    local expected_file="$EXPECTED_DIR/$filename.out"
    local output_file="$(mktemp)"

    echo -n "$filename: "

    # Determine if there is an input file to provide as stdin
    if [[ -f "$input_file" ]]; then
        input_redirect="< \"$input_file\""
    else
        input_redirect=""
    fi

    start_time=$(date +%s.%N)
    if eval "$BINARY \"$example_file\" $input_redirect" > "$output_file" 2> "$output_file.stderr"; then
        end_time=$(date +%s.%N)
    else
        end_time=$(date +%s.%N)
        echo -e "\033[91mERROR\033[0m"
        cat "$output_file.stderr"
        rm "$output_file" "$output_file.stderr"
        return
    fi

    # Calculate execution time
    elapsed_time=$(printf "%.4f" "$(echo "$end_time - $start_time" | bc -l)")

    # Compare the output with the expected output
    if [[ -f "$expected_file" ]]; then
        if diff -q "$output_file" "$expected_file" > /dev/null; then
            echo -e "\033[92m\033[1mSUCCESS\033[0m \033[92m(Time: ${elapsed_time}s)\033[0m"
        else
            echo -e "\033[91m\033[1mFAILURE\033[0m \033[91m(Time: ${elapsed_time}s)\033[0m"
            echo "  Output differs from expected:"
            diff "$output_file" "$expected_file"
        fi
    else
        echo -e "\033[91mFAILURE\033[0m (Time: ${elapsed_time}s)"
        echo "  Expected output file '$expected_file' not found."
    fi

    # Clean up temporary files
    rm "$output_file" "$output_file.stderr"
}

export -f run_test
export BINARY INPUT_DIR EXPECTED_DIR

# Find all brainfuck example files and run tests in parallel
find "$EXAMPLES_DIR" -name "*.b" | parallel run_test
