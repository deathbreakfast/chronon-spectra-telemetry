use spectra::spectra_metric;

spectra_metric! {
    ChrononPartitionAssignments {
        store: "chronon",
        name: "chronon_partition_assignments",
        version: "0.1.0",
        description: "Chronon partition lease refresh operations. Labels: partition.",
    }
}
