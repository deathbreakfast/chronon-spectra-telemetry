use spectra::spectra_metric;

spectra_metric! {
    ChrononRunsFailed {
        store: "chronon",
        name: "chronon_runs_failed",
        version: "0.1.0",
        description: "Chronon script runs failed. Labels: job_name, deployment_shape, error_class.",
        level: Error,
    }
}
