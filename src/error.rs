use crate::error_codes::API_ERROR_UNKNOWN;
use serde_json::Value;
use thiserror::Error;

/// Thrown when an ignite API request fails.
/// Contains only machine-readable data — no localized messages.
#[derive(Debug, Error, Clone)]
#[error("{code}")]
pub struct ApiError {
    pub code: String,
    pub status: u16,
    pub payload: Value,
}

impl ApiError {
    pub fn new(code: impl Into<String>, status: u16, payload: Value) -> Self {
        Self {
            code: code.into(),
            status,
            payload,
        }
    }

    pub fn from_response(status: u16, payload: Value) -> Self {
        let code = extract_error_code(&payload).unwrap_or_else(|| API_ERROR_UNKNOWN.to_string());
        Self::new(code, status, payload)
    }
}

/// Extracts a machine-readable error code from an API error payload.
pub fn extract_error_code(data: &Value) -> Option<String> {
    let obj = data.as_object()?;

    if let Some(code) = obj.get("errorCode").and_then(Value::as_str) {
        let trimmed = code.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    if let Some(error) = obj.get("error").and_then(Value::as_str) {
        let trimmed = error.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    if let Some(message) = obj.get("message").and_then(Value::as_str) {
        let trimmed = message.trim();
        if trimmed == "unauthenticated" {
            return Some("unauthenticated".to_string());
        }
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    None
}

#[derive(Debug, Error)]
pub enum IgniteError {
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("invalid URL: {0}")]
    InvalidUrl(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extract_error_code_prefers_error_code_field() {
        let payload = json!({ "errorCode": "invoiceNotFound" });
        assert_eq!(
            extract_error_code(&payload).as_deref(),
            Some("invoiceNotFound")
        );
    }

    #[test]
    fn extract_error_code_unauthenticated_message() {
        let payload = json!({ "message": "unauthenticated" });
        assert_eq!(
            extract_error_code(&payload).as_deref(),
            Some("unauthenticated")
        );
    }

    #[test]
    fn api_error_from_response_uses_unknown_fallback() {
        let err = ApiError::from_response(500, json!({}));
        assert_eq!(err.code, API_ERROR_UNKNOWN);
        assert_eq!(err.status, 500);
    }
}
