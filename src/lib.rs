//! Official ignite API client.
//!
//! Machine-facing SDK — no localized error messages. API failures return
//! [`ApiError`] with a machine-readable `code`, HTTP `status`, and raw `payload`.
//! Map `code` to your own i18n layer.

mod client;
mod config;
mod error;
pub mod error_codes;
pub mod models;
mod resources;
mod transport;

pub use client::IgniteClient;
pub use config::{ClientKind, IgniteClientConfig};
pub use error::{extract_error_code, ApiError, IgniteError};
pub use error_codes::{ALL_API_ERROR_CODES, API_ERROR_UNKNOWN};
pub use models::*;
pub use resources::*;
