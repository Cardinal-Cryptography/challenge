use pallet_contracts_primitives::{ContractExecResult, ExecReturnValue};
use pallet_contracts_rpc::CallRequest;
use sp_rpc::number::NumberOrHex;
use subxt::{rpc::RpcParams, OnlineClient};

use crate::{contract_account, AccountId32, AlephConfig, GAS_LIMIT};

pub async fn rpc_call(
    origin: &AccountId32,
    message_with_params: &[u8],
    client: &OnlineClient<AlephConfig>,
) -> Result<ExecReturnValue, subxt::Error> {
    let request = CallRequest {
        origin: origin.clone(),
        dest: contract_account(),
        value: NumberOrHex::Number(0),
        gas_limit: NumberOrHex::Number(GAS_LIMIT),
        storage_deposit_limit: None,
        input_data: message_with_params.to_vec().into(),
    };

    let mut rpc_params = RpcParams::new();
    rpc_params.push(request)?;

    let result: ContractExecResult<u128> =
        client.rpc().request("contracts_call", rpc_params).await?;
    Ok(result
        .result
        .map_err(|e| subxt::Error::Other(format!("RPC call failed with {:?}", e)))?)
}
