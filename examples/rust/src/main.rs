mod badges;

use std::{env, fs};

use anyhow::{anyhow, Result as AnyResult};
use subxt::{
    ext::{
        sp_core::{crypto::Ss58Codec, sr25519::Pair, Pair as _},
        sp_runtime::AccountId32,
    },
    tx::PairSigner,
    OnlineClient, PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_url = "wss://ws.test.azero.dev:443")]
pub mod aleph {}

const TESTNET_WS: &'static str = "wss://ws.test.azero.dev:443";
const HARDXORE_ADDRESS: &'static str = "5GErKuHmZ8ytupuZb78AJbHY9yoaDnKLdLUYKchYukhrsjVj";
const BADGES: [&'static str; 5] = ["WARMUP", "XOR-0", "XOR-1", "XOR-2", "XOR-3"];

const HAS_BADGE_SELECTOR: [u8; 4] = [0xfd, 0xdc, 0xef, 0x2b];
const READ_GAS_LIMIT: u64 = 100_000_000_000;

/// We should be quite compatible to Polkadot.
type AlephConfig = PolkadotConfig;

fn get_signer() -> AnyResult<PairSigner<AlephConfig, Pair>> {
    let seed_path = env::current_dir()?.join("seed.phrase");
    let seed = fs::read_to_string(seed_path)?;
    let pair = Pair::from_string(&seed, None)
        .map_err(|e| anyhow!("Cannot create signer from the seed: {:?}", e))?;
    let signer = PairSigner::new(pair);
    println!("Keypair read -- AccountId = {}", signer.account_id());

    Ok(signer)
}

fn contract_account() -> AccountId32 {
    AccountId32::from_string(HARDXORE_ADDRESS).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = get_signer()?;
    let client = OnlineClient::<AlephConfig>::from_url(TESTNET_WS).await?;

    badges::print_badges(signer.account_id(), &client).await;
    Ok(())
}
