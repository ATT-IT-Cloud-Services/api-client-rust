use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct PingResource {
    transport: HttpTransport,
}

impl PingResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /ping`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/ping", query).await
    }
    // </generated-openapi>

}
