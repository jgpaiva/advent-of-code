#!/bin/sh

cargo test  -- --nocapture && cargo run --release && cargo clippy
