use sails_rs::prelude::{scale_codec::*, *};

pub type Signature = [u8; 65];
pub type Certificate = [u8; 32];
pub type RequestHash = [u8; 32];
pub type AccountId = ActorId;

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Event {
    PriceFeedUpdate {
        request_hash: RequestHash,
        price_entry: PriceEntry,
    },
    SignersThresholdUpdate {
        new_threshold: u8,
    },
    SourcesThresholdUpdate {
        new_threshold: u8,
    },
    ValidTimePeriodUpdate {
        new_time_period: u64,
    },
    TrustedSignerUpdate {
        update: TrustedSignerUpdate,
    },
    CertificateTrustStoreUpdated {
        update: CertificateTrustStoreUpdate,
    },
    OwnerProposed {
        proposed_owner: AccountId,
    },
    OwnerAccepted,
    OwnerRemoved {
        owner: AccountId,
    },
}

/// Contract configurations are contained in this structure
#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo, Default)]
pub struct Config {
    pub valid_sources_threshold: u8,
    pub valid_signers_threshold: u8,
    pub valid_time_period: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub enum ConfigureArgument {
    SignersThreshold(u8),
    SourcesThreshold(u8),
    TimePeriod(u64),
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo, Default)]
pub struct PriceEntry {
    pub timestamp: u64,
    pub prices: Vec<u128>,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct PricePayload {
    pub prices: Vec<u128>,
    pub timestamp: u64,
    pub certificates: Vec<Certificate>,
    pub request_hash: RequestHash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
pub enum AcelonError {
    NotEnoughValidSignatures,
    InvalidPayload,
    NotEnoughValidSources,
    NotOwner,
    NotProposedOwner,
    InvalidSignature,
}

/// The allowed sources update operation.
#[derive(Debug, Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Copy)]
pub enum ListUpdateOperation {
    Add,
    Remove,
}

#[derive(Debug, Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq)]
pub struct ListUpdate<T>
where
    T: Encode + Decode + TypeInfo + MaxEncodedLen + Clone + PartialEq,
{
    /// The update operation.
    pub operation: ListUpdateOperation,
    pub item: T,
}

pub type TrustedSignerUpdate = ListUpdate<AccountId>;
pub type CertificateTrustStoreUpdate = ListUpdate<Certificate>;
