# juno-rust-proto

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Rust crate for interacting with [Protobufs] defined by the [Juno] blockchain.

The goal of this crate is to provide complete proto struct definitions for interacting
with a Cosmos SDK blockchain.

Currently, this crate only provides a subset of the many total structs exported by
Cosmos SDK proto files.

Pull requests to expand coverage are welcome.

[Documentation][docs-link]

## Minimum Supported Rust Version

This crate is supported on Rust **1.57** or newer.

[//]: # "badges"
[crate-image]: https://buildstats.info/crate/juno-rust-proto
[crate-link]: https://crates.io/crates/juno-rust-proto
[docs-image]: https://docs.rs/juno-rust-proto/badge.svg
[docs-link]: https://docs.rs/juno-rust-proto/
[build-image]: https://github.com/cosmos/cosmos-rust/workflows/juno-rust-proto/badge.svg
[build-link]: https://github.com/cosmos/cosmos-rust/actions/workflows/juno-rust-proto.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/cosmos/cosmos-rust/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.57+-blue.svg

[//]: # "links"
[Protobufs]: (https://github.com/CosmosContracts/juno/tree/main/proto)
[Juno]: https://github.com/CosmosContracts/juno
