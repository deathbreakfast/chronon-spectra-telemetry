//! Process-isolated `CHRONON_TELEMETRY=console` install path.
#![allow(missing_docs)]

use chronon_spectra_telemetry::install_from_env;

#[test]
fn install_from_env_console_returns_usable_sink_happy() {
    std::env::set_var("CHRONON_TELEMETRY", "console");
    let sink = install_from_env();
    sink.record_counter("chronon_runs_started", &[("job_name", "j")], 1);
    sink.record_gauge("chronon_run_duration_ms", &[("job_name", "j")], 1.0);
    sink.log_event(
        "chronon_scheduler_log",
        &[("component", "scheduler"), ("message", "console")],
    );
}
