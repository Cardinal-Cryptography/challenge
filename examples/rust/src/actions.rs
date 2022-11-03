use subxt::{ext::sp_runtime::MultiAddress, tx::Signer, Config, OnlineClient};

use crate::{aleph, contract_account, AlephConfig, GAS_LIMIT, REGISTER_RANDOMNESS_SELECTOR};

pub async fn register_randomness<S: Signer<AlephConfig> + Send + Sync>(
    signer: &S,
    client: &OnlineClient<AlephConfig>,
) -> <AlephConfig as Config>::Hash {
    let tx = aleph::tx().contracts().call(
        MultiAddress::Id(contract_account()),
        0,
        GAS_LIMIT,
        None,
        REGISTER_RANDOMNESS_SELECTOR.to_vec(),
    );

    println!("Registering randomness...");

    let evs = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, signer)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();

    let block_hash = evs.block_hash();

    println!("Randomness registered for block with hash {block_hash:?}\n");

    block_hash
}
