use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct NewsletterResource {
    transport: HttpTransport,
}

impl NewsletterResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /newsletter/unsubscribe`.
    pub async fn unsubscribe(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/newsletter/unsubscribe", query).await
    }

    /// Generated `GET /newsletter/verify`.
    pub async fn verify(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/newsletter/verify", query).await
    }
    // </generated-openapi>

}
