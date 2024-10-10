use frame_support::{
    traits::{Get, GetStorageVersion, StorageVersion},
    weights::Weight,
};

use super::*;

pub fn migrate<T: Config>() -> Weight {
    let migrations: [(u16, &dyn Fn() -> Weight); 0] = [];

    let on_chain_version = Pallet::<T>::on_chain_storage_version();
    let mut weight: Weight = Default::default();
    for (i, f) in migrations.into_iter() {
        if on_chain_version < StorageVersion::new(i) {
            weight += f();
        }
    }

    STORAGE_VERSION.put::<Pallet<T>>();
    weight + T::DbWeight::get().writes(1)
}
