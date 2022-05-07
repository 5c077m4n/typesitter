#!/bin/sh -eu

wasm-pack build --release --target web "$(git rev-parse --show-toplevel)/crates/utils/js_bind" -- --features demo
