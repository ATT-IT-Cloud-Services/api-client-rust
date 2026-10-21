use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct ByoasResource {
    transport: HttpTransport,
}

impl ByoasResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `POST /byoas/[id]/legal`.
    pub async fn legal(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/byoas/{}/legal", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /byoas/[id]/networks`.
    pub async fn networks(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/byoas/{}/networks", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /byoas/[id]/networks`.
    pub async fn post_networks(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/byoas/{}/networks", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /byoas/apply`.
    pub async fn apply(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/byoas/apply", body).await
    }

    /// Generated `GET /byoas/status`.
    pub async fn status(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/byoas/status", query).await
    }
    // </generated-openapi>

}
