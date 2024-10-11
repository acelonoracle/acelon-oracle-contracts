#![no_std]

use service::AcelonOracleService;
use types::AccountId;

mod service;
mod storage;
pub mod types;
pub mod utils;

pub struct AcelonOracleProgram(());

#[sails_rs::program]
impl AcelonOracleProgram {
    // Program's constructor
    pub fn new(owner: AccountId) -> Self {
        AcelonOracleService::init(owner);
        Self(())
    }

    // Exposed service
    pub fn oracle_service(&self) -> AcelonOracleService {
        AcelonOracleService::default()
    }
}
