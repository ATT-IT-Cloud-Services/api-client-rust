use std::collections::HashMap;

/// Client platform sent as `X-Ignite-Client`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClientKind {
    Desktop,
    Mobile,
    #[default]
    Api,
}

impl ClientKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Mobile => "mobile",
            Self::Api => "api",
        }
    }
}

/// Configuration for [`crate::IgniteClient`].
#[derive(Debug, Clone)]
pub struct IgniteClientConfig {
    /// Base URL of the ignite API (e.g. `https://api.example.test`).
    pub base_url: String,
    /// Bearer session token or API key.
    pub token: Option<String>,
    /// Sent as `X-Ignite-Client`. Defaults to [`ClientKind::Api`].
    pub client_kind: ClientKind,
    /// Additional headers merged into every request.
    pub headers: HashMap<String, String>,
    /// Enable sandbox mode: sends `x-sandbox: true` and requires no API key or session.
    pub sandbox: bool,
    /// Force a sandbox error with this machine-readable code (`x-sandbox-error`).
    pub sandbox_error: Option<String>,
    /// Optional HTTP status for a forced sandbox error (`x-sandbox-status`, default 400).
    pub sandbox_status: Option<u16>,
}

impl IgniteClientConfig {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            token: None,
            client_kind: ClientKind::default(),
            headers: HashMap::new(),
            sandbox: false,
            sandbox_error: None,
            sandbox_status: None,
        }
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn optional_token(mut self, token: Option<String>) -> Self {
        self.token = token;
        self
    }

    pub fn client_kind(mut self, kind: ClientKind) -> Self {
        self.client_kind = kind;
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn sandbox(mut self, enabled: bool) -> Self {
        self.sandbox = enabled;
        self
    }

    pub fn sandbox_error(mut self, code: impl Into<String>) -> Self {
        self.sandbox_error = Some(code.into());
        self
    }

    pub fn sandbox_status(mut self, status: u16) -> Self {
        self.sandbox_status = Some(status);
        self
    }
}

pub type QueryValue = serde_json::Value;

pub fn query_params(pairs: &[(&str, QueryValue)]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (key, value) in pairs {
        if value.is_null() {
            continue;
        }
        let s = match value {
            QueryValue::String(v) => v.clone(),
            QueryValue::Number(n) => n.to_string(),
            QueryValue::Bool(b) => b.to_string(),
            _ => continue,
        };
        map.insert((*key).to_string(), s);
    }
    map
}
