//! sindri-scroll-sdk
//!
//! This program connects the Sindri proving service to the Scroll proving SDK.
//!
use clap::Parser;
use scroll_proving_sdk::{config::Config, prover::ProverBuilder, utils::init_tracing};
use sindri_scroll_sdk::prover::{CloudProver, override_config};

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
    let mut cfg: Config = Config::from_file(args.config_file)?;
    let cfg = override_config(&mut cfg).expect("Failed to override config");
    let cloud_prover = CloudProver::new(
        cfg.prover
            .cloud
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Missing cloud prover configuration"))?,
    );
    let prover = ProverBuilder::new(cfg)
        .with_proving_service(Box::new(cloud_prover))
        .build()
        .await?;

    prover.run().await;

    Ok(())
}
