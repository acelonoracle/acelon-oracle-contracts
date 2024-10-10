# Acelon Oracle substrate pallet

This pallet implements the logic to receive price updated from [Acelon](https://acelon.io/) oracles.

# Chain configuration

Add the pallet to the runtime by listing it in the `contrusct_runtime!` macro and implementing the `Config`:

```rust
use pallet_acelon_oracle::types::{CU32, Public, Signature};

impl crate::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxPrices = CU32<50>;
    type MaxCertificates = CU32<50>;
    type MaxPriceUpdates = CU32<10>;
    type Signature = Signature;
    type Public = Public;
    type WeightInfo = ();
}
```

## Usage

The core functionality the pallet offers is the `update_price_feeds` extrinsic, which takes as input a list of updates and corresponding signatures that have been produced by the Acelon oracles.

See the test in [tests.rs](src/tests.rs) for an example.

The pallet also stores a list of valid signers (Oracle addresses) and certificate hashes, only prices provided by the those Oracles (singers) and certificates will be accepted.
```
