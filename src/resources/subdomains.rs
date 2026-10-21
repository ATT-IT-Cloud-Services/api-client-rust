use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::models::{AllowedDomainsResponse, SubdomainListResponse, SuccessResponse};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct SubdomainsResource {
    transport: HttpTransport,
}

impl SubdomainsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<SubdomainListResponse, IgniteError> {
        self.transport.get("/subdomains", None).await
    }

    pub async fn allowed_domains(&self) -> Result<AllowedDomainsResponse, IgniteError> {
        self.transport.get("/subdomains/domains", None).await
    }

    pub async fn create(
        &self,
        body: HashMap<String, Value>,
    ) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post(
                "/subdomains",
                Some(Value::Object(body.into_iter().collect())),
            )
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .delete(&format!("/subdomains/{id}"), None, None)
            .await
    }

    // <generated-openapi>
    /// Generated `PATCH /subdomains/[id]`.
    pub async fn update(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/subdomains/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }
    // </generated-openapi>
}
