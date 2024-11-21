use scroll_proving_sdk::{config::Config, prover::CircuitType};
use sindri_scroll_sdk::prover::CloudProver;
use std::env::{remove_var, set_var};

// Ensures that configuration file loading does not require environment variables
#[tokio::test]
async fn test_config_without_envs() {
    let default_config_path = "tests/test_data/default_config.json";
    let cfg: Config = Config::from_file_and_env(default_config_path.to_string())
        .expect("Issue loading test configuration file");

    // Check a few values for consistency with test_data file
    assert_eq!(cfg.prover_name_prefix, "sindri_");
    assert_eq!(cfg.prover.circuit_type, CircuitType::Bundle);
    assert_eq!(cfg.db_path, "db");
    assert_eq!(cfg.prover.n_workers, 1_usize);
}

// Ensures that various environment variable overrides are successful
#[test]
fn test_config_with_envs() {
    // Establish some environment variable overrides

    let num_workers_env_name = "N_WORKERS";
    let num_workers_override = 10_usize;

    let coordinator_url_env_name = "COORDINATOR_BASE_URL";
    let coordinator_url_override = "my-special-coordinator";

    set_var(num_workers_env_name, num_workers_override.to_string());
    set_var(coordinator_url_env_name, coordinator_url_override);

    let default_config_path = "tests/test_data/default_config.json";
    let cfg: Config = Config::from_file_and_env(default_config_path.to_string())
        .expect("Issue loading test configuration file");

    // Ensure override was successful
    assert_eq!(
        cfg.prover.n_workers, num_workers_override,
        "Number of workers override was not successful"
    );
    assert_eq!(
        cfg.coordinator.base_url, coordinator_url_override,
        "Coordinator base url override was not successful"
    );

    // Unset the test overrides so that other tests are n
    remove_var(num_workers_env_name);
    remove_var(coordinator_url_env_name);
}

// Ensures that our understanding of the configuration file matches `scroll-proving-sdk`
// expectations.  If this test fails, investigate recent updates upstream and ensure
// example `config.json` files are accurate.
#[tokio::test]
async fn test_initialize_from_config_pipeline() {
    let default_config_path = "tests/test_data/default_config.json";
    let cfg: Config = Config::from_file_and_env(default_config_path.to_string())
        .expect("Issue loading test configuration file");

    // Ensure that a cloud prover may be built from the config file
    let _cloud_prover = CloudProver::new(
        cfg.prover
            .cloud
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Missing cloud prover configuration"))
            .expect("Issue instantiating cloud prover in config tests"),
    );
}
