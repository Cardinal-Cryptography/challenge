use codec::Encode;
use pallet_contracts_primitives::ExecReturnValue;
use subxt::{ext::sp_runtime::AccountId32, OnlineClient};

use crate::{
    rpc::rpc_call, AlephConfig, BlockNumber, Hash, BADGES, GET_RANDOMNESS_SELECTOR,
    HAS_BADGE_SELECTOR,
};

async fn has_badge(
    account: &AccountId32,
    badge: &str,
    client: &OnlineClient<AlephConfig>,
) -> Result<bool, subxt::Error> {
    let mut message_with_params = HAS_BADGE_SELECTOR.to_vec();
    account.encode_to(&mut message_with_params);
    badge.encode_to(&mut message_with_params);

    if let Ok(ExecReturnValue { data, .. }) = rpc_call(account, &message_with_params, client).await
    {
        Ok(data.0.len() > 1)
    } else {
        Ok(false)
    }
}

pub async fn print_badges(account: &AccountId32, client: &OnlineClient<AlephConfig>) {
    for badge in BADGES {
        let result = has_badge(account, badge, client).await.unwrap();
        println!("Checking if {account} has badge {badge} -- {result}")
    }
}

async fn get_block_number(block_hash: Hash, client: &OnlineClient<AlephConfig>) -> BlockNumber {
    client
        .rpc()
        .header(Some(block_hash))
        .await
        .unwrap()
        .unwrap()
        .number
}

pub async fn get_randomness(
    block_hash: Hash,
    account: &AccountId32,
    client: &OnlineClient<AlephConfig>,
) {
    let block_number = get_block_number(block_hash, client).await;

    let mut message_with_params = GET_RANDOMNESS_SELECTOR.to_vec();
    block_number.encode_to(&mut message_with_params);

    if let Ok(ExecReturnValue { data, .. }) = rpc_call(account, &message_with_params, client).await
    {
        println!("Randomness for block {block_number} is {data:?}\n")
    } else {
        println!("Failed to read randomness")
    }
}
