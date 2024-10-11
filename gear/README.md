# Acelon Oracle Gear smart contract

This smart contract implements the logic to receive price updated from [Acelon](https://acelon.io/) oracles.

The contract is written using [Sails and Gear](https://wiki.vara.network/docs/build), which is the smart contract technology supported by the [Vara Network](https://vara.network/).

## Build

### Prerequisites

Please follow the [Vara setup guide](https://wiki.vara.network/docs/getting-started-in-5-minutes).

### Build contract

```shell
cargo build --release
```

The command above will generate the contract WASM code and metadata.

## Tests

```shell
cargo test
```

## Deploy

For detailed steps, please see the [Upload program](https://wiki.vara.network/docs/build/deploy) page on the Vara wiki.

## Usage

The core functionality the contract offers is the `update_price_feeds` call, which takes as input a list of updates and corresponding signatures that have been produced by the Acelon oracles.

See the test in [gtest.rs](tests/gtest.rs) for an example.

The contract also stores a list of valid signers (Oracle addresses) and certificate hashes, only prices provided by the those Oracles (singers) and certificates will be accepted.
