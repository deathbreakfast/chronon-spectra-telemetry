//! Process-isolated default (`spectra`) install path.
#![allow(missing_docs)]

use chronon_spectra_telemetry::install_from_env;

#[test]
fn install_from_env_default_uses_spectra_sink_happy() {
    std::env::remove_var("CHRONON_TELEMETRY");
    let sink = install_from_env();
    sink.record_counter("chronon_runs_started", &[("job_name", "j")], 1);
    sink.log_event(
        "chronon_scheduler_log",
        &[("component", "scheduler"), ("message", "ready")],
    );
    // Cached: second call is the same Arc even if env changes afterward.
    std::env::set_var("CHRONON_TELEMETRY", "off");
    let again = install_from_env();
    assert!(std::sync::Arc::ptr_eq(&sink, &again));
}
