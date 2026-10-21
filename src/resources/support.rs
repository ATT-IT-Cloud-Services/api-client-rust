use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{
    CreateTicketRequest, CreateTicketResponse, KnowledgeBaseResponse, ReplyTicketRequest,
    SuccessResponse, TicketDetailResponse, TicketListResponse,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct SupportResource {
    transport: HttpTransport,
}

impl SupportResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn tickets(
        &self,
        limit: u64,
        status: Option<&str>,
    ) -> Result<TicketListResponse, IgniteError> {
        let mut pairs = vec![("limit", json!(limit))];
        if let Some(s) = status {
            pairs.push(("status", json!(s)));
        }
        let query = query_params(&pairs);
        self.transport.get("/tickets", Some(query)).await
    }

    pub async fn ticket(&self, id: &str) -> Result<TicketDetailResponse, IgniteError> {
        self.transport.get(&format!("/tickets/{id}"), None).await
    }

    pub async fn create_ticket(
        &self,
        body: CreateTicketRequest,
    ) -> Result<CreateTicketResponse, IgniteError> {
        self.transport
            .post("/tickets", Some(serde_json::to_value(body)?))
            .await
    }

    pub async fn reply(
        &self,
        ticket_id: &str,
        content: &str,
    ) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post(
                &format!("/tickets/{ticket_id}/messages"),
                Some(serde_json::to_value(ReplyTicketRequest {
                    content: content.to_string(),
                })?),
            )
            .await
    }

    pub async fn knowledge_base(
        &self,
        lang: Option<&str>,
    ) -> Result<KnowledgeBaseResponse, IgniteError> {
        let query = lang.map(|l| query_params(&[("lang", json!(l))]));
        self.transport.get("/knowledge-base", query).await
    }

    // <generated-openapi>
    /// Generated `GET /feedback`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/feedback", query).await
    }

    /// Generated `POST /feedback`.
    pub async fn create(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/feedback", body).await
    }

    /// Generated `PATCH /tickets/[id]`.
    pub async fn update(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/tickets/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /tickets/[id]`.
    pub async fn post_create(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/tickets/{}", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /tickets/[id]/export`.
    pub async fn export(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/tickets/{}/export", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /tickets/attachable-services`.
    pub async fn attachable_services(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/tickets/attachable-services", query).await
    }

    /// Generated `GET /tickets/share-candidates`.
    pub async fn share_candidates(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/tickets/share-candidates", query).await
    }
    // </generated-openapi>
}
