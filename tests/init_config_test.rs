use scroll_proving_sdk::{config::Config, prover::CircuitType};
use sindri_scroll_sdk::prover::CloudProver;

// Ensures that configuration file loading does not require environment variables
#[test]
fn test_config_without_envs() {

    // The utility function below wraps our test code in a closure which is exclusive from
    // `test_config_with_envs`.  No matter what `N_WORKERS` and `COORDINATOR_BASE_URL` are
    // they are unset for the purposes of this test.
    temp_env::with_vars(
        [
            ("N_WORKERS", None::<String>),
            ("COORDINATOR_BASE_URL", None),
        ],
        || {
            let default_config_path = "tests/test_data/default_config.json";
            let cfg: Config = Config::from_file_and_env(default_config_path.to_string())
                .expect("Issue loading test configuration file");

            // Check a few values for consistency with test_data file
            assert_eq!(cfg.prover_name_prefix, "sindri_");
            assert_eq!(cfg.prover.circuit_types[0], CircuitType::Chunk);
            assert_eq!(cfg.prover.circuit_types[1], CircuitType::Batch);
            assert_eq!(cfg.prover.circuit_types[2], CircuitType::Bundle);
            assert_eq!(cfg.db_path, Some("db".to_string()));
            assert_eq!(cfg.prover.n_workers, 1_usize);
        },
    );
}

// Ensures that various environment variable overrides are successful
#[test]
fn test_config_with_envs() {
    // Establish some environment variable overrides

    let num_workers_env_name = "N_WORKERS";
    let num_workers_override = "10";
    let coordinator_url_env_name = "COORDINATOR_BASE_URL";
    let coordinator_url_override = "my-special-coordinator";

    // The utility function below sets environment variables for the duration of the closure.
    // A singleton mutex ensures that the other test, `test_config_without_envs`, is not
    // influenced by these environment variable settings.
    temp_env::with_vars(
        [
            (num_workers_env_name, Some(num_workers_override)),
            (coordinator_url_env_name, Some(coordinator_url_override)),
        ],
        || {
            let default_config_path = "tests/test_data/default_config.json";
            let cfg: Config = Config::from_file_and_env(default_config_path.to_string())
                .expect("Issue loading test configuration file");

            // Ensure override was successful
            assert_eq!(
                cfg.prover.n_workers,
                num_workers_override
                    .parse::<usize>()
                    .expect("Unable to parse `n_workers` value into usize type"),
                "Number of workers override was not successful"
            );
            assert_eq!(
                cfg.coordinator.base_url, coordinator_url_override,
                "Coordinator base url override was not successful"
            );
        },
    );
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
