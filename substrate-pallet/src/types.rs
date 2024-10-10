use frame_support::{
    sp_runtime::traits::IdentifyAccount, storage::bounded_vec::BoundedVec, traits::Get,
};
use scale::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::AccountId32, ecdsa, RuntimeDebug, TypedGet};

use crate::traits::RecoverableSignature;

pub type Certificate = [u8; 32];
pub type RequestHash = [u8; 32];

#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct PriceEntry<MaxPrices: Get<u32>> {
    pub timestamp: u64,
    pub prices: BoundedVec<u128, MaxPrices>,
}

#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct PricePayload<MaxPrices: Get<u32>, MaxCertificates: Get<u32>> {
    pub prices: BoundedVec<u128, MaxPrices>,
    pub timestamp: u64,
    pub certificates: BoundedVec<Certificate, MaxCertificates>,
    pub request_hash: RequestHash,
}

#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq)]
pub struct Signature([u8; 65]);

impl Signature {
    fn canonical_signature(&self) -> ecdsa::Signature {
        let mut canonical_value = self.0;
        if canonical_value[64] > 26 {
            canonical_value[64] -= 27;
        }
        canonical_value.into()
    }
}

impl RecoverableSignature for Signature {
    type Public = Public;

    fn recover_prehashed(&self, message: &[u8; 32]) -> Option<Self::Public> {
        self.canonical_signature()
            .recover_prehashed(message)
            .map(Public)
    }
}

impl From<[u8; 65]> for Signature {
    fn from(value: [u8; 65]) -> Self {
        Self(value)
    }
}

#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq)]
pub struct Public(ecdsa::Public);

impl IdentifyAccount for Public {
    type AccountId = AccountId32;

    fn into_account(self) -> Self::AccountId {
        sp_io::hashing::blake2_256(self.0 .0.as_slice()).into()
    }
}

/// The allowed sources update operation.
#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Copy)]
pub enum ListUpdateOperation {
    Add,
    Remove,
}

#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq)]
pub struct ListUpdate<T>
where
    T: Encode + Decode + TypeInfo + MaxEncodedLen + Clone + PartialEq,
{
    /// The update operation.
    pub operation: ListUpdateOperation,
    pub item: T,
}

#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Eq, PartialEq)]
pub struct CU32<const T: u32>;
impl<const T: u32> Get<u32> for CU32<T> {
    fn get() -> u32 {
        T
    }
}

impl<const T: u32> Get<Option<u32>> for CU32<T> {
    fn get() -> Option<u32> {
        Some(T)
    }
}

impl<const T: u32> TypedGet for CU32<T> {
    type Type = u32;
    fn get() -> u32 {
        T
    }
}

#[cfg(feature = "std")]
impl<const T: u32> Serialize for CU32<T> {
    fn serialize<D>(&self, serializer: D) -> Result<D::Ok, D::Error>
    where
        D: serde::Serializer,
    {
        serializer.serialize_u32(<Self as TypedGet>::get())
    }
}

#[cfg(feature = "std")]
impl<'de, const T: u32> Deserialize<'de> for CU32<T> {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(CU32::<T>)
    }
}

pub type TrustedSignerUpdate<AccountId> = ListUpdate<AccountId>;
pub type CertificateTrustStoreUpdate = ListUpdate<Certificate>;

pub type MaxPricesFor<T> = <T as crate::Config>::MaxPrices;
pub type MaxCertificatesFor<T> = <T as crate::Config>::MaxCertificates;
pub type MaxPriceUpdatesFor<T> = <T as crate::Config>::MaxPriceUpdates;

pub type PriceEntryFor<T> = PriceEntry<MaxPricesFor<T>>;
pub type PricePayloadFor<T> = PricePayload<MaxPricesFor<T>, MaxCertificatesFor<T>>;
