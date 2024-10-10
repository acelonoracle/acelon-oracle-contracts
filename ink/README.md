# Acelon Oracle Ink smart contract

This smart contract implements the logic to receive price updated from [Acelon](https://acelon.io/) oracles.

The contract is written using [ink!](https://use.ink/).

## Build

### Prerequisites

Please follow the [ink! Setup](https://use.ink/getting-started/setup) steps.

### Build contract

```shell
cargo contract build --release
```

The command above will generate the contract WASM code and metadata.

## Tests

```shell
cargo test
```

## Deploy

For detailed steps, please see the [Deploy Your Contract](https://use.ink/getting-started/deploy-your-contract) page on ink!.

A convenient way is to use the ink! [UI web application](https://ui.use.ink/).

## Usage

The core functionality the contract offers is the `update_price_feeds` call, which takes as input a list of updates and corresponding signatures that have been produced by the Acelon oracles.

See the test at the bottom of [lib.rs](lib.rs) for an example.

The contract also stores a list of valid signers (Oracle addresses) and certificate hashes, only prices provided by the those Oracles (singers) and certificates will be accepted.
