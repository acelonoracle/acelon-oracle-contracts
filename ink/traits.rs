use ink::{prelude::vec::Vec, primitives::AccountId};

use crate::types::{Certificate, Error, PriceEntry, RequestHash, Signature};

#[ink::trait_definition]
pub trait PriceOracle {
    #[ink(message)]
    fn update_price_feeds(
        &mut self,
        update_data: Vec<Vec<u8>>,
        signatures: Vec<Vec<Signature>>,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn price_feed_exists(&self, request_hash: RequestHash) -> bool;

    #[ink(message)]
    fn get_valid_time_period(&self) -> u64;

    #[ink(message)]
    fn get_price(&self, request_hash: RequestHash) -> Option<PriceEntry>;

    #[ink(message)]
    fn get_price_no_holder_than(&self, request_hash: RequestHash, age: u64) -> Option<PriceEntry>;

    #[ink(message)]
    fn update_signers_threshold(&mut self, new_threshold: u8) -> Result<(), Error>;

    #[ink(message)]
    fn update_sources_threshold(&mut self, new_threshold: u8) -> Result<(), Error>;

    #[ink(message)]
    fn update_valid_time_period(&mut self, new_time_period: u64) -> Result<(), Error>;

    #[ink(message)]
    fn add_trusted_signer(&mut self, new_trusted_signer: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn remove_trusted_signer(&mut self, trusted_signer: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn add_certificate_to_trust_store(&mut self, new_certificate: Certificate)
        -> Result<(), Error>;

    #[ink(message)]
    fn remove_certificate_to_trust_store(&mut self, certificate: Certificate) -> Result<(), Error>;
}
