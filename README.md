# Advent of Code 2024

My solutions for [Advent of Code 2024](https://adventofcode.com/2024), done in Rust.

Each day has two parts (earlier days in `dayXYa` and `dayXYb`, later days in `dayXY/src/part_1.rs`
and `dayXY/src/part_2.rs`). Sometimes the two parts are very similar with only minor modifications, other times the
changes are more invasive. I sometimes come back to solutions after they're completed to optimize them or to apply
something new that I've learned so I can remember it in the future; you can view the original solution for each day in
the git history.

## CLI

A basic CLI is provided in [aoc.sh](aoc.sh) for configuring project directories. The first time the script is run, it
will ask for a session token which gets stored in `.env` (used to fetch inputs). The session cookie can be found by
logging in to https://adventofcode.com/ and using the dev tools to either view the request headers when loading a page,
or by viewing cookies directly under the Application/Storage section of the dev tools.

To create project directories for a new day, run eg:

```bash
./aoc.sh new 1
```

Inputs are `.gitignore`'d. To download inputs for an existing day's projects, run eg:

```bash
./aoc.sh download 1
```

Examples aren't automatically downloaded, so you'll have to copy those manually into `example.txt`.

## Running a Day

The code for a day is run through cargo like normal, there's nothing special tying the projects together.

For earlier days (day01a, day01b, etc):

```bash
# Download inputs if not already present
./aoc.sh download 1

cd day01a

cargo run
# or for the days where I lazily use a brute force solution 
# cargo run --release
```

For later days that use a single project with two parts:

```bash
./aoc.sh downlad 12

cd day12

# For example input
cargo run -- 1 example

# For actual input
cargo run -- 1 input
```
