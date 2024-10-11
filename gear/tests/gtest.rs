use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
    ActorId,
};

use acelon_oracle_app::utils::*;
use acelon_oracle_client::{traits::*, *};

use hex_literal::hex;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn test_update_price_feeds() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(acelon_oracle::WASM_BINARY);

    let program_factory = acelon_oracle_client::AcelonOracleFactory::new(remoting.clone());

    let program_id = program_factory
        .new(ACTOR_ID.into()) // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = acelon_oracle_client::OracleService::new(remoting.clone());

    let oracle_1: ActorId = blake2_256(
        hex!("03b689085552418111b9c8a6683e470c83f16f79a96c8f5fe35df1a3ab0e24d020").as_slice(),
    )
    .into();

    service_client.configure(vec![
        ConfigureArgument::SignersThreshold(1),
        ConfigureArgument::SourcesThreshold(1),
        ConfigureArgument::TimePeriod(0),
    ]);

    service_client.update_trusted_signer(ListUpdateForActorId {
        operation: ListUpdateOperation::Add,
        item: oracle_1,
    });

    service_client.update_certificate_to_trust_store(ListUpdateForArrOf32U8 {
        operation: ListUpdateOperation::Add,
        item: hex!("ef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e"),
    });

    service_client.update_certificate_to_trust_store(ListUpdateForArrOf32U8 {
        operation: ListUpdateOperation::Add,
        item: hex!("4795062d13e1ed971c6b6e5699764681e4d090bad39a7ef367cc9cb705652384"),
    });

    service_client.update_certificate_to_trust_store(ListUpdateForArrOf32U8 {
        operation: ListUpdateOperation::Add,
        item: hex!("b75b095dc5d2a59f082ef17d0c64104aaeab321bb0a49a26b9b27a59792fce80"),
    });

    service_client
        .update_price_feeds(
            vec![hex!("0440cf669e0e0000000000000000000000b3285d6c920100000cef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e4795062d13e1ed971c6b6e5699764681e4d090bad39a7ef367cc9cb705652384b75b095dc5d2a59f082ef17d0c64104aaeab321bb0a49a26b9b27a59792fce80dd450ee6601d873d2d381d738d87a131f5605ec30c6adbc9138b79f2dfd6f3dc").to_vec()],
            vec![vec![hex!("147124904bd67756d1b5efecd302e01c87639cab15a6b05f5913877c0d55c5787b676599461f0da5fe4c3fd02981a1216cd226b767c2f0ef464bbed26c72fa221b")]],
        ) // Call service's method (see app/src/lib.rs:14)
        .send_recv(program_id)
        .await
        .unwrap();
}
