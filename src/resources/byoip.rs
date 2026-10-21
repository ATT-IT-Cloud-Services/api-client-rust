use crate::error::IgniteError;
use crate::models::{ByoipAssignRequest, ByoipCancelRequest, ByoipStatusResponse, SuccessResponse};
use crate::transport::HttpTransport;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ByoipResource {
    transport: HttpTransport,
}

impl ByoipResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn status(&self) -> Result<ByoipStatusResponse, IgniteError> {
        self.transport.get("/byoip/status", None).await
    }

    pub async fn assign(&self, body: ByoipAssignRequest) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post("/byoip/assign", Some(serde_json::to_value(body)?))
            .await
    }

    pub async fn unassign(&self, assignment_id: &str) -> Result<SuccessResponse, IgniteError> {
        let path = format!("/byoip/assign/{}", encode(assignment_id));
        self.transport.delete(&path, None, None).await
    }

    pub async fn cancel(&self, network_id: &str) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post(
                "/byoip/cancel",
                Some(serde_json::to_value(ByoipCancelRequest {
                    network_id: network_id.to_string(),
                })?),
            )
            .await
    }

    // <generated-openapi>
    /// Generated `POST /byoip/apply`.
    pub async fn apply(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/byoip/apply", body).await
    }

    /// Generated `GET /byoip/assign`.
    pub async fn get_assign(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/byoip/assign", query).await
    }

    /// Generated `POST /byoip/deannouncement/request`.
    pub async fn deannouncement_request(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/byoip/deannouncement/request", body).await
    }

    /// Generated `POST /byoip/migration/request`.
    pub async fn migration_request(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/byoip/migration/request", body).await
    }

    /// Generated `POST /byoip/subscribe`.
    pub async fn subscribe(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/byoip/subscribe", body).await
    }

    /// Generated `GET /byoip/transfers`.
    pub async fn transfers(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/byoip/transfers", query).await
    }

    /// Generated `POST /byoip/transfers`.
    pub async fn post_transfers(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/byoip/transfers", body).await
    }

    /// Generated `GET /byoip/wallet`.
    pub async fn wallet(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/byoip/wallet", query).await
    }

    /// Generated `POST /byoip/wallet`.
    pub async fn post_wallet(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/byoip/wallet", body).await
    }

    /// Generated `PUT /byoip/wallet`.
    pub async fn replace_wallet(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.put("/byoip/wallet", body).await
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
