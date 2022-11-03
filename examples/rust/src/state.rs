use codec::Encode;
use pallet_contracts_primitives::ExecReturnValue;
use subxt::{ext::sp_runtime::AccountId32, OnlineClient};

use crate::{rpc::rpc_call, AlephConfig, BADGES, HAS_BADGE_SELECTOR};

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
