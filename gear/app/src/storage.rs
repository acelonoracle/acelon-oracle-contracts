use collections::HashMap;
use sails_rs::prelude::*;

use crate::types::*;

pub static mut STORAGE: Option<Storage> = None;

#[derive(Debug, Default)]
pub struct Storage {
    config: Config,

    trusted_signers: HashMap<AccountId, ()>,
    certificate_trust_store: HashMap<Certificate, ()>,

    price_feeds: HashMap<RequestHash, PriceEntry>,

    owners: HashMap<AccountId, ()>,
    proposed_owners: HashMap<AccountId, ()>,
}

impl Storage {
    pub fn get_mut() -> &'static mut Self {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }

    pub fn get() -> &'static Self {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }

    pub fn config() -> &'static mut Config {
        let storage = Self::get_mut();
        &mut storage.config
    }

    pub fn trusted_signers() -> &'static mut HashMap<AccountId, ()> {
        let storage = Self::get_mut();
        &mut storage.trusted_signers
    }

    pub fn certificate_trust_store() -> &'static mut HashMap<Certificate, ()> {
        let storage = Self::get_mut();
        &mut storage.certificate_trust_store
    }

    pub fn price_feeds() -> &'static mut HashMap<RequestHash, PriceEntry> {
        let storage = Self::get_mut();
        &mut storage.price_feeds
    }

    pub fn owners() -> &'static mut HashMap<AccountId, ()> {
        let storage = Self::get_mut();
        &mut storage.owners
    }

    pub fn proposed_owners() -> &'static mut HashMap<AccountId, ()> {
        let storage = Self::get_mut();
        &mut storage.proposed_owners
    }
}
