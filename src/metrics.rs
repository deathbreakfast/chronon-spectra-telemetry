//! Remapping from [`chronon_telemetry::TelemetrySink`] onto typed Spectra schemas.

use serde_json::Value;
use spectra_core::{try_log_event, try_record_counter, try_record_gauge};

use crate::events::{executor_error_fields, run_log_fields, scheduler_log_fields};
use crate::labels::{
    bound_job_name_label, deployment_shape_from_env, error_class_from_labels, ErrorClass,
};

fn label_value<'a>(labels: &'a [(&'a str, &'a str)], key: &str) -> Option<&'a str> {
    labels.iter().find_map(|(k, v)| (*k == key).then_some(*v))
}

const fn counter_delta(delta: u64) -> i64 {
    if delta > i64::MAX as u64 {
        i64::MAX
    } else {
        delta.cast_signed()
    }
}

/// Handle a [`chronon_telemetry::TelemetrySink::record_counter`] call, adding the
/// allowlisted `deployment_shape` label and remapping to this crate's schema-defined label sets.
pub fn record_counter(name: &str, labels: &[(&str, &str)], delta: u64) {
    let deployment_shape = deployment_shape_from_env();
    match name {
        "chronon_runs_started" | "chronon_runs_completed" => {
            let job_name = bound_job_name_label(
                label_value(labels, "job_name")
                    .or_else(|| label_value(labels, "job"))
                    .unwrap_or("unknown"),
            );
            try_record_counter(
                name,
                &[
                    ("job_name", job_name.as_str()),
                    ("deployment_shape", deployment_shape),
                ],
                counter_delta(delta),
            );
        }
        "chronon_runs_failed" => {
            let job_name = bound_job_name_label(
                label_value(labels, "job_name")
                    .or_else(|| label_value(labels, "job"))
                    .unwrap_or("unknown"),
            );
            let error_class = error_class_from_labels(labels);
            try_record_counter(
                name,
                &[
                    ("job_name", job_name.as_str()),
                    ("deployment_shape", deployment_shape),
                    ("error_class", error_class),
                ],
                counter_delta(delta),
            );
        }
        "chronon_scheduler_ticks" => {
            try_record_counter(
                "chronon_scheduler_ticks",
                &[("deployment_shape", deployment_shape)],
                counter_delta(delta),
            );
        }
        "chronon_remote_errors" => {
            let operation = label_value(labels, "operation").unwrap_or("unknown");
            try_record_counter(name, &[("operation", operation)], counter_delta(delta));
        }
        "chronon_partition_assignments" => {
            let partition = label_value(labels, "partition").unwrap_or("unknown");
            try_record_counter(name, &[("partition", partition)], counter_delta(delta));
        }
        _ => {}
    }
}

/// Handle a [`chronon_telemetry::TelemetrySink::record_gauge`] call.
pub fn record_gauge(name: &str, labels: &[(&str, &str)], value: f64) {
    match name {
        "chronon_run_duration_ms" => {
            let job_name = bound_job_name_label(
                label_value(labels, "job_name")
                    .or_else(|| label_value(labels, "job"))
                    .unwrap_or("unknown"),
            );
            try_record_gauge(name, &[("job_name", job_name.as_str())], value);
        }
        "chronon_partition_assignments" => {
            let partition = label_value(labels, "partition").unwrap_or("unknown");
            try_record_gauge(name, &[("partition", partition)], value);
        }
        _ => {}
    }
}

/// Handle a [`chronon_telemetry::TelemetrySink::log_event`] call, remapping legacy
/// schema/field names onto this crate's typed Spectra event tables.
pub fn log_event(schema: &str, fields: &[(&str, &str)]) {
    match schema {
        "chronon_executor_error" => {
            let job_name =
                bound_job_name_label(label_value(fields, "job_name").unwrap_or("unknown"));
            let run_id = label_value(fields, "run_id").unwrap_or("");
            let script_name = label_value(fields, "script_name").unwrap_or("");
            let operation = label_value(fields, "phase").unwrap_or("script_invoke");
            let error = label_value(fields, "message").unwrap_or("unknown error");
            try_log_event(
                "chronon_executor_error",
                &executor_error_fields(job_name.as_str(), run_id, script_name, operation, error),
            );

            let deployment_shape = deployment_shape_from_env();
            let error_class = ErrorClass::from_message(error).as_str();
            try_record_counter(
                "chronon_runs_failed",
                &[
                    ("job_name", job_name.as_str()),
                    ("deployment_shape", deployment_shape),
                    ("error_class", error_class),
                ],
                1,
            );
        }
        "chronon_run_failed" => {
            let run_id = label_value(fields, "run_id").unwrap_or("");
            let job_name = label_value(fields, "job").unwrap_or("unknown");
            let message = label_value(fields, "error").unwrap_or("run failed");
            try_log_event(
                "chronon_run_log",
                &run_log_fields(run_id, job_name, "", "failed", 0, message),
            );
        }
        "chronon_scheduler_warn" | "chronon_scheduler_info" | "chronon_scheduler_error" => {
            let component = label_value(fields, "component").unwrap_or("scheduler");
            let message = label_value(fields, "message").unwrap_or("");
            try_log_event(
                "chronon_scheduler_log",
                &scheduler_log_fields(component, message),
            );
        }
        "chronon_run_log" | "chronon_scheduler_log" => {
            let object = fields
                .iter()
                .fold(serde_json::Map::new(), |mut acc, (k, v)| {
                    acc.insert((*k).to_string(), Value::String((*v).to_string()));
                    acc
                });
            try_log_event(schema, &Value::Object(object));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_value_and_counter_delta_happy() {
        let labels = [("job_name", "nightly"), ("job", "legacy")];
        assert_eq!(label_value(&labels, "job_name"), Some("nightly"));
        assert_eq!(label_value(&labels, "job"), Some("legacy"));
        assert_eq!(counter_delta(7), 7);
        assert_eq!(counter_delta(0), 0);
    }

    #[test]
    fn label_value_missing_and_counter_delta_overflow_sad() {
        let labels = [("job_name", "nightly")];
        assert_eq!(label_value(&labels, "missing"), None);
        assert_eq!(counter_delta(u64::MAX), i64::MAX);
    }
}
