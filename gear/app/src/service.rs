use gstd::exec;
use gstd::msg;
use sails_rs::prelude::*;

use crate::storage::*;
use crate::types::*;
use crate::utils::*;

#[derive(Default)]
pub struct AcelonOracleService();

impl AcelonOracleService {
    pub fn init(owner: AccountId) -> Self {
        unsafe {
            STORAGE = Some(Storage::default());
        }
        Storage::owners().insert(owner, ());
        Self()
    }

    fn ensure_owner() -> Result<(), AcelonError> {
        let owners = Storage::owners();
        if owners.contains_key(&msg::source()) {
            Ok(())
        } else {
            Err(AcelonError::NotOwner)
        }
    }

    fn ensure_proposed_owner() -> Result<(), AcelonError> {
        let proposed_owners = Storage::proposed_owners();
        if proposed_owners.contains_key(&msg::source()) {
            Ok(())
        } else {
            Err(AcelonError::NotProposedOwner)
        }
    }

    fn do_update_price_feeds(
        update_data: Vec<Vec<u8>>,
        signatures: Vec<Vec<Signature>>,
    ) -> Result<Vec<Event>, AcelonError> {
        if signatures.len() < update_data.len() {
            return Err(AcelonError::NotEnoughValidSignatures);
        }
        let mut events = Vec::<Event>::new();
        for (i, data) in update_data.into_iter().enumerate() {
            // 1. check the signatures
            let mut valid_signers_counter = 0u8;
            let signatures_to_check = &signatures[i];
            let message_hash = blake2_256(&data);
            for signature in signatures_to_check {
                if let Ok(signer_pk) = secp256k1_ecdsa_recover_compressed(signature, &message_hash)
                {
                    let singer_account_id: AccountId = blake2_256(&signer_pk).into();
                    let is_trusted = Storage::trusted_signers().get(&singer_account_id).is_some();
                    if is_trusted {
                        valid_signers_counter = valid_signers_counter.saturating_add(1);
                        if valid_signers_counter >= Storage::config().valid_signers_threshold {
                            break;
                        }
                    }
                }
            }
            if valid_signers_counter < Storage::config().valid_signers_threshold {
                return Err(AcelonError::NotEnoughValidSignatures);
            }

            // 2. decode the data
            let price_payload = PricePayload::decode(&mut data.as_slice())
                .map_err(|_| AcelonError::InvalidPayload)?;

            // 3. check certificates
            let mut valid_sources_counter = 0u8;
            for certificate in &price_payload.certificates {
                let is_valid_certificate = Storage::certificate_trust_store()
                    .get(certificate)
                    .is_some();
                if is_valid_certificate {
                    valid_sources_counter = valid_sources_counter.saturating_add(1);
                    if valid_sources_counter >= Storage::config().valid_sources_threshold {
                        break;
                    }
                }
            }
            if valid_sources_counter < Storage::config().valid_sources_threshold {
                return Err(AcelonError::NotEnoughValidSources);
            }

            // 4. set the new price
            let is_more_recent = Storage::price_feeds()
                .get(&price_payload.request_hash)
                .map(|current_price| current_price.timestamp < price_payload.timestamp)
                .unwrap_or(true);
            if is_more_recent {
                let price_entry = PriceEntry {
                    timestamp: price_payload.timestamp,
                    prices: price_payload.prices,
                };
                Storage::price_feeds().insert(price_payload.request_hash, price_entry.clone());
                events.push(Event::PriceFeedUpdate {
                    request_hash: price_payload.request_hash,
                    price_entry,
                });
            }
        }
        Ok(events)
    }
}

#[sails_rs::service(events = Event)]
impl AcelonOracleService {
    pub fn new() -> Self {
        Self()
    }

