//! Chronon Spectra schema modules (inventory + typed helpers + topics).
//!
//! Each module wraps one `spectra_schema!` / `spectra_metric!` invocation under
//! `schemas/` at the repo root (relative to this file, one directory up from `src/`); the
//! macro generates the row/payload types, the typed logger/recorder, the Photon topic
//! constant, and the `inventory` registration for that table or counter/gauge. This module
//! itself is private — see [`crate::helpers`] and [`crate::topics`] for the re-exported,
//! effectively-public names.
#![allow(clippy::too_many_arguments, clippy::pedantic, clippy::nursery)]

/// `chronon_executor_error` event schema (see `schemas/chronon_executor_error_spectra_schema.rs`).
#[path = "../schemas/chronon_executor_error_spectra_schema.rs"]
pub mod chronon_executor_error;

/// `chronon_partition_assignments` gauge schema (see
/// `schemas/chronon_partition_assignments_spectra_metric.rs`).
#[path = "../schemas/chronon_partition_assignments_spectra_metric.rs"]
pub mod chronon_partition_assignments;

/// `chronon_remote_errors` counter schema (see `schemas/chronon_remote_errors_spectra_metric.rs`).
#[path = "../schemas/chronon_remote_errors_spectra_metric.rs"]
pub mod chronon_remote_errors;

/// `chronon_run_duration_ms` gauge schema (see `schemas/chronon_run_duration_ms_spectra_metric.rs`).
#[path = "../schemas/chronon_run_duration_ms_spectra_metric.rs"]
pub mod chronon_run_duration_ms;

/// `chronon_run_log` event schema (see `schemas/chronon_run_log_spectra_schema.rs`).
#[path = "../schemas/chronon_run_log_spectra_schema.rs"]
pub mod chronon_run_log;

/// `chronon_runs_completed` counter schema (see `schemas/chronon_runs_completed_spectra_metric.rs`).
#[path = "../schemas/chronon_runs_completed_spectra_metric.rs"]
pub mod chronon_runs_completed;

/// `chronon_runs_failed` counter schema (see `schemas/chronon_runs_failed_spectra_metric.rs`).
#[path = "../schemas/chronon_runs_failed_spectra_metric.rs"]
pub mod chronon_runs_failed;

/// `chronon_runs_started` counter schema (see `schemas/chronon_runs_started_spectra_metric.rs`).
#[path = "../schemas/chronon_runs_started_spectra_metric.rs"]
pub mod chronon_runs_started;

/// `chronon_scheduler_log` event schema (see `schemas/chronon_scheduler_log_spectra_schema.rs`).
#[path = "../schemas/chronon_scheduler_log_spectra_schema.rs"]
pub mod chronon_scheduler_log;

/// `chronon_scheduler_ticks` counter schema (see `schemas/chronon_scheduler_ticks_spectra_metric.rs`).
#[path = "../schemas/chronon_scheduler_ticks_spectra_metric.rs"]
pub mod chronon_scheduler_ticks;
