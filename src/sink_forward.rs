//! Consumer-side forwarders onto typed Chronon Spectra helpers.
//!
//! # Examples
//!
//! ```rust,no_run
//! use chronon_spectra_telemetry::sink_forward::forward_counter;
//! use chrono::Utc;
//! use serde_json::json;
//!
//! forward_counter(
//!     "chronon_runs_started",
//!     json!({"job_name": "nightly", "deployment_shape": "embedded"}),
//!     1,
//!     Utc::now(),
//! );
//! ```

use crate::helpers::{
    ChrononExecutorErrorLogger, ChrononPartitionAssignmentsRecorder, ChrononRemoteErrorsRecorder,
    ChrononRunDurationMsRecorder, ChrononRunLogLogger, ChrononRunsCompletedRecorder,
    ChrononRunsFailedRecorder, ChrononRunsStartedRecorder, ChrononSchedulerLogLogger,
    ChrononSchedulerTicksRecorder,
};

fn field_str(fields: &serde_json::Value, key: &str) -> String {
    fields
        .get(key)
        .and_then(|v| match v {
            serde_json::Value::String(s) => Some(s.clone()),
            serde_json::Value::Bool(b) => Some(b.to_string()),
            serde_json::Value::Number(n) => Some(n.to_string()),
            _ => None,
        })
        .unwrap_or_default()
}

fn field_i64(fields: &serde_json::Value, key: &str) -> i64 {
    fields
        .get(key)
        .and_then(|v| match v {
            serde_json::Value::Number(n) => n.as_i64(),
            serde_json::Value::String(s) => s.parse().ok(),
            _ => None,
        })
        .unwrap_or(0)
}

/// Forward a metric emit onto the matching typed recorder.
pub fn forward_counter(
    name: &str,
    labels: serde_json::Value,
    delta: i64,
    ts: chrono::DateTime<chrono::Utc>,
) {
    match name {
        "chronon_partition_assignments" => {
            ChrononPartitionAssignmentsRecorder::record_at(delta, labels, ts);
        }
        "chronon_remote_errors" => ChrononRemoteErrorsRecorder::record_at(delta, labels, ts),
        "chronon_run_duration_ms" => ChrononRunDurationMsRecorder::record_at(delta, labels, ts),
        "chronon_runs_completed" => ChrononRunsCompletedRecorder::record_at(delta, labels, ts),
        "chronon_runs_failed" => ChrononRunsFailedRecorder::record_at(delta, labels, ts),
        "chronon_runs_started" => ChrononRunsStartedRecorder::record_at(delta, labels, ts),
        "chronon_scheduler_ticks" => ChrononSchedulerTicksRecorder::record_at(delta, labels, ts),
        _ => {}
    }
}

/// Forward an event emit onto the matching typed logger.
pub fn forward_event(table: &str, fields: &serde_json::Value, ts: chrono::DateTime<chrono::Utc>) {
    match table {
        "chronon_executor_error" => ChrononExecutorErrorLogger::log_at(
            field_str(fields, "job_name"),
            field_str(fields, "run_id"),
            field_str(fields, "script_name"),
            field_str(fields, "operation"),
            field_str(fields, "error"),
            ts,
        ),
        "chronon_run_log" => ChrononRunLogLogger::log_at(
            field_str(fields, "run_id"),
            field_str(fields, "job_name"),
            field_str(fields, "script_name"),
            field_str(fields, "status"),
            field_i64(fields, "duration_ms"),
            field_str(fields, "message"),
            ts,
        ),
        "chronon_scheduler_log" => ChrononSchedulerLogLogger::log_at(
            field_str(fields, "component"),
            field_str(fields, "message"),
            ts,
        ),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn field_str_and_i64_happy_coercions() {
        let fields = json!({
            "s": "hello",
            "b": true,
            "n": 42,
            "ns": "7",
        });
        assert_eq!(field_str(&fields, "s"), "hello");
        assert_eq!(field_str(&fields, "b"), "true");
        assert_eq!(field_str(&fields, "n"), "42");
        assert_eq!(field_i64(&fields, "n"), 42);
        assert_eq!(field_i64(&fields, "ns"), 7);
    }

    #[test]
    fn field_str_and_i64_missing_or_invalid_default_sad() {
        let fields = json!({
            "arr": [],
            "obj": {},
            "bad": "not-a-number",
            "null": null,
        });
        assert_eq!(field_str(&fields, "missing"), "");
        assert_eq!(field_str(&fields, "arr"), "");
        assert_eq!(field_str(&fields, "obj"), "");
        assert_eq!(field_str(&fields, "null"), "");
        assert_eq!(field_i64(&fields, "missing"), 0);
        assert_eq!(field_i64(&fields, "bad"), 0);
        assert_eq!(field_i64(&fields, "arr"), 0);
        assert_eq!(field_i64(&fields, "null"), 0);
    }
}
