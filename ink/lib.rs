#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod traits;
mod types;

fn diff(a: u64, b: u64) -> u64 {
    if a > b {
        a.saturating_sub(b)
    } else {
        b.saturating_sub(a)
    }
}

#[ink::contract]
mod acelon_oracle {
    use ink::{env::hash::Blake2x256, prelude::vec::Vec, storage::Mapping};
    use scale::Decode;

    use crate::{
        traits::PriceOracle,
        types::{Certificate, Error, PriceEntry, PricePayload, RequestHash, Signature},
    };

    #[ink(event)]
    pub struct PriceFeedUpdate {
        request_hash: RequestHash,
        price_entry: PriceEntry,
    }

    #[ink(event)]
    pub struct SignersThresholdUpdate {
        new_threshold: u8,
    }

    #[ink(event)]
    pub struct SourcesThresholdUpdate {
        new_threshold: u8,
    }

    #[ink(event)]
    pub struct ValidTimePeriodUpdate {
        new_time_period: u64,
    }

    #[ink(event)]
    pub struct TrustedSignerAdded {
        new_trusted_signer: AccountId,
    }

    #[ink(event)]
    pub struct TrustedSignerRemoved {
        trusted_signer: AccountId,
    }

    #[ink(event)]
    pub struct CertificateAdded {
        new_certificate: Certificate,
    }

    #[ink(event)]
    pub struct CertificateRemoved {
        certificate: Certificate,
    }

    #[ink(event)]
    pub struct OwnerProposed {
        proposed_owner: AccountId,
    }

    #[ink(event)]
    pub struct OwnerAccepted;

    #[ink(event)]
    pub struct OwnerRemoved {
        owner: AccountId,
    }

    #[ink(storage)]
    pub struct AcelonOracle {
        valid_sources_threshold: u8,
        valid_signers_threshold: u8,
        valid_time_period: u64,

        trusted_signers: Mapping<AccountId, bool>,
        certificate_trust_store: Mapping<Certificate, bool>,

        price_feeds: Mapping<RequestHash, PriceEntry>,

        owners: Mapping<AccountId, ()>,
        proposed_owners: Mapping<AccountId, ()>,
    }

