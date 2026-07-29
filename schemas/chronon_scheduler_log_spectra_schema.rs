use spectra::spectra_schema;

spectra_schema! {
    ChrononSchedulerLog {
        store: "chronon",
        table: "chronon_scheduler_log",
        version: "0.1.0",
        description: "Chronon scheduler and backend operational notes.",
        fields: [
            component: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            message: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
        ],
    }
}
