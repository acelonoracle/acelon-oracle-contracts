use frame_support::{
    derive_impl,
    sp_runtime::{traits::IdentityLookup, AccountId32, BuildStorage},
    traits::{ConstU16, ConstU32, ConstU64},
};
use sp_core::H256;

use crate::types::CU32;

pub type AccountId = AccountId32;
pub type Block = frame_system::mocking::MockBlock<Test>;

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();

        let parachain_info_config = parachain_info::GenesisConfig {
            parachain_id: 2000.into(),
            ..Default::default()
        };

        <parachain_info::GenesisConfig<Test> as BuildStorage>::assimilate_storage(
            &parachain_info_config,
            &mut t,
        )
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {}
    }
}

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>} = 0,
        ParachainInfo: parachain_info::{Pallet, Storage, Config<T>},
        Acelon: crate::{Pallet, Call, Storage, Event<T>}
    }
);

#[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Nonce = u64;
    type Hash = H256;
    type Block = Block;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type AccountData = ();
    type DbWeight = ();
    type BlockWeights = ();
    type BlockLength = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl parachain_info::Config for Test {}

impl crate::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type MaxPrices = CU32<50>;
    type MaxCertificates = CU32<50>;
    type MaxPriceUpdates = CU32<10>;
    type Signature = crate::types::Signature;
    type Public = crate::types::Public;
    type WeightInfo = ();
}
