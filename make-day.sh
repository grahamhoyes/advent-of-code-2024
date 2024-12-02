#!/bin/bash

function get_session_token() {
    if [ -f .env ]; then
        source .env
    fi

    if [ -z "$AOC_SESSION" ]; then
        echo "AOC session token not found in .env"
        echo -n "Please enter your session token (input will be hidden): "
        read -s session_token
        echo  # New line after hidden input

        # Write to .env, creating it if it doesn't exist
        if [ ! -f .env ]; then
            echo "AOC_SESSION=$session_token" > .env
        else
            # Check if AOC_SESSION already exists in file (might not if it was set manually)
            if ! grep -q "^AOC_SESSION=" .env; then
                echo "AOC_SESSION=$session_token" >> .env
            fi
        fi

        # Set for current session
        export AOC_SESSION=$session_token
    fi
}

# Fetch input, storing it in the inputs/ directory (.gitignore'd)
function fetch_input() {
    day_no_padding="$1"
    day=$(printf "%02d" "$day_no_padding")
    input_dir="inputs"
    mkdir -p "$input_dir"
    input_file="${input_dir}/day${day}.txt"

    # Only download if we don't already have it
    if [ ! -f "$input_file" ]; then
        get_session_token  # Ensure we have a session token

        # Fetch the input using curl
        http_status=$(curl -s -w "%{http_code}" -o "$input_file" \
            -H "Cookie: session=${AOC_SESSION}" \
            "https://adventofcode.com/2024/day/${day_no_padding}/input")

        if [ "$http_status" -ne 200 ]; then
            echo "Failed to fetch input for day ${day_no_padding} (HTTP ${http_status})"
            # Create an empty input file for manual filling
            rm -f "$input_file"
            touch "$input_file"
            return 1
        else
            echo "Successfully downloaded input for day ${day_no_padding}"
        fi
    fi
    return 0
}

function make_day() {
  day=$(printf "%02d" "$1")
  part=$2

  name="day${day}${part}"

  cargo new "$name"
  touch "$name/example.txt"
  cp "inputs/day${day}.txt" "$name/input.txt"

  cp "template/src/main.rs" "$name/src/main.rs"
}

fetch_input "$1"

make_day "$1" "a"
make_day "$1" "b"
