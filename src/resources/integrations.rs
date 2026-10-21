use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct IntegrationsResource {
    transport: HttpTransport,
}

impl IntegrationsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `DELETE /integrations/modrinth/account`.
    pub async fn delete_modrinth_account(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/integrations/modrinth/account", query, body).await
    }

    /// Generated `GET /integrations/modrinth/account`.
    pub async fn modrinth_account(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/integrations/modrinth/account", query).await
    }

    /// Generated `GET /integrations/modrinth/callback`.
    pub async fn modrinth_callback(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/integrations/modrinth/callback", query).await
    }

    /// Generated `GET /integrations/modrinth/connect`.
    pub async fn modrinth_connect(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/integrations/modrinth/connect", query).await
    }
    // </generated-openapi>

}
