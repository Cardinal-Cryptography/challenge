use codec::{Compact, Encode};
use subxt::{
    error::DispatchError, ext::sp_runtime::MultiAddress, tx::Signer, Config, Error, OnlineClient,
};

use crate::{
    aleph, contract_account, AlephConfig, BlockNumber, ATTEMPT_XOR_3_SELECTOR, GAS_LIMIT,
    REGISTER_RANDOMNESS_SELECTOR,
};

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

pub(crate) async fn attempt_xor_3<S: Signer<AlephConfig> + Send + Sync>(
    signer: &S,
    client: &OnlineClient<AlephConfig>,
) {
    let mut bytes = ATTEMPT_XOR_3_SELECTOR.to_vec();
    let solution: [BlockNumber; 2] = [14000000, 14000005];
    // Decoding vector requires firstly passing its length.
    Compact(solution.len() as u32).encode_to(&mut bytes);
    solution.encode_to(&mut bytes);

    let tx = aleph::tx().contracts().call(
        MultiAddress::Id(contract_account()),
        0,
        GAS_LIMIT,
        None,
        bytes,
    );

    println!("Attempting to solve XOR-3...");

    let error = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, signer)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap_err();

    match error {
        Error::Runtime(DispatchError::Module(error)) => {
            println!("Attempt failed: {error:?}\n")
        }
        _ => println!("Attempt failed\n"),
    }
}
