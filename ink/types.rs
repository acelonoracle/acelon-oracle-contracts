use ink::prelude::vec::Vec;
use scale::{Decode, Encode};
use scale_info::TypeInfo;

pub type Signature = [u8; 65];
pub type Certificate = [u8; 32];
pub type RequestHash = [u8; 32];

#[derive(Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PriceEntry {
    pub timestamp: u64,
    pub prices: Vec<u128>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PricePayload {
    pub prices: Vec<u128>,
    pub timestamp: u64,
    pub certificates: Vec<Certificate>,
    pub request_hash: RequestHash,
}

#[derive(scale_info::TypeInfo, Debug, PartialEq, Eq, Encode, Decode)]
pub enum Error {
    NotEnoughValidSignatures,
    InvalidPayload,
    NotEnoughValidSources,
    NotOwner,
    NotProposedOwner,
}