    pub fn config(&self) -> &'static Config {
        Storage::config()
    }

    pub fn configure(&mut self, actions: Vec<ConfigureArgument>) {
        panicking(Self::ensure_owner);

        let config = Storage::config();

        for action in actions {
            match action {
                ConfigureArgument::SignersThreshold(new_threshold) => {
                    config.valid_signers_threshold = new_threshold;
                    let _ = self.notify_on(Event::SignersThresholdUpdate { new_threshold });
                }
                ConfigureArgument::SourcesThreshold(new_threshold) => {
                    config.valid_sources_threshold = new_threshold;
                    let _ = self.notify_on(Event::SourcesThresholdUpdate { new_threshold });
                }
                ConfigureArgument::TimePeriod(new_time_period) => {
                    config.valid_time_period = new_time_period;
                    let _ = self.notify_on(Event::ValidTimePeriodUpdate { new_time_period });
                }
            }
        }
    }

    pub fn update_price_feeds(
        &mut self,
        update_data: Vec<Vec<u8>>,
        signatures: Vec<Vec<Signature>>,
    ) {
        let events = panicking(move || Self::do_update_price_feeds(update_data, signatures));
        for event in events {
            let _ = self.notify_on(event);
        }
    }

    pub fn price_feed_exists(&self, request_hash: RequestHash) -> bool {
        Storage::price_feeds()
            .get(&request_hash)
            .map(|value| value.timestamp > 0)
            .unwrap_or(false)
    }

    pub fn get_valid_time_period(&self) -> u64 {
        Storage::config().valid_time_period
    }

    pub fn get_price(&self, request_hash: RequestHash) -> Option<&'static PriceEntry> {
        self.get_price_no_holder_than(request_hash, Storage::config().valid_time_period)
    }

    pub fn get_price_no_holder_than(
        &self,
        request_hash: RequestHash,
        age: u64,
    ) -> Option<&'static PriceEntry> {
        let now = exec::block_timestamp();
        let price_feed = Storage::price_feeds().get(&request_hash);
        if let Some(price_feed) = price_feed {
            let price_time = price_feed.timestamp;
            if diff(price_time, now) <= age {
                return Some(price_feed);
            }
        }
        None
    }

    pub fn update_trusted_signer(&mut self, update: TrustedSignerUpdate) {
        panicking(Self::ensure_owner);
        match update.operation {
            ListUpdateOperation::Add => {
                Storage::trusted_signers().insert(update.item, ());
            }
            ListUpdateOperation::Remove => {
                Storage::trusted_signers().remove(&update.item);
            }
        }
        let _ = self.notify_on(Event::TrustedSignerUpdate { update });
    }

    pub fn update_certificate_to_trust_store(&mut self, update: CertificateTrustStoreUpdate) {
        panicking(Self::ensure_owner);
        match update.operation {
            ListUpdateOperation::Add => {
                Storage::certificate_trust_store().insert(update.item, ());
            }
            ListUpdateOperation::Remove => {
                Storage::certificate_trust_store().remove(&update.item);
            }
        }
        let _ = self.notify_on(Event::CertificateTrustStoreUpdated { update });
    }

    pub fn propose_owner(&mut self, new_owner: AccountId) {
        panicking(Self::ensure_owner);
        Storage::proposed_owners().insert(new_owner, ());
        let _ = self.notify_on(Event::OwnerProposed {
            proposed_owner: new_owner,
        });
    }

    pub fn accept_owner(&mut self) {
        panicking(Self::ensure_proposed_owner);
        let caller = msg::source();
        Storage::proposed_owners().remove(&caller);
        Storage::owners().insert(caller, ());
        let _ = self.notify_on(Event::OwnerAccepted);
    }

    pub fn remove_owner(&mut self, owner: AccountId) {
        panicking(Self::ensure_owner);
        let maybe_removed_owner = Storage::owners().remove(&owner);
        if maybe_removed_owner.is_some() {
            let _ = self.notify_on(Event::OwnerRemoved { owner });
        }
    }
}

fn diff(a: u64, b: u64) -> u64 {
    if a > b {
        a.saturating_sub(b)
    } else {
        b.saturating_sub(a)
    }
}
