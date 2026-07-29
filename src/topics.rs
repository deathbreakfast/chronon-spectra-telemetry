//! Transport `*Payload` / `*_TOPIC` DTOs from Chronon Spectra schemas.
//!
//! Each `*_TOPIC` constant is the Photon topic name a Spectra sink publishes to, and the
//! matching `*Payload` is the serialized wire type carried on that topic.
//!
//! # Examples
//!
//! ```rust,no_run
//! use chronon_spectra_telemetry::topics::{ChrononRunsStartedPayload, CHRONON_RUNS_STARTED_TOPIC};
//!
//! assert_eq!(ChrononRunsStartedPayload::topic(), CHRONON_RUNS_STARTED_TOPIC);
//! ```

/// Payload and topic constant for `chronon_executor_error`.
pub use crate::schemas::chronon_executor_error::{
    ChrononExecutorErrorPayload, CHRONON_EXECUTOR_ERROR_TOPIC,
};
/// Payload and topic constant for `chronon_partition_assignments`.
pub use crate::schemas::chronon_partition_assignments::{
    ChrononPartitionAssignmentsPayload, CHRONON_PARTITION_ASSIGNMENTS_TOPIC,
};
/// Payload and topic constant for `chronon_remote_errors`.
pub use crate::schemas::chronon_remote_errors::{
    ChrononRemoteErrorsPayload, CHRONON_REMOTE_ERRORS_TOPIC,
};
/// Payload and topic constant for `chronon_run_duration_ms`.
pub use crate::schemas::chronon_run_duration_ms::{
    ChrononRunDurationMsPayload, CHRONON_RUN_DURATION_MS_TOPIC,
};
/// Payload and topic constant for `chronon_run_log`.
pub use crate::schemas::chronon_run_log::{ChrononRunLogPayload, CHRONON_RUN_LOG_TOPIC};
/// Payload and topic constant for `chronon_runs_completed`.
pub use crate::schemas::chronon_runs_completed::{
    ChrononRunsCompletedPayload, CHRONON_RUNS_COMPLETED_TOPIC,
};
/// Payload and topic constant for `chronon_runs_failed`.
pub use crate::schemas::chronon_runs_failed::{
    ChrononRunsFailedPayload, CHRONON_RUNS_FAILED_TOPIC,
};
/// Payload and topic constant for `chronon_runs_started`.
pub use crate::schemas::chronon_runs_started::{
    ChrononRunsStartedPayload, CHRONON_RUNS_STARTED_TOPIC,
};
/// Payload and topic constant for `chronon_scheduler_log`.
pub use crate::schemas::chronon_scheduler_log::{
    ChrononSchedulerLogPayload, CHRONON_SCHEDULER_LOG_TOPIC,
};
/// Payload and topic constant for `chronon_scheduler_ticks`.
pub use crate::schemas::chronon_scheduler_ticks::{
    ChrononSchedulerTicksPayload, CHRONON_SCHEDULER_TICKS_TOPIC,
};
