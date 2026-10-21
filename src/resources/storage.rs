use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct StorageResource {
    transport: HttpTransport,
}

impl StorageResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /storage`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/storage", query).await
    }

    /// Generated `POST /storage`.
    pub async fn create(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/storage", body).await
    }

    /// Generated `DELETE /storage/[id]`.
    pub async fn remove(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/storage/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /storage/[id]`.
    pub async fn find(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/storage/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /storage/[id]`.
    pub async fn update(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/storage/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /storage/[id]/cancel`.
    pub async fn cancel(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/storage/{}/cancel", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /storage/add-to-cart`.
    pub async fn add_to_cart(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/storage/add-to-cart", body).await
    }

    /// Generated `GET /storage/config`.
    pub async fn config(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/storage/config", query).await
    }
    // </generated-openapi>

}
