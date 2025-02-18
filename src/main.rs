//! sindri-scroll-sdk
//!
//! This program connects the Sindri proving service to the Scroll proving SDK.
//!
use clap::Parser;
use scroll_proving_sdk::{prover::ProverBuilder, utils::init_tracing};
use sindri_scroll_sdk::prover::{CloudProver, CloudProverConfig};

#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)]
struct Args {
    /// Path of config file
    #[arg(long = "config", default_value = "config.json")]
    config_file: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let args = Args::parse();
    let cfg = CloudProverConfig::from_file_and_env(args.config_file)?;
    let sdk_config = cfg.sdk_config.clone();
    let cloud_prover = CloudProver::new(cfg);
    let prover = ProverBuilder::new(sdk_config)
        .with_proving_service(Box::new(cloud_prover))
        .build()
        .await?;

    prover.run().await;

    Ok(())
}
