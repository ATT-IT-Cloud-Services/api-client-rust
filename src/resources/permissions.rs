use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct PermissionsResource {
    transport: HttpTransport,
}

impl PermissionsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `DELETE /permissions`.
    pub async fn remove(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/permissions", query, body).await
    }

    /// Generated `GET /permissions`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/permissions", query).await
    }

    /// Generated `POST /permissions`.
    pub async fn create(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/permissions", body).await
    }

    /// Generated `DELETE /permissions/[id]`.
    pub async fn delete_remove(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/permissions/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /permissions/[id]`.
    pub async fn update(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/permissions/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /permissions/[id]/details`.
    pub async fn details(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/permissions/{}/details", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /permissions/accept`.
    pub async fn accept(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/permissions/accept", body).await
    }

    /// Generated `POST /permissions/decline`.
    pub async fn decline(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/permissions/decline", body).await
    }

    /// Generated `GET /permissions/me`.
    pub async fn me(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/permissions/me", query).await
    }
    // </generated-openapi>

}
