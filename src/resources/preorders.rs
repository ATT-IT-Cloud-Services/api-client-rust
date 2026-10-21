use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct PreordersResource {
    transport: HttpTransport,
}

impl PreordersResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /preorders`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/preorders", query).await
    }

    /// Generated `POST /preorders/[id]/withdraw`.
    pub async fn withdraw(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/preorders/{}/withdraw", encode_path(id));
        self.transport.post(&path, body).await
    }
    // </generated-openapi>

}
