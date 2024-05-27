#!/usr/bin/env bash
set -e

cargo +nightly rustc --release --target=wasm32-unknown-unknown \
    -- -C target-feature=+multivalue

cp target/wasm32-unknown-unknown/release/lo.wasm .