    impl AcelonOracle {
        #[ink(constructor)]
        pub fn new(
            owners: Vec<AccountId>,
            trusted_signers: Vec<AccountId>,
            certificate_trust_store: Vec<Certificate>,
            valid_signers_threshold: u8,
            valid_sources_threshold: u8,
            valid_time_period: u64,
        ) -> Self {
            let mut signers = Mapping::default();
            for signer in trusted_signers {
                ink::env::debug_println!("inserting signer: {:?}", signer);
                signers.insert(signer, &true);
            }

            let mut trust_store = Mapping::default();
            for cert in certificate_trust_store {
                trust_store.insert(cert, &true);
            }

            let mut owners_map = Mapping::default();

            for owner in owners {
                owners_map.insert(owner, &());
            }

            Self {
                valid_sources_threshold,
                valid_signers_threshold,
                valid_time_period,
                trusted_signers: signers,
                certificate_trust_store: trust_store,
                price_feeds: Mapping::default(),
                owners: owners_map,
                proposed_owners: Mapping::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(
                Default::default(),
                Default::default(),
                Default::default(),
                1,
                1,
                3_600_000,
            )
        }

        #[ink(message)]
        pub fn propose_owner(&mut self, new_owner: AccountId) -> Result<(), Error> {
            self.ensure_owner()?;
            self.proposed_owners.insert(new_owner, &());
            self.env().emit_event(OwnerProposed {
                proposed_owner: new_owner,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn accept_owner(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            let maybe_proposed = self.proposed_owners.take(caller);
            if maybe_proposed.is_none() {
                return Err(Error::NotProposedOwner);
            }
            self.owners.insert(caller, &());
            self.env().emit_event(OwnerAccepted);
            Ok(())
        }

        #[ink(message)]
        pub fn remove_owner(&mut self, owner: AccountId) -> Result<(), Error> {
            self.ensure_owner()?;
            let maybe_removed_owner = self.owners.take(owner);
            if maybe_removed_owner.is_some() {
                self.env().emit_event(OwnerRemoved { owner })
            }
            Ok(())
        }

        fn ensure_owner(&self) -> Result<(), Error> {
            let caller = self.env().caller();
            let maybe_owner = self.owners.get(caller);
            if maybe_owner.is_none() {
                return Err(Error::NotOwner);
            }
            Ok(())
        }
    }

    impl PriceOracle for AcelonOracle {
        #[ink(message)]
        fn update_price_feeds(
            &mut self,
            update_data: Vec<Vec<u8>>,
            signatures: Vec<Vec<Signature>>,
        ) -> Result<(), Error> {
            if signatures.len() < update_data.len() {
                return Err(Error::NotEnoughValidSignatures);
            }
            for (i, data) in update_data.into_iter().enumerate() {
                // 1. check the signatures
                let mut valid_signers_counter = 0u8;
                let signatures_to_check = &signatures[i];
                let message_hash = self.env().hash_bytes::<Blake2x256>(&data);
                for signature in signatures_to_check {
                    if let Ok(signer_pk) = self.env().ecdsa_recover(signature, &message_hash) {
                        let signer_account_id: AccountId =
                            self.env().hash_bytes::<Blake2x256>(&signer_pk).into();
                        let is_trusted =
                            self.trusted_signers.get(signer_account_id).unwrap_or(false);
                        if is_trusted {
                            valid_signers_counter = valid_signers_counter.saturating_add(1);
                            if valid_signers_counter >= self.valid_signers_threshold {
                                break;
                            }
                        }
                    }
                }
                if valid_signers_counter < self.valid_signers_threshold {
                    return Err(Error::NotEnoughValidSignatures);
                }

                // 2. decode the data
                let price_paylod = PricePayload::decode(&mut data.as_slice())
                    .map_err(|_| Error::InvalidPayload)?;

                // 3. check certificates
                let mut valid_sources_counter = 0u8;
                for certificate in &price_paylod.certificates {
                    let is_valid_certificate = self
                        .certificate_trust_store
                        .get(certificate)
                        .unwrap_or(false);
                    if is_valid_certificate {
                        valid_sources_counter = valid_sources_counter.saturating_add(1);
                        if valid_sources_counter >= self.valid_sources_threshold {
                            break;
                        }
                    }
                }
                if valid_sources_counter < self.valid_sources_threshold {
                    return Err(Error::NotEnoughValidSources);
                }

                // 4. set the new price
                let is_more_recent = self
                    .price_feeds
                    .get(price_paylod.request_hash)
                    .map(|curret_price| curret_price.timestamp < price_paylod.timestamp)
                    .unwrap_or(true);
                if is_more_recent {
                    let price_entry = PriceEntry {
                        timestamp: price_paylod.timestamp,
                        prices: price_paylod.prices,
                    };
                    self.price_feeds
                        .insert(price_paylod.request_hash, &price_entry);
                    self.env().emit_event(PriceFeedUpdate {
                        request_hash: price_paylod.request_hash,
                        price_entry,
                    });
                }
            }
            Ok(())
        }

        #[ink(message)]
        fn price_feed_exists(&self, request_hash: RequestHash) -> bool {
            self.price_feeds
                .get(request_hash)
                .map(|value| value.timestamp > 0)
                .unwrap_or(false)
        }

        #[ink(message)]
        fn get_valid_time_period(&self) -> u64 {
            self.valid_time_period
        }

        #[ink(message)]
        fn get_price(&self, request_hash: RequestHash) -> Option<PriceEntry> {
            self.get_price_no_holder_than(request_hash, self.valid_time_period)
        }

        #[ink(message)]
        fn get_price_no_holder_than(
            &self,
            request_hash: RequestHash,
            age: u64,
        ) -> Option<PriceEntry> {
            let now = self.env().block_timestamp();
            let price_feed = self.price_feeds.get(request_hash);
            if let Some(price_feed) = price_feed {
                let price_time = price_feed.timestamp;
                if super::diff(price_time, now) <= age {
                    return Some(price_feed);
                }
            }
            None
        }

        #[ink(message)]
        fn update_signers_threshold(&mut self, new_threshold: u8) -> Result<(), Error> {
            self.ensure_owner()?;
            self.valid_signers_threshold = new_threshold;
            self.env()
                .emit_event(SignersThresholdUpdate { new_threshold });
            Ok(())
        }

        #[ink(message)]
        fn update_sources_threshold(&mut self, new_threshold: u8) -> Result<(), Error> {
            self.ensure_owner()?;
            self.valid_sources_threshold = new_threshold;
            self.env()
                .emit_event(SourcesThresholdUpdate { new_threshold });
            Ok(())
        }

        #[ink(message)]
        fn update_valid_time_period(&mut self, new_time_period: u64) -> Result<(), Error> {
            self.ensure_owner()?;
            self.valid_time_period = new_time_period;
            self.env()
                .emit_event(ValidTimePeriodUpdate { new_time_period });
            Ok(())
        }

        #[ink(message)]
        fn add_trusted_signer(&mut self, new_trusted_signer: AccountId) -> Result<(), Error> {
            self.ensure_owner()?;
            self.trusted_signers.insert(new_trusted_signer, &true);
            self.env()
                .emit_event(TrustedSignerAdded { new_trusted_signer });
            Ok(())
        }

        #[ink(message)]
        fn remove_trusted_signer(&mut self, trusted_signer: AccountId) -> Result<(), Error> {
            self.ensure_owner()?;
            let maybe_removed_signer = self.trusted_signers.take(trusted_signer);
            if maybe_removed_signer.is_some() {
                self.env()
                    .emit_event(TrustedSignerRemoved { trusted_signer });
            }
            Ok(())
        }

        #[ink(message)]
        fn add_certificate_to_trust_store(
            &mut self,
            new_certificate: Certificate,
        ) -> Result<(), Error> {
            self.ensure_owner()?;
            self.certificate_trust_store.insert(new_certificate, &true);
            self.env().emit_event(CertificateAdded { new_certificate });
            Ok(())
        }

        #[ink(message)]
        fn remove_certificate_to_trust_store(
            &mut self,
            certificate: Certificate,
        ) -> Result<(), Error> {
            self.ensure_owner()?;
            let maybe_removed_cert = self.certificate_trust_store.take(certificate);
            if maybe_removed_cert.is_some() {
                self.env().emit_event(CertificateRemoved { certificate });
            }
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use hex_literal::hex;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let acelon_oracle = AcelonOracle::default();
            assert_eq!(acelon_oracle.get_valid_time_period(), 3_600_000);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn test_update_price_feed() {
            let oracle_1: AccountId = blake2_256(
                hex!("03b689085552418111b9c8a6683e470c83f16f79a96c8f5fe35df1a3ab0e24d020")
                    .as_slice(),
            )
            .into();
            let mut acelon_oracle = AcelonOracle::new(
                vec![],
                vec![oracle_1],
                vec![
                    hex!("ef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e"),
                    hex!("4795062d13e1ed971c6b6e5699764681e4d090bad39a7ef367cc9cb705652384"),
                    hex!("b75b095dc5d2a59f082ef17d0c64104aaeab321bb0a49a26b9b27a59792fce80"),
                ],
                1,
                1,
                0,
            );
            let result = acelon_oracle.update_price_feeds(
                vec![hex!("0440cf669e0e0000000000000000000000b3285d6c920100000cef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e4795062d13e1ed971c6b6e5699764681e4d090bad39a7ef367cc9cb705652384b75b095dc5d2a59f082ef17d0c64104aaeab321bb0a49a26b9b27a59792fce80dd450ee6601d873d2d381d738d87a131f5605ec30c6adbc9138b79f2dfd6f3dc").to_vec()], 
                vec![vec![hex!("147124904bd67756d1b5efecd302e01c87639cab15a6b05f5913877c0d55c5787b676599461f0da5fe4c3fd02981a1216cd226b767c2f0ef464bbed26c72fa221b")]],
            );
            assert_eq!(result, Ok(()));
        }

        #[inline(always)]
        fn blake2<const N: usize>(data: &[u8]) -> [u8; N] {
            blake2b_simd::Params::new()
                .hash_length(N)
                .hash(data)
                .as_bytes()
                .try_into()
                .expect("slice is always the necessary length")
        }

        pub fn blake2_256(data: &[u8]) -> [u8; 32] {
            blake2(data)
        }
    }
}
