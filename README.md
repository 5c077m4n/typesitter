# Typesitter

[![CI](https://github.com/5c077m4n/typesitter/actions/workflows/ci.yaml/badge.svg)](https://github.com/5c077m4n/typesitter/actions/workflows/ci.yaml)

A native [typescript](https://www.typescriptlang.org/) runtime ([demo](https://5c077m4n.github.io/typesitter/))

## Usage

-   Custom interpreter:
    -   `cargo run`
-   LLVM:
-   Make sure that LLVM is installed on your system and that the param `LLVM_SYS_<version>0_PREFIX` is exported by your shell (my setup is using v14 since it's the latest one supported by the [`inkwell`](https://thedan64.github.io/inkwell/inkwell/index.html) crate)
    -   `cargo run --no-default-features --features llvm`

## Feature table (`vm` feature only for now)

| Feature                                     | Done | Todo | In progress |
| ------------------------------------------- | :--: | :--: | :---------: |
| `const`/`let` assignment (f64 only for now) |  x   |      |             |
| `console.log`/`console.error`               |  x   |      |             |
| Aggregate errors                            |  x   |      |             |
| Boolean/unary operators                     |      |      |      x      |
| Functions                                   |      |  x   |             |
| Passing raw values into function calls      |      |  x   |             |
| Support more literal (primitive) types      |      |  x   |             |
| Support arrays/objects                      |      |  x   |             |
| Everything else...                          |      |  x   |             |
