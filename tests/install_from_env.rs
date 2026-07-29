//! Process-isolated install coverage (`OnceLock` caches the first resolution).
#![allow(missing_docs)]

use chronon_spectra_telemetry::install_from_env;

#[test]
fn install_from_env_off_aliases_return_usable_sink_happy() {
    // First resolution in this process: off aliases → NoOpSink.
    std::env::set_var("CHRONON_TELEMETRY", "off");
    let sink = install_from_env();
    sink.record_counter("chronon_scheduler_ticks", &[], 1);
    sink.record_gauge("chronon_run_duration_ms", &[], 0.0);
    sink.log_event(
        "chronon_scheduler_log",
        &[("component", "scheduler"), ("message", "quiet")],
    );

    // OnceLock: second install returns the same cached sink (still usable after env flip).
    std::env::set_var("CHRONON_TELEMETRY", "spectra");
    #[allow(deprecated)]
    let again = chronon_spectra_telemetry::install_ops_log_from_env();
    again.record_counter("chronon_runs_started", &[("job_name", "j")], 1);
    assert!(std::sync::Arc::ptr_eq(&sink, &again));
}
