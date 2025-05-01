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

# Temp files for storing results
OPTIMISED_RESULTS=$(mktemp)
UNOPTIMISED_RESULTS=$(mktemp)
EXAMPLES_LIST=$(mktemp)

# Function to run a single test with or without optimisations
run_test() {
    local example_file="$1"
    local optimised="$2"
    local results_file="$3"
    local filename=$(basename "$example_file" .b)
    local input_file="$INPUT_DIR/$filename.in"
    local expected_file="$EXPECTED_DIR/$filename.out"
    local output_file="$(mktemp)"

    # Determine if there is an input file to provide as stdin
    if [[ -f "$input_file" ]]; then
        input_redirect="< \"$input_file\""
    else
        input_redirect=""
    fi

    # Set or unset NO_OPT environment variable based on optimised flag
    if [[ "$optimised" == "false" ]]; then
        export NO_OPT=1
        echo -n "$filename (unoptimised): "
    else
        unset NO_OPT
        echo -n "$filename (optimised): "
    fi

    start_time=$(date +%s.%N)
    if eval "$BINARY \"$example_file\" $input_redirect" > "$output_file" 2> "$output_file.stderr"; then
        end_time=$(date +%s.%N)
    else
        end_time=$(date +%s.%N)
        echo -e "\033[91mERROR\033[0m"
        cat "$output_file.stderr"
        rm "$output_file" "$output_file.stderr"
        # Mark as failed in results
        echo "$filename -1" >> "$results_file"
        return
    fi

    # Calculate execution time
    elapsed_time=$(printf "%.4f" "$(echo "$end_time - $start_time" | bc -l)")

    # Compare the output with the expected output
    if [[ -f "$expected_file" ]]; then
        if diff -q "$output_file" "$expected_file" > /dev/null; then
            echo -e "\033[92m\033[1mSUCCESS\033[0m \033[92m(Time: ${elapsed_time}s)\033[0m"
            # Write results to file: filename elapsed_time
            echo "$filename $elapsed_time" >> "$results_file"
        else
            echo -e "\033[91m\033[1mFAILURE\033[0m \033[91m(Time: ${elapsed_time}s)\033[0m"
            echo "  Output differs from expected:"
            diff "$output_file" "$expected_file"
            # Mark as failed in results
            echo "$filename -1" >> "$results_file"
        fi
    else
        echo -e "\033[91mFAILURE\033[0m (Time: ${elapsed_time}s)"
        echo "  Expected output file '$expected_file' not found."
        # Mark as failed in results
        echo "$filename -1" >> "$results_file"
    fi

    # Clean up temporary files
    rm "$output_file" "$output_file.stderr"
}

export -f run_test
export BINARY INPUT_DIR EXPECTED_DIR

# Find all brainfuck example files and store them
find "$EXAMPLES_DIR" -name "*.b" | sort > "$EXAMPLES_LIST"

echo "Running tests without optimisations..."

cat "$EXAMPLES_LIST" | parallel run_test {} false "$UNOPTIMISED_RESULTS"

echo -e "\nRunning tests with optimisations..."

cat "$EXAMPLES_LIST" | parallel run_test {} true "$OPTIMISED_RESULTS"


# Generate markdown table
echo -e "\n\n| Example        | Unoptimised | Optimised | Improvement factor |"
echo "|----------------|-------------|-----------|--------------------|"

# Process results and generate table rows
while read example_file; do
    filename=$(basename "$example_file" .b)
    unopt_time=$(grep "^$filename " "$UNOPTIMISED_RESULTS" | awk '{print $2}')
    opt_time=$(grep "^$filename " "$OPTIMISED_RESULTS" | awk '{print $2}')
    
    # Skip if either test failed
    if [[ "$unopt_time" == "-1" || "$opt_time" == "-1" || -z "$unopt_time" || -z "$opt_time" ]]; then
        continue
    fi
    
    # Calculate improvement factor
    improvement=$(printf "%.2f" "$(echo "$unopt_time / $opt_time" | bc -l)")
    
    # Format the table row
    printf "| %-14s | %-11s | %-9s | %-18s |\n" "\`$filename.b\`" "${unopt_time}s" "${opt_time}s" "${improvement}x"
done < "$EXAMPLES_LIST"

# Clean up temp files
rm "$OPTIMISED_RESULTS" "$UNOPTIMISED_RESULTS" "$EXAMPLES_LIST"
