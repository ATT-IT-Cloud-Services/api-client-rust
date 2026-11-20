use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct McpResource {
    transport: HttpTransport,
}

impl McpResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `DELETE /mcp`.
    pub async fn remove(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/mcp", query, body).await
    }

    /// Generated `GET /mcp`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/mcp", query).await
    }

    /// Generated `POST /mcp`.
    pub async fn create(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/mcp", body).await
    }
    // </generated-openapi>

}
