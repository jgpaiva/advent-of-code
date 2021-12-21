# Advent of code
My Rust🦀 solutions for coding puzzles at https://adventofcode.com/

Currently features some solutions for the years [2021](src/year_2021) and [2018](src/year_2018). Each of these folders has several of files, each corresponding to a day in that year. Each file has one publicly exported function for each of the two parts of the daily puzzles.

## Running

To run all of the efficiently implemented solutions, run `cargo run`. To run all solutions, edit [src/main.rs](src/main.rs), change `skip_slow` to `false`, run `cargo run`, and go make a cup of tea while you wait because some of the solutions take a while to run.

## Developing

To run the tests, run `cargo test`. Or even better, install [cargo watch](https://crates.io/crates/cargo-watch) and run `cargo watch -x 'test -- --nocapture' -x 'run --release' -x clippy`; this command will do everything you need automatically when you modify/create any file in the project.