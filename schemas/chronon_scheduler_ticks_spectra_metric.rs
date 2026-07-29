use spectra::spectra_metric;

spectra_metric! {
    ChrononSchedulerTicks {
        store: "chronon",
        name: "chronon_scheduler_ticks",
        version: "0.1.0",
        description: "Chronon coordinator tick iterations. Labels: deployment_shape.",
        level: Debug,
    }
}
