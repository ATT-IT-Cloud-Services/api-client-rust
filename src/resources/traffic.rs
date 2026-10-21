use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct TrafficResource {
    transport: HttpTransport,
}

impl TrafficResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `POST /traffic/pricing-preview`.
    pub async fn pricing_preview(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/traffic/pricing-preview", body).await
    }
    // </generated-openapi>

}
