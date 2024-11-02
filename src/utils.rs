use crate::prover::SindriProofInfoResponse;
use chrono::{DateTime, FixedOffset};

// Convert an ISO 8601 string to a f64 timestamp.
fn iso8601_to_f64(iso8601_str: &str) -> Result<f64, chrono::ParseError> {
    // Parse the ISO 8601 string into a DateTime<Utc> object
    let datetime: DateTime<FixedOffset> = match DateTime::parse_from_rfc3339(&iso8601_str) {
        Ok(datetime) => datetime,
        Err(error) => return Err(error),
    };

    // Convert the DateTime<Utc> object to a timestamp (seconds since the Unix epoch)
    let timestamp = datetime.timestamp() as f64;

    // Convert the nanoseconds part to f64 and add it to the timestamp
    let nanoseconds = datetime.timestamp_subsec_nanos() as f64;
    let timestamp_f64 = timestamp + nanoseconds / 1_000_000_000.0;

    Ok(timestamp_f64)
}

// Return the created, start, and finish times of the proof task.
pub fn proving_timestamps_from_response(
    resp: &SindriProofInfoResponse,
) -> (f64, Option<f64>, Option<f64>) {
    let mut started_at: Option<f64> = None;
    let mut finished_at: Option<f64> = None;

    let created_at: f64 = match iso8601_to_f64(&resp.date_created) {
        Ok(created_at) => created_at,
        Err(_) => return (0.0, None, None),
    };
    let compute_time_sec = resp.compute_time_sec.unwrap_or(0.0);
    let queue_time_sec = resp.queue_time_sec.unwrap_or(0.0);

    if queue_time_sec > 0.0 {
        started_at = Some(created_at + queue_time_sec);
        if compute_time_sec > 0.0 {
            finished_at = Some(created_at + queue_time_sec + compute_time_sec);
        }
    }

    log::trace!(
        "resp.date_created: {:?} created_at: {} started_at: {:?} finished_at: {:?}",
        resp.date_created,
        created_at,
        started_at,
        finished_at
    );
    (created_at, started_at, finished_at)
}
