use clap::Parser;
use sindri_scroll_sdk::prover::{override_config};
use scroll_proving_sdk::{config::Config, prover::CircuitType};

#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)]
struct Args {
    /// Path of config file
    #[arg(long = "config", default_value = "config.json")]
    config_file: String,
}

#[test]
fn test_override_config() {
    let args = Args::parse();
    let mut cfg: Config = Config::from_file(args.config_file).unwrap();
    // Override the config with the environment variables
    std::env::set_var("PROVER_CIRCUIT_TYPE", "chunk");
    override_config(&mut cfg).expect("Failed to override config");
    assert_eq!(cfg.prover.circuit_type, CircuitType::Chunk);
}