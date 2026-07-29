use spectra::spectra_metric;

spectra_metric! {
    ChrononRunDurationMs {
        store: "chronon",
        name: "chronon_run_duration_ms",
        version: "0.1.0",
        description: "Chronon run wall-clock duration in milliseconds. Labels: job_name.",
        level: Trace,
        default_sample_rate: 0.1,
    }
}
