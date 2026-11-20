use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct OrdersResource {
    transport: HttpTransport,
}

impl OrdersResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /orders`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/orders", query).await
    }

    /// Generated `GET /orders/[id]`.
    pub async fn find(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/orders/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /orders/[id]/choice`.
    pub async fn choice(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/orders/{}/choice", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /orders/[id]/credit-notes/[creditNoteId]/document`.
    pub async fn credit_notes_document(
        &self,
        id: &str,
        credit_note_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/orders/{}/credit-notes/{}/document", encode_path(id), encode_path(credit_note_id));
        self.transport.get(&path, query).await
    }
    // </generated-openapi>

}
