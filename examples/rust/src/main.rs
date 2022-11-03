mod actions;
mod rpc;
mod state;

use std::{env, fs};

use anyhow::{anyhow, Result as AnyResult};
use subxt::{
    ext::{
        sp_core::{crypto::Ss58Codec, sr25519::Pair, Pair as _},
        sp_runtime::AccountId32,
    },
    tx::PairSigner,
    Config, OnlineClient, PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_url = "wss://ws.test.azero.dev:443")]
pub mod aleph {}

const TESTNET_WS: &str = "wss://ws.test.azero.dev:443";
const HARDXORE_ADDRESS: &str = "5GErKuHmZ8ytupuZb78AJbHY9yoaDnKLdLUYKchYukhrsjVj";
const BADGES: [&str; 5] = ["WARMUP", "XOR-0", "XOR-1", "XOR-2", "XOR-3"];

const HAS_BADGE_SELECTOR: [u8; 4] = [0xfd, 0xdc, 0xef, 0x2b];
const REGISTER_RANDOMNESS_SELECTOR: [u8; 4] = [0x0b, 0x81, 0x97, 0x41];
const GET_RANDOMNESS_SELECTOR: [u8; 4] = [0x19, 0x4a, 0x46, 0xc8];
const ATTEMPT_XOR_3_SELECTOR: [u8; 4] = [0xb7, 0x56, 0x5f, 0x19];
const GAS_LIMIT: u64 = 100_000_000_000;

/// We should be quite compatible to Polkadot.
type AlephConfig = PolkadotConfig;

type Hash = <AlephConfig as Config>::Hash;
type BlockNumber = <AlephConfig as Config>::BlockNumber;

fn get_signer() -> AnyResult<PairSigner<AlephConfig, Pair>> {
    let seed_path = env::current_dir()?.join("seed.phrase");
    let seed = fs::read_to_string(seed_path)?;
    let pair = Pair::from_string(&seed, None)
        .map_err(|e| anyhow!("Cannot create signer from the seed: {:?}", e))?;
    let signer = PairSigner::new(pair);
    println!("Keypair read -- AccountId = {}\n", signer.account_id());

    Ok(signer)
}

fn contract_account() -> AccountId32 {
    AccountId32::from_string(HARDXORE_ADDRESS).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = get_signer()?;
    let client = OnlineClient::<AlephConfig>::from_url(TESTNET_WS).await?;

    let block_hash = actions::register_randomness(&signer, &client).await;
    state::get_randomness(block_hash, signer.account_id(), &client).await;

    // Don't expect a success here -- the above solution is obviously wrong for `XOR-3`
    actions::attempt_xor_3(&signer, &client).await;

    state::print_badges(signer.account_id(), &client).await;

    Ok(())
}
