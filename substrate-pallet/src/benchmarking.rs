use frame_benchmarking::{benchmarks, whitelist_account};
use frame_support::traits::Get;
use frame_system::RawOrigin;
use hex_literal::hex;
use sp_std::prelude::*;

use super::{
    types::{CertificateTrustStoreUpdate, ListUpdateOperation, TrustedSignerUpdate},
    *,
};

benchmarks! {
    where_clause { where
        <T as frame_system::Config>::AccountId: From<[u8; 32]>,
        <T as Config>::Signature: From<[u8; 65]>,
    }

    update_price_feeds {
        let x in 1 .. T::MaxPriceUpdates::get();
        let caller: T::AccountId = [0u8; 32].into();
        whitelist_account!(caller);
        let mut updates = Vec::<Vec<u8>>::new();
        let mut signatures = Vec::<Vec<T::Signature>>::new();
        let oracle: T::AccountId = sp_io::hashing::blake2_256(
            hex!("03b689085552418111b9c8a6683e470c83f16f79a96c8f5fe35df1a3ab0e24d020").as_slice(),
        ).into();
        Pallet::<T>::update_trusted_signer(RawOrigin::Root.into(), TrustedSignerUpdate::<T::AccountId> { operation: ListUpdateOperation::Add, item: oracle })?;
        Pallet::<T>::update_certificate_trust_store(RawOrigin::Root.into(), CertificateTrustStoreUpdate { operation: ListUpdateOperation::Add, item: hex!("ef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e") })?;
        Pallet::<T>::update_certificate_trust_store(RawOrigin::Root.into(), CertificateTrustStoreUpdate { operation: ListUpdateOperation::Add, item: hex!("4795062d13e1ed971c6b6e5699764681e4d090bad39a7ef367cc9cb705652384") })?;
        Pallet::<T>::update_certificate_trust_store(RawOrigin::Root.into(), CertificateTrustStoreUpdate { operation: ListUpdateOperation::Add, item: hex!("b75b095dc5d2a59f082ef17d0c64104aaeab321bb0a49a26b9b27a59792fce80") })?;
        for i in 0..x {
            updates.push(hex!("0440cf669e0e0000000000000000000000b3285d6c920100000cef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e4795062d13e1ed971c6b6e5699764681e4d090bad39a7ef367cc9cb705652384b75b095dc5d2a59f082ef17d0c64104aaeab321bb0a49a26b9b27a59792fce80dd450ee6601d873d2d381d738d87a131f5605ec30c6adbc9138b79f2dfd6f3dc").to_vec());
            signatures.push(vec![hex!("147124904bd67756d1b5efecd302e01c87639cab15a6b05f5913877c0d55c5787b676599461f0da5fe4c3fd02981a1216cd226b767c2f0ef464bbed26c72fa221b").into()]);
        }
    }: _(RawOrigin::Signed(caller), updates.try_into().unwrap(), signatures.try_into().unwrap())

    update_signers_threshold {
        let new_threshold: u8 = 10;
    }: _(RawOrigin::Root, new_threshold)

    update_sources_threshold {
        let new_threshold: u8 = 10;
    }: _(RawOrigin::Root, new_threshold)

    update_valid_time_period {
        let new_time_period: u64 = 3_600_000;
    }: _(RawOrigin::Root, new_time_period)

    update_trusted_signer {
        let oracle: T::AccountId = sp_io::hashing::blake2_256(
            hex!("03b689085552418111b9c8a6683e470c83f16f79a96c8f5fe35df1a3ab0e24d020").as_slice(),
        ).into();
        let update = TrustedSignerUpdate::<T::AccountId> { operation: ListUpdateOperation::Add, item: oracle };
    }: _(RawOrigin::Root, update)

    update_certificate_trust_store {
        let update = CertificateTrustStoreUpdate { operation: ListUpdateOperation::Add, item: hex!("ef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e") };
    }: _(RawOrigin::Root, update)

    impl_benchmark_test_suite!(Pallet, mock::ExtBuilder::default().build(), mock::Test);
}
