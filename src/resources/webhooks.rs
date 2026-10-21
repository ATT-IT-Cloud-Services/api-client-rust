use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct WebhooksResource {
    transport: HttpTransport,
}

impl WebhooksResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `POST /webhooks/stripe`.
    pub async fn stripe(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/webhooks/stripe", body).await
    }
    // </generated-openapi>

}
