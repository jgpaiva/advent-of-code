#!/bin/sh

cargo test  -- --nocapture && cargo run && cargo clippy
