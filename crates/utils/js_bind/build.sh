#!/bin/sh -eu

wasm-pack build --target nodejs "$(git rev-parse --show-toplevel)/crates/utils/js_bind"
