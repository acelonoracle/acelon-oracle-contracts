#![cfg(test)]

use frame_support::{assert_ok, sp_runtime::AccountId32};
use hex_literal::hex;

use crate::{mock::*, types::*};

#[test]
fn test_update_price_feeds() {
    ExtBuilder.build().execute_with(|| {
        let oracle: AccountId = sp_io::hashing::blake2_256(
            hex!("03b689085552418111b9c8a6683e470c83f16f79a96c8f5fe35df1a3ab0e24d020").as_slice(),
        )
        .into();
        assert_ok!(Acelon::update_trusted_signer(
            RuntimeOrigin::root(),
            TrustedSignerUpdate {
                operation: ListUpdateOperation::Add,
                item: oracle,
            },
        ));

        assert_ok!(Acelon::update_certificate_trust_store(
            RuntimeOrigin::root(),
            CertificateTrustStoreUpdate {
                operation: ListUpdateOperation::Add,
                item: hex!("ef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e")
            }
        ));

        assert_ok!(Acelon::update_certificate_trust_store(
            RuntimeOrigin::root(),
            CertificateTrustStoreUpdate {
                operation: ListUpdateOperation::Add,
                item: hex!("4795062d13e1ed971c6b6e5699764681e4d090bad39a7ef367cc9cb705652384")
            }
        ));

        assert_ok!(Acelon::update_certificate_trust_store(
            RuntimeOrigin::root(),
            CertificateTrustStoreUpdate {
                operation: ListUpdateOperation::Add,
                item: hex!("b75b095dc5d2a59f082ef17d0c64104aaeab321bb0a49a26b9b27a59792fce80")
            }
        ));

        assert_ok!(Acelon::update_signers_threshold(RuntimeOrigin::root(), 1));
        assert_ok!(Acelon::update_sources_threshold(RuntimeOrigin::root(), 1));

        assert_ok!(Acelon::update_price_feeds(
            RuntimeOrigin::signed(AccountId32::new([0u8; 32])), 
            vec![hex!("0440cf669e0e0000000000000000000000b3285d6c920100000cef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e4795062d13e1ed971c6b6e5699764681e4d090bad39a7ef367cc9cb705652384b75b095dc5d2a59f082ef17d0c64104aaeab321bb0a49a26b9b27a59792fce80dd450ee6601d873d2d381d738d87a131f5605ec30c6adbc9138b79f2dfd6f3dc").to_vec()].try_into().unwrap(),
            vec![vec![hex!("147124904bd67756d1b5efecd302e01c87639cab15a6b05f5913877c0d55c5787b676599461f0da5fe4c3fd02981a1216cd226b767c2f0ef464bbed26c72fa221b").into()]].try_into().unwrap(),
        ));
    });
}
