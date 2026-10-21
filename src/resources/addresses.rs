use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct AddressesResource {
    transport: HttpTransport,
}

impl AddressesResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `DELETE /addresses`.
    pub async fn remove(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/addresses", query, body).await
    }

    /// Generated `GET /addresses`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/addresses", query).await
    }

    /// Generated `POST /addresses`.
    pub async fn create(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/addresses", body).await
    }

    /// Generated `PUT /addresses`.
    pub async fn replace(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.put("/addresses", body).await
    }
    // </generated-openapi>

}
