#!/bin/bash

# Get the directory where the script is located
ROOT_DIR="$(dirname "$(realpath "$0")")"

function get_session_token() {
    if [ -f "${ROOT_DIR}/.env" ]; then
        source "${ROOT_DIR}/.env"
    fi

    if [ -z "$AOC_SESSION" ]; then
        echo "AOC session token not found in .env"
        echo -n "Please enter your session token (input will be hidden): "
        read -r -s session_token
        echo  # New line after hidden input

        # Write to .env, creating it if it doesn't exist
        if [ ! -f "${ROOT_DIR}/.env" ]; then
            echo "AOC_SESSION=$session_token" > "${ROOT_DIR}/.env"
        else
            # Check if AOC_SESSION already exists in file (might not if it was set manually)
            if ! grep -q "^AOC_SESSION=" "${ROOT_DIR}/.env"; then
                echo "AOC_SESSION=$session_token" >> "${ROOT_DIR}/.env"
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
    input_dir="${ROOT_DIR}/inputs"
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
            rm -f "$input_file"
            return 1
        else
            echo "Successfully downloaded input for day ${day_no_padding}"
        fi
    fi
    return 0
}

function make_day_part() {
  day=$(printf "%02d" "$1")
  part=$2

  name="day${day}${part}"

  if [ "$(pwd)" != "${ROOT_DIR}" ]; then
    cd "${ROOT_DIR}" || exit 1
  fi

  cd "${ROOT_DIR}" || exit 1
  cargo new "$name"
  touch "$name/example.txt"

  input_file="${ROOT_DIR}/inputs/day${day}.txt"
  if [ -f "${input_file}" ]; then
    cp "${ROOT_DIR}/inputs/day${day}.txt" "$name/input.txt"
  else
    touch "$name/input.txt"
  fi

  cp "${ROOT_DIR}/template/src/main.rs" "$name/src/main.rs"
}

function make_day() {
  day="$1"

  fetch_input "$day"
  if [ $? -ne 0 ]; then
    echo "Input files will be empty, please fill manually"
  fi

  make_day_part "$day" "a"
  make_day_part "$day" "b"
}

function download_inputs() {
  day="$1"
  day_padded=$(printf "%02d" "$day")

  if [ ! -d "${ROOT_DIR}/day${day_padded}a" ] || [ ! -d "${ROOT_DIR}/day${day_padded}b" ]; then
    echo "Error: Day ${day} directories not found. Create them first with '$0 new ${day}'"
    exit 1
  fi

  fetch_input "$day"
  if [ $? -ne 0 ]; then
    exit 1
  fi

  cp "${ROOT_DIR}/inputs/day${day_padded}.txt" "${ROOT_DIR}/day${day_padded}a/input.txt"
  cp "${ROOT_DIR}/inputs/day${day_padded}.txt" "${ROOT_DIR}/day${day_padded}b/input.txt"
}

function usage() {
  echo "Advent of Code CLI

Usage: $0 <command> <day>

Commands:
  new <day>         Create new project directories for the specified day (parts 1 and 2)
  download <day>    Download input for the specified day (directories must exist)

Examples:
  $0 new 1    Create day01a and day01b projects, downloading inputs
  $0 input 1  Download input for into pre-existing day01a and day01b projects
"
}

case "$1" in
  "new")
    if [ -z "$2" ]; then
      echo "Error: Day number required"
      usage
      exit 1
    fi
    make_day "$2"
    ;;
  "download")
    if [ -z "$2" ]; then
      echo "Error: Day number required"
      usage
      exit 1
    fi
    download_inputs "$2"
    ;;
  *)
    usage
    exit 1
    ;;
esac