use anyhow::{anyhow, Result as AnyResult};
use std::{env, fs};
use subxt::ext::sp_core::sr25519::Pair;
use subxt::ext::sp_core::Pair as _;
use subxt::tx::PairSigner;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_url = "wss://ws.test.azero.dev:443")]
pub mod aleph {}

const TESTNET_WS: &'static str = "wss://ws.test.azero.dev:443";

/// We should be quite compatible to Polkadot.
type AlephConfig = PolkadotConfig;

fn get_signer() -> AnyResult<PairSigner<AlephConfig, Pair>> {
    let seed_path = env::current_dir()?.join("seed.phrase");
    let seed = fs::read_to_string(seed_path)?;
    let pair = Pair::from_string(&seed, None)
        .map_err(|e| anyhow!("Cannot create signer from the seed: {:?}", e))?;
    Ok(PairSigner::new(pair))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = get_signer()?;
    let client = OnlineClient::<AlephConfig>::from_url(TESTNET_WS).await?;
    Ok(())
}
