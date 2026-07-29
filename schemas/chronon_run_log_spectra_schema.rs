use spectra::spectra_schema;

spectra_schema! {
    ChrononRunLog {
        store: "chronon",
        table: "chronon_run_log",
        version: "0.1.0",
        description: "Chronon run lifecycle trace (start, complete, fail).",
        fields: [
            run_id: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            job_name: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            script_name: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            status: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            duration_ms: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            message: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
        ],
    }
}
