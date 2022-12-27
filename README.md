# juno-rust

Rust libraries for Juno. The following table shows every published crates maintained in this repository:

| Crate                                             | Description                                                                                                                                                            | Crates.io                                                                                                                                 | Docs                                                                                        |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| [juno-rust-proto](juno-rust-proto)                | Juno's proto-generated types and helpers for interacting with the appchain. Compatible with CosmWasm contract.                                                         | [![juno-rust-proto on crates.io](https://img.shields.io/crates/v/juno-rust-proto.svg)](https://crates.io/crates/juno-rust-proto)                      | [![Docs](https://docs.rs/juno-rust-proto/badge.svg)](https://docs.rs/juno-rust-proto)               |

---

## Building Proto files from source

The single proto-build crate in this repo clones and rebuilds proto files for
all other crates, simply make the required edits in [main.rs](proto-build/main.rs) and run

    cd proto-build && cargo run

## Minimum Supported Rust Version

Rust **1.59**