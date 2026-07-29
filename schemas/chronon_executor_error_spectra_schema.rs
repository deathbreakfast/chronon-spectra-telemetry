use spectra::spectra_schema;

spectra_schema! {
    ChrononExecutorError {
        store: "chronon",
        table: "chronon_executor_error",
        version: "0.1.0",
        description: "Chronon executor failures (registry lookup, Valence build, script invoke).",
        level: Error,
        fields: [
            job_name: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            run_id: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            script_name: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            operation: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            error: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
        ],
    }
}
