use chrono::{DateTime, Utc};
use sindri_scroll_sdk::prover::{SindriProofInfoResponse, SindriTaskStatus};
use sindri_scroll_sdk::utils::proving_timestamps_from_response;

#[test]
fn test_proving_timestamps_from_response() {
    let compute_time_sec: f64 = 10.0;
    let date_str: &str = "2024-10-30T12:34:56.000Z"; // intentionally does not include fractional seconds
    let queue_time_sec: f64 = 5.0;
    let response: SindriProofInfoResponse = SindriProofInfoResponse {
        compute_time_sec: Some(compute_time_sec),
        date_created: date_str.to_string(),
        error: None,
        proof: None,
        proof_id: "proof_id".to_string(),
        queue_time_sec: Some(queue_time_sec),
        status: SindriTaskStatus::Success,

        verification_key: None,
    };

    // Create a DateTime<Utc> from the date string, and get the corresponding timestamp.
    let create_time: DateTime<Utc> = date_str.parse().expect("Failed to parse date");
    let create_time_timestamp: f64 = create_time.timestamp() as f64; // number of second in the original timestamp

    // We expect three timestamps (seconds since the epoch).
    let (created_at, started_at, finished_at) = proving_timestamps_from_response(&response);

    assert_eq!(created_at, create_time_timestamp);
    assert_eq!(started_at.unwrap(), create_time_timestamp + queue_time_sec);
    assert_eq!(
        finished_at.unwrap(),
        create_time_timestamp + queue_time_sec + compute_time_sec
    );
}
