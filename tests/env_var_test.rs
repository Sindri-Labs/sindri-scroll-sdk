use clap::Parser;
use sindri_scroll_sdk::prover::{override_config};
use scroll_proving_sdk::{config::Config, prover::CircuitType};

#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)]
struct Args {
    /// Path of config file
    #[arg(long = "config", default_value = "example.config.json")]
    config_file: String,
}

// This test checks deserialization to CircuitType.
// Override the default config to use the chunk circuit type.
#[test]
fn test_circuit_type_override() {
    let args = Args::parse();
    let mut cfg: Config = Config::from_file(args.config_file).unwrap();
    std::env::set_var("PROVER_CIRCUIT_TYPE", "chunk");
    override_config(&mut cfg).expect("Failed to override config");
    assert_eq!(cfg.prover.circuit_type, CircuitType::Chunk);
}

// This test checks correct deserialization of string types.
// It overrides the default config to use a local endpoint 
// for the coordinator and also updates the API key.
// It also tests nested fields for Option types.
#[test]
fn test_string_type_override() {
    let args = Args::parse();
    let mut cfg: Config = Config::from_file(args.config_file).unwrap();
    std::env::set_var("PROVER_NAME_PREFIX", "__SINDRI__");
    std::env::set_var("L2GETH_ENDPOINT", "http://l2-rpc:8545");
    std::env::set_var("PROVER_CLOUD_API_KEY", "sindri_apikeyvalue");
    override_config(&mut cfg).expect("Failed to override config");
    assert_eq!(cfg.prover_name_prefix, "__SINDRI__");
    assert_eq!(cfg.l2geth.unwrap().endpoint, "http://l2-rpc:8545");
    assert_eq!(cfg.prover.cloud.unwrap().api_key, "sindri_apikeyvalue");
}

// This test checks correct deserialization of integer types.
#[test] 
fn test_uint_type_override() {
    let args = Args::parse();
    let mut cfg: Config = Config::from_file(args.config_file).unwrap();
    std::env::set_var("PROVER_N_WORKERS", "100");
    std::env::set_var("PROVER_CLOUD_RETRY_COUNT", "15");
    std::env::set_var("PROVER_CLOUD_CONNECTION_TIMEOUT_SEC", "300");
    override_config(&mut cfg).expect("Failed to override config");
    assert_eq!(cfg.prover.n_workers, 100_usize);
    assert_eq!(cfg.prover.cloud.clone().unwrap().retry_count, 15_u32);
    assert_eq!(cfg.prover.cloud.unwrap().connection_timeout_sec, 300_u64);
}