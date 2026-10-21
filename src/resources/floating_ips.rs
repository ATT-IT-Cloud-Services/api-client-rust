use serde_json::{json, Value};
use std::collections::HashMap;

use crate::error::IgniteError;
use crate::models::{
    FloatingIpAssignRequest, FloatingIpListResponse, FloatingIpOptionsResponse,
    FloatingIpOrderRequest, FloatingIpOrderResponse, SuccessResponse,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct FloatingIpsResource {
    transport: HttpTransport,
}

impl FloatingIpsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<FloatingIpListResponse, IgniteError> {
        self.transport.get("/floating-ips", None).await
    }

    pub async fn options(&self) -> Result<FloatingIpOptionsResponse, IgniteError> {
        self.transport.get("/floating-ips/options", None).await
    }

    pub async fn order(
        &self,
        body: FloatingIpOrderRequest,
    ) -> Result<FloatingIpOrderResponse, IgniteError> {
        self.transport
            .post("/floating-ips/order", Some(serde_json::to_value(body)?))
            .await
    }

    pub async fn assign(&self, id: &str, service_id: &str) -> Result<SuccessResponse, IgniteError> {
        let path = format!("/floating-ips/{}/assign", encode(id));
        self.transport
            .post(
                &path,
                Some(serde_json::to_value(FloatingIpAssignRequest {
                    service_id: service_id.to_string(),
                })?),
            )
            .await
    }

    pub async fn unassign(&self, id: &str) -> Result<SuccessResponse, IgniteError> {
        let path = format!("/floating-ips/{}/unassign", encode(id));
        self.transport.post(&path, Some(json!({}))).await
    }

    pub async fn cancel(&self, id: &str) -> Result<SuccessResponse, IgniteError> {
        let path = format!("/floating-ips/{}/cancel", encode(id));
        self.transport.post(&path, Some(json!({}))).await
    }

    // <generated-openapi>
    /// Generated `POST /floating-ips/[id]/cancel`.
    pub async fn post_cancel(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/floating-ips/{}/cancel", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /floating-ips/[id]/targets`.
    pub async fn targets(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/floating-ips/{}/targets", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /floating-ips/[id]/unassign`.
    pub async fn post_unassign(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/floating-ips/{}/unassign", encode_path(id));
        self.transport.post(&path, body).await
    }
    // </generated-openapi>
}

fn encode(value: &str) -> String {
    value
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}
