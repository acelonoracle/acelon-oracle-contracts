use frame_support::{sp_runtime::traits::Saturating, traits::Get, weights::Weight};
use sp_std::{fmt, prelude::*};

/// A bound that can be used to restrict length sequence types such as [`frame_support::BoundedVec`] appearing in types used in dispatchable functions.
///
/// Similar to [`frame_support::Parameter`] without encoding traits, since bounds are never encoded.
pub trait ParameterBound: Get<u32> + Clone + Eq + fmt::Debug + scale_info::TypeInfo {}
impl<T> ParameterBound for T where T: Get<u32> + Clone + Eq + fmt::Debug + scale_info::TypeInfo {}

pub trait RecoverableSignature {
    type Public;

    fn recover_prehashed(&self, message: &[u8; 32]) -> Option<Self::Public>;
}

pub trait WeightInfo {
    fn update_price_feeds(updates: u32) -> Weight;
    fn update_signers_threshold() -> Weight;
    fn update_sources_threshold() -> Weight;
    fn update_valid_time_period() -> Weight;
    fn update_trusted_signer() -> Weight;
    fn update_certificate_trust_store() -> Weight;
}

impl WeightInfo for () {
    fn update_price_feeds(updates: u32) -> Weight {
        Weight::from_parts(10_000.saturating_mul(updates as u64), 0)
    }

    fn update_signers_threshold() -> Weight {
        Weight::from_parts(10_000, 0)
    }

    fn update_sources_threshold() -> Weight {
        Weight::from_parts(10_000, 0)
    }

    fn update_valid_time_period() -> Weight {
        Weight::from_parts(10_000, 0)
    }

    fn update_trusted_signer() -> Weight {
        Weight::from_parts(10_000, 0)
    }

    fn update_certificate_trust_store() -> Weight {
        Weight::from_parts(10_000, 0)
    }
}
