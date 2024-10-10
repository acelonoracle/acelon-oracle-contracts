#![cfg_attr(not(feature = "std"), no_std)]

mod migration;
pub mod traits;
pub mod types;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::{DispatchResultWithPostInfo, *},
        sp_runtime::traits::IdentifyAccount,
        Blake2_128Concat, Parameter,
    };
    use frame_system::{
        ensure_root,
        pallet_prelude::{OriginFor, *},
    };
    use sp_std::prelude::*;

    use crate::{
        traits::{ParameterBound, RecoverableSignature, WeightInfo},
        types::{
            Certificate, CertificateTrustStoreUpdate, ListUpdateOperation, PriceEntry,
            PricePayload, RequestHash, TrustedSignerUpdate,
        },
    };

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        #[pallet::constant]
        type MaxPrices: Get<u32> + ParameterBound;
        #[pallet::constant]
        type MaxCertificates: Get<u32> + ParameterBound;
        #[pallet::constant]
        type MaxPriceUpdates: Get<u32> + Parameter;
        /// The signature type. It can be set to [pallet_acelon_oracle::types::Signature].
        type Signature: Parameter + RecoverableSignature<Public = Self::Public>;
        /// The public key type. It can be set to [pallet_acelon_oracle::types::Public].
        type Public: IdentifyAccount<AccountId = Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    pub(crate) const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn valid_signers_threshold)]
    pub type ValidSignersThreshold<T: Config> = StorageValue<_, u8, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn valid_sources_threshold)]
    pub type ValidSourcesThreshold<T: Config> = StorageValue<_, u8, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn valid_time_period)]
    pub type ValidTimePeriod<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn trusted_signer)]
    pub type TrustedSigner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

    #[pallet::storage]
    #[pallet::getter(fn certificate_trust_store)]
    pub type CertificateTrustStore<T: Config> = StorageMap<_, Blake2_128Concat, Certificate, ()>;

    #[pallet::storage]
    #[pallet::getter(fn price_feed)]
    pub type PriceFeed<T: Config> =
        StorageMap<_, Blake2_128Concat, RequestHash, PriceEntry<T::MaxPrices>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A price feed was updated. [request_hash]
        PriceFeedUpdate(RequestHash),
        /// Signers threshold updated. [new_threshold]
        SignersThresholdUpdated(u8),
        /// Sources threshold updated. [new_threshold]
        SourcesThresholdUpdated(u8),
        /// Valid time period updated. [new_time_period]
        ValidTimePeriodUpdated(u64),
        /// Trusted signers updated. [update]
        TrustedSignersUpdated(TrustedSignerUpdate<T::AccountId>),
        /// Certificate trust store updated. [update]
        CertificateTrustStoreUpdated(CertificateTrustStoreUpdate),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Not enough valid signatures.
        NotEnoughValidSignatures,
        /// Invalid price payload.
        InvalidPayload,
        /// Not enough valid sources.
        NotEnoughValidSources,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            crate::migration::migrate::<T>()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(< T as Config >::WeightInfo::update_price_feeds(update_data.len() as u32))]
        pub fn update_price_feeds(
            origin: OriginFor<T>,
            update_data: BoundedVec<Vec<u8>, T::MaxPriceUpdates>,
            signatures: BoundedVec<Vec<T::Signature>, T::MaxPriceUpdates>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            ensure!(
                signatures.len() >= update_data.len(),
                Error::<T>::NotEnoughValidSignatures
            );

            for (i, data) in update_data.into_iter().enumerate() {
                // 1. check the signatures
                let mut valid_signers_counter = 0u8;
                let signatures_to_check = &signatures[i];
                let message_hash = sp_io::hashing::blake2_256(data.as_slice());
                let signers_threshold = Self::valid_signers_threshold();
                for signature in signatures_to_check {
                    if let Some(signer_pk) = signature.recover_prehashed(&message_hash) {
                        let signer_account_id: T::AccountId = signer_pk.into_account();
                        let is_trusted = Self::trusted_signer(&signer_account_id).is_some();
                        if is_trusted {
                            valid_signers_counter = valid_signers_counter.saturating_add(1);
                            if valid_signers_counter >= signers_threshold {
                                break;
                            }
                        }
                    }
                }

                ensure!(
                    valid_signers_counter >= signers_threshold,
                    Error::<T>::NotEnoughValidSignatures
                );

                // 2. decode the data
                let price_payload =
                    PricePayload::<T::MaxPrices, T::MaxCertificates>::decode(&mut data.as_slice())
                        .map_err(|_| Error::<T>::InvalidPayload)?;

                // 3. check certificates
                let mut valid_sources_counter = 0u8;
                let sources_threshold = Self::valid_sources_threshold();
                for certificate in &price_payload.certificates {
                    let is_valid_certificate = Self::certificate_trust_store(certificate).is_some();
                    if is_valid_certificate {
                        valid_sources_counter = valid_sources_counter.saturating_add(1);
                        if valid_sources_counter >= sources_threshold {
                            break;
                        }
                    }
                }

                ensure!(
                    valid_sources_counter >= sources_threshold,
                    Error::<T>::NotEnoughValidSources
                );

                // 4. set the new price
                let is_more_recent = Self::price_feed(price_payload.request_hash)
                    .map(|current_price| current_price.timestamp < price_payload.timestamp)
                    .unwrap_or(true);
                if is_more_recent {
                    <PriceFeed<T>>::insert(
                        price_payload.request_hash,
                        PriceEntry {
                            timestamp: price_payload.timestamp,
                            prices: price_payload.prices,
                        },
                    );

                    Self::deposit_event(Event::<T>::PriceFeedUpdate(price_payload.request_hash));
                }
            }

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(< T as Config >::WeightInfo::update_signers_threshold())]
        pub fn update_signers_threshold(
            origin: OriginFor<T>,
            new_threshold: u8,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            <ValidSignersThreshold<T>>::put(new_threshold);
            Self::deposit_event(Event::<T>::SignersThresholdUpdated(new_threshold));
            Ok(().into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(< T as Config >::WeightInfo::update_sources_threshold())]
        pub fn update_sources_threshold(
            origin: OriginFor<T>,
            new_threshold: u8,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            <ValidSourcesThreshold<T>>::put(new_threshold);
            Self::deposit_event(Event::<T>::SourcesThresholdUpdated(new_threshold));
            Ok(().into())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(< T as Config >::WeightInfo::update_valid_time_period())]
        pub fn update_valid_time_period(
            origin: OriginFor<T>,
            new_time_period: u64,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            <ValidTimePeriod<T>>::put(new_time_period);
            Self::deposit_event(Event::<T>::ValidTimePeriodUpdated(new_time_period));
            Ok(().into())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(< T as Config >::WeightInfo::update_trusted_signer())]
        pub fn update_trusted_signer(
            origin: OriginFor<T>,
            update: TrustedSignerUpdate<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            match &update.operation {
                ListUpdateOperation::Add => {
                    <TrustedSigner<T>>::insert(&update.item, ());
                }
                ListUpdateOperation::Remove => {
                    <TrustedSigner<T>>::remove(&update.item);
                }
            }
            Self::deposit_event(Event::<T>::TrustedSignersUpdated(update));
            Ok(().into())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(< T as Config >::WeightInfo::update_certificate_trust_store())]
        pub fn update_certificate_trust_store(
            origin: OriginFor<T>,
            update: CertificateTrustStoreUpdate,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            match &update.operation {
                ListUpdateOperation::Add => {
                    <CertificateTrustStore<T>>::insert(update.item, ());
                }
                ListUpdateOperation::Remove => {
                    <CertificateTrustStore<T>>::remove(update.item);
                }
            }
            Self::deposit_event(Event::<T>::CertificateTrustStoreUpdated(update));
            Ok(().into())
        }
    }
}
