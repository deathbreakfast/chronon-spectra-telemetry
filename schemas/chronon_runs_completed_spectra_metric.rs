use spectra::spectra_metric;

spectra_metric! {
    ChrononRunsCompleted {
        store: "chronon",
        name: "chronon_runs_completed",
        version: "0.1.0",
        description: "Chronon script runs completed successfully. Labels: job_name, deployment_shape.",
    }
}
