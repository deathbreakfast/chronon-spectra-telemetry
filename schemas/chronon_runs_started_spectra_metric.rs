use spectra::spectra_metric;

spectra_metric! {
    ChrononRunsStarted {
        store: "chronon",
        name: "chronon_runs_started",
        version: "0.1.0",
        description: "Chronon script runs started. Labels: job_name, deployment_shape.",
    }
}
