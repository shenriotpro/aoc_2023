# aoc_2023

An attempt at https://adventofcode.com/2023 in Rust.

## Setup

`curl -b $(cat session.txt) https://adventofcode.com/2023/day/1/input > data/day01_input.txt`

`cp src/bin/template src/bin/day01.rs`

## Checking

`cargo fmt`

`cargo clippy --all-targets`

`cargo test --release`

## Running

`cargo run --bin day01 --release`