//! Spectra-backed self-telemetry for [Chronon].
//!
//! Typed event/metric schemas, Photon topic helpers, and a
//! [`TelemetrySink`](chronon_telemetry::TelemetrySink) adapter for Chronon runtime signals.
//!
//! [Chronon]'s [`TelemetrySink`](chronon_telemetry::TelemetrySink) trait is deliberately
//! backend-agnostic: Chronon calls `record_counter` / `record_gauge` / `log_event` on whatever
//! implementation the host installs.
//!
//! This crate is that implementation for hosts that already emit their own telemetry through
//! [Spectra]: [`SpectraTelemetrySink`] forwards each call into `spectra-core`, remapping raw
//! label/field names onto this crate's typed schemas along the way, and [`install_from_env`]
//! resolves and caches it based on `CHRONON_TELEMETRY`.
//!
//! [Chronon]: https://github.com/unified-field-dev/chronon
//! [Spectra]: https://github.com/unified-field-dev/spectra
//!
//! ## Features
//!
//! - **`TelemetrySink` install** — [`SpectraTelemetrySink`] implements
//!   [`chronon_telemetry::TelemetrySink`] by routing counters, gauges, and events through
//!   `spectra-core`.
//! - **Env-driven install** — [`install_from_env`] reads `CHRONON_TELEMETRY`
//!   (`off` / `console` / default-to-Spectra) and caches the matching sink for the process.
//! - **Typed schemas** — Spectra DSL schemas for Chronon's own run/scheduler tables and
//!   counters, registered via `inventory` when linked into a host.
//! - **Topic + codegen helpers** — generated `*Payload` / `*_TOPIC` DTOs and `*Recorder` /
//!   `*Logger` types, importable straight from the crate root (e.g.
//!   [`ChrononRunsStartedRecorder`]).
//! - **Consumer-side forwarding** — [`sink_forward`] re-dispatches raw metric/event emits onto
//!   the matching typed Spectra recorder, for sink consumers that re-emit Chronon's signals
//!   downstream instead of calling the typed helpers directly.
//!

//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---|---|
//! | Install | [`install_from_env`] / [`SpectraTelemetrySink`] |
//! | Sink forwarding | [`sink_forward`] |
//!
//! Labels (`job_name` / `deployment_shape` / `error_class`) are attached automatically by the
//! installed sink (see `CHRONON_DEPLOYMENT_SHAPE` below); this crate has no dedicated label types.
//!
//! ## Generated schemas & topics
//!
//! Typed `*Recorder` / `*Logger` / `*Payload` / `*_TOPIC` symbols are re-exported at the crate
//! root and grouped under [`helpers`] and [`topics`]. One mid-level pattern for both surfaces:
//!
//! ```rust,no_run
//! use chronon_spectra_telemetry::{
//!     ChrononRunsStartedPayload, ChrononRunsStartedRecorder, CHRONON_RUNS_STARTED_TOPIC,
//! };
//!
//! ChrononRunsStartedRecorder::record(
//!     1,
//!     serde_json::json!({"job_name": "nightly", "deployment_shape": "embedded"}),
//! );
//! assert_eq!(ChrononRunsStartedPayload::topic(), CHRONON_RUNS_STARTED_TOPIC);
//! ```
//!
//! See [`helpers`] for the full recorder/logger set and [`topics`] for transport DTOs.
//!
//! ## Environment
//!
//! | Variable | Values | Default |
//! |----------|--------|---------|
//! | `CHRONON_TELEMETRY` | `off`, `console`, `spectra` | `spectra` (when Spectra is configured) |
//! | `CHRONON_DEPLOYMENT_SHAPE` | any string | derived from `CHRONON_INSTANCE_ROLE` / `CHRONON_REMOTE_BASE_URL`, else `embedded` |
//!
//! # Getting started
//!
//! Install the sink once at startup and hand it to your `ChrononBuilder`:
//!
//! ```rust,no_run
//! use chronon_spectra_telemetry::install_from_env;
//!
//! // Reads `CHRONON_TELEMETRY` (off / console / default-to-Spectra), caching the
//! // resolved sink for the life of the process.
//! let sink = install_from_env();
//!
//! // ChrononBuilder::new().telemetry_sink(sink)...
//! let _ = sink;
//! ```
//!
//! ## Where to look next
//!
//! - [`install_from_env`] / [`SpectraTelemetrySink`] — process-wide `TelemetrySink` bootstrap
//! - [`sink_forward`] — forwarders for sink consumers that re-emit onto the typed Spectra recorders
//! - [`helpers`] / [`topics`] — generated recorders, loggers, payloads, and topic constants

#![allow(clippy::too_long_first_doc_paragraph)]

mod events;
/// Typed emit helpers from Chronon Spectra schemas.
pub mod helpers;
mod install;
mod labels;
mod metrics;
mod sanitize;
// macro-generated Spectra schema types; documented via each schema's `description`
#[allow(missing_docs)]
mod schemas;
/// Forwarders for sink consumers that re-dispatch raw metric/event emits onto the matching
/// typed Spectra recorder generated from this crate's schemas.
///
/// # Examples
///
/// ```rust,no_run
/// use chronon_spectra_telemetry::sink_forward;
/// use chrono::Utc;
/// use serde_json::json;
///
/// let ts = Utc::now();
/// sink_forward::forward_counter(
///     "chronon_runs_started",
///     json!({"job_name": "nightly", "deployment_shape": "embedded"}),
///     1,
///     ts,
/// );
/// ```
pub mod sink_forward;
/// Transport `*Payload` / `*_TOPIC` DTOs from Chronon Spectra schemas.
pub mod topics;

pub use helpers::*;
pub use topics::*;

#[allow(deprecated)]
pub use install::install_ops_log_from_env;
pub use install::{install_from_env, SpectraTelemetrySink};
