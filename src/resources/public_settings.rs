use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct PublicSettingsResource {
    transport: HttpTransport,
}

impl PublicSettingsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /public-settings`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public-settings", query).await
    }
    // </generated-openapi>

}
