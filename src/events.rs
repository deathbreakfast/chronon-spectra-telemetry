//! Crate-internal JSON field builders for Chronon self-telemetry events.

use serde_json::{json, Value};

use crate::sanitize::sanitize_error_message;

/// Build the JSON field set for a `chronon_run_log` row.
pub fn run_log_fields(
    run_id: &str,
    job_name: &str,
    script_name: &str,
    status: &str,
    duration_ms: i64,
    message: &str,
) -> Value {
    json!({
        "run_id": run_id,
        "job_name": job_name,
        "script_name": script_name,
        "status": status,
        "duration_ms": duration_ms,
        "message": sanitize_error_message(message),
    })
}

/// Build the JSON field set for a `chronon_executor_error` row.
pub fn executor_error_fields(
    job_name: &str,
    run_id: &str,
    script_name: &str,
    operation: &str,
    error: &str,
) -> Value {
    json!({
        "job_name": job_name,
        "run_id": run_id,
        "script_name": script_name,
        "operation": operation,
        "error": sanitize_error_message(error),
    })
}

/// Build the JSON field set for a `chronon_scheduler_log` row.
pub fn scheduler_log_fields(component: &str, message: &str) -> Value {
    json!({
        "component": component,
        "message": sanitize_error_message(message),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sanitize::MAX_ERROR_MESSAGE_CHARS;

    #[test]
    fn run_log_fields_happy_shape() {
        let fields = run_log_fields("r1", "nightly", "script_a", "completed", 42, "ok");
        assert_eq!(fields["run_id"], "r1");
        assert_eq!(fields["job_name"], "nightly");
        assert_eq!(fields["script_name"], "script_a");
        assert_eq!(fields["status"], "completed");
        assert_eq!(fields["duration_ms"], 42);
        assert_eq!(fields["message"], "ok");
    }

    #[test]
    fn executor_and_scheduler_fields_happy_shape() {
        let exec = executor_error_fields("j", "r", "s", "script_invoke", "boom");
        assert_eq!(exec["job_name"], "j");
        assert_eq!(exec["run_id"], "r");
        assert_eq!(exec["script_name"], "s");
        assert_eq!(exec["operation"], "script_invoke");
        assert_eq!(exec["error"], "boom");

        let sched = scheduler_log_fields("scheduler", "tick");
        assert_eq!(sched["component"], "scheduler");
        assert_eq!(sched["message"], "tick");
    }

    #[test]
    fn event_field_builders_sanitize_secrets_and_truncate_sad() {
        let secret = format!("failed password={}", "hunter2");
        let exec = executor_error_fields("j", "r", "s", "op", &secret);
        let msg = exec["error"].as_str().unwrap_or("");
        assert!(msg.contains("[redacted]"));
        assert!(!msg.contains("hunter2"));

        let long = "m".repeat(800);
        let run = run_log_fields("r", "j", "s", "failed", 0, &long);
        let sched = scheduler_log_fields("scheduler", &long);
        for fields in [&run, &sched] {
            let text = fields["message"].as_str().unwrap_or("");
            assert!(text.chars().count() <= MAX_ERROR_MESSAGE_CHARS + 1);
            assert!(text.ends_with('…'));
        }
    }
}
