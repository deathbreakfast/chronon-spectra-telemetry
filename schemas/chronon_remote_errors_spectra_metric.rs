use spectra::spectra_metric;

spectra_metric! {
    ChrononRemoteErrors {
        store: "chronon",
        name: "chronon_remote_errors",
        version: "0.1.0",
        description: "Chronon remote HTTP coordinator errors. Labels: operation.",
        level: Error,
    }
}
