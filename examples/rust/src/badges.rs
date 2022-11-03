use codec::Encode;
use pallet_contracts_primitives::{ContractExecResult, ExecReturnValue};
use pallet_contracts_rpc::CallRequest;
use sp_rpc::number::NumberOrHex;
use subxt::{ext::sp_runtime::AccountId32, rpc::RpcParams, OnlineClient};

use crate::{contract_account, AlephConfig, BADGES, HAS_BADGE_SELECTOR, READ_GAS_LIMIT};

async fn has_badge(
    account: &AccountId32,
    badge: &str,
    client: &OnlineClient<AlephConfig>,
) -> Result<bool, subxt::Error> {
    let mut message_with_params = HAS_BADGE_SELECTOR.to_vec();
    account.encode_to(&mut message_with_params);
    badge.encode_to(&mut message_with_params);

    let request = CallRequest {
        origin: account.clone(),
        dest: contract_account(),
        value: NumberOrHex::Number(0),
        gas_limit: NumberOrHex::Number(READ_GAS_LIMIT),
        storage_deposit_limit: None,
        input_data: message_with_params.into(),
    };

    let mut rpc_params = RpcParams::new();
    rpc_params.push(request)?;

    let result: ContractExecResult<u128> =
        client.rpc().request("contracts_call", rpc_params).await?;
    if let Ok(ExecReturnValue { data, .. }) = result.result {
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
