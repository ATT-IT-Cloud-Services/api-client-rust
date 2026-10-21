use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::Value;
use url::Url;

use crate::config::{ClientKind, IgniteClientConfig};
use crate::error::{ApiError, IgniteError};

#[derive(Clone)]
pub struct HttpTransport {
    inner: Arc<TransportInner>,
}

struct TransportInner {
    base_url: String,
    client: Client,
    client_kind: ClientKind,
    token: RwLock<Option<String>>,
    extra_headers: HashMap<String, String>,
    sandbox: bool,
    sandbox_error: Option<String>,
    sandbox_status: Option<u16>,
}

pub(crate) fn encode_path(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(char::from(*byte));
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

impl HttpTransport {
    pub fn new(config: IgniteClientConfig) -> Result<Self, reqwest::Error> {
        let base_url = config.base_url.trim_end_matches('/').to_string();
        let client = Client::builder().build()?;
        Ok(Self {
            inner: Arc::new(TransportInner {
                base_url,
                client,
                client_kind: config.client_kind,
                token: RwLock::new(config.token),
                extra_headers: config.headers,
                sandbox: config.sandbox,
                sandbox_error: config.sandbox_error,
                sandbox_status: config.sandbox_status,
            }),
        })
    }

    pub fn set_token(&self, token: Option<String>) {
        if let Ok(mut guard) = self.inner.token.write() {
            *guard = token;
        }
    }

    pub fn token(&self) -> Option<String> {
        self.inner.token.read().ok().and_then(|g| g.clone())
    }

    pub async fn get<T>(
        &self,
        path: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<T, IgniteError>
    where
        T: DeserializeOwned,
    {
        self.request(Method::GET, path, query, None).await
    }

    pub async fn post<T>(&self, path: &str, body: Option<Value>) -> Result<T, IgniteError>
    where
        T: DeserializeOwned,
    {
        self.request(Method::POST, path, None, body).await
    }

    pub async fn put<T>(&self, path: &str, body: Option<Value>) -> Result<T, IgniteError>
    where
        T: DeserializeOwned,
    {
        self.request(Method::PUT, path, None, body).await
    }

    pub async fn patch<T>(&self, path: &str, body: Option<Value>) -> Result<T, IgniteError>
    where
        T: DeserializeOwned,
    {
        self.request(Method::PATCH, path, None, body).await
    }

    pub async fn delete<T>(
        &self,
        path: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<T, IgniteError>
    where
        T: DeserializeOwned,
    {
        self.request(Method::DELETE, path, query, body).await
    }

    fn build_url(
        &self,
        path: &str,
        query: &Option<HashMap<String, String>>,
    ) -> Result<Url, IgniteError> {
        let normalized = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{path}")
        };
        let mut url = Url::parse(&format!(
            "{}{normalized}",
            self.inner.base_url.trim_end_matches('/')
        ))
        .map_err(|e| IgniteError::InvalidUrl(e.to_string()))?;
        if let Some(params) = query {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in params {
                pairs.append_pair(key, value);
            }
        }
        Ok(url)
    }

    fn build_headers(
        &self,
        extra: &HashMap<String, String>,
        is_json: bool,
    ) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert(
            "X-Ignite-Client".to_string(),
            self.inner.client_kind.as_str().to_string(),
        );
        if is_json {
            headers.insert("Content-Type".to_string(), "application/json".to_string());
        }
        if let Ok(guard) = self.inner.token.read() {
            if let Some(token) = guard.as_ref() {
                headers.insert("Authorization".to_string(), format!("Bearer {token}"));
            }
        }
        if self.inner.sandbox {
            headers.insert("x-sandbox".to_string(), "true".to_string());
        }
        if let Some(code) = &self.inner.sandbox_error {
            headers.insert("x-sandbox-error".to_string(), code.clone());
        }
        if let Some(status) = self.inner.sandbox_status {
            headers.insert("x-sandbox-status".to_string(), status.to_string());
        }
        headers.extend(self.inner.extra_headers.clone());
        headers.extend(extra.clone());
        headers
    }

    async fn request<T>(
        &self,
        method: Method,
        path: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<T, IgniteError>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(path, &query)?;
        let is_json = body.is_some();
        let headers = self.build_headers(&HashMap::new(), is_json);

        let mut builder: RequestBuilder = self.inner.client.request(method, url);
        for (key, value) in headers {
            builder = builder.header(key, value);
        }
        if let Some(json_body) = body {
            builder = builder.json(&json_body);
        }

        let response = builder.send().await?;
        let http_status = response.status().as_u16();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let parsed: Value = if content_type.contains("application/json") {
            response.json().await.unwrap_or(Value::Null)
        } else if http_status == 204 {
            Value::Null
        } else {
            let text = response.text().await.unwrap_or_default();
            if text.is_empty() {
                Value::Null
            } else {
                Value::String(text)
            }
        };

        let semantic_status = parsed
            .get("status")
            .and_then(Value::as_u64)
            .map(|s| s as u16)
            .unwrap_or(http_status);

        let api_failed = if parsed.is_object() {
            is_api_failure(semantic_status, &parsed)
        } else {
            !(200..300).contains(&http_status)
        };

        if api_failed {
            if semantic_status == 401 || is_unauthenticated(&parsed) {
                self.set_token(None);
            }
            return Err(ApiError::from_response(semantic_status, parsed).into());
        }

        if parsed.is_null() {
            return serde_json::from_value(Value::Object(Default::default()))
                .map_err(IgniteError::from);
        }

        serde_json::from_value(parsed).map_err(IgniteError::from)
    }
}

fn is_api_failure(semantic_status: u16, body: &Value) -> bool {
    if let Some(ok) = body.get("ok").and_then(Value::as_bool) {
        if !ok {
            return true;
        }
    }
    if body.get("success") == Some(&Value::Bool(false)) {
        return true;
    }
    !(200..300).contains(&semantic_status)
}

fn is_unauthenticated(body: &Value) -> bool {
    body.get("errorCode")
        .and_then(Value::as_str)
        .map(|c| c == "unauthenticated")
        .unwrap_or(false)
        || body
            .get("message")
            .and_then(Value::as_str)
            .map(|m| m == "unauthenticated")
            .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn is_api_failure_detects_success_false() {
        assert!(is_api_failure(200, &json!({ "success": false })));
    }

    #[test]
    fn is_api_failure_detects_ok_false() {
        assert!(is_api_failure(200, &json!({ "ok": false })));
    }

    #[test]
    fn is_api_failure_accepts_success_true() {
        assert!(!is_api_failure(200, &json!({ "success": true })));
    }

    #[test]
    fn build_url_adds_query_params() {
        let transport =
            HttpTransport::new(IgniteClientConfig::new("https://api.example.test")).unwrap();
        let mut query = HashMap::new();
        query.insert("limit".to_string(), "50".to_string());
        let url = transport.build_url("/services", &Some(query)).unwrap();
        assert_eq!(url.as_str(), "https://api.example.test/services?limit=50");
    }

    #[test]
    fn build_url_maps_plus_prefix() {
        let transport =
            HttpTransport::new(IgniteClientConfig::new("https://api.example.test")).unwrap();
        let url = transport.build_url("/plus/status", &None).unwrap();
        assert_eq!(url.as_str(), "https://api.example.test/plus/status");
    }

    #[test]
    fn build_headers_adds_sandbox_headers() {
        let config = IgniteClientConfig::new("https://api.example.test")
            .sandbox(true)
            .sandbox_error("rate_limited")
            .sandbox_status(429);
        let transport = HttpTransport::new(config).unwrap();
        let headers = transport.build_headers(&HashMap::new(), false);
        assert_eq!(headers.get("x-sandbox").map(String::as_str), Some("true"));
        assert_eq!(
            headers.get("x-sandbox-error").map(String::as_str),
            Some("rate_limited")
        );
        assert_eq!(
            headers.get("x-sandbox-status").map(String::as_str),
            Some("429")
        );
    }

    #[test]
    fn build_headers_omits_sandbox_when_disabled() {
        let transport =
            HttpTransport::new(IgniteClientConfig::new("https://api.example.test")).unwrap();
        let headers = transport.build_headers(&HashMap::new(), false);
        assert!(!headers.contains_key("x-sandbox"));
        assert!(!headers.contains_key("x-sandbox-error"));
        assert!(!headers.contains_key("x-sandbox-status"));
    }
}
