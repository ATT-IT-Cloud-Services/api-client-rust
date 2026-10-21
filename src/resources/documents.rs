use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct DocumentsResource {
    transport: HttpTransport,
}

impl DocumentsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /documents`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/documents", query).await
    }

    /// Generated `GET /documents/[id]`.
    pub async fn find(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/documents/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /documents/[id]/pdf`.
    pub async fn pdf(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/documents/{}/pdf", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /documents/[id]/respond`.
    pub async fn respond(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/documents/{}/respond", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /documents/contracts`.
    pub async fn contracts(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/documents/contracts", query).await
    }

    /// Generated `GET /documents/contracts/[id]/versions/[version]/files/[filename]`.
    pub async fn contracts_versions_files(
        &self,
        id: &str,
        version: &str,
        filename: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/documents/contracts/{}/versions/{}/files/{}", encode_path(id), encode_path(version), encode_path(filename));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /documents/contracts/[id]/versions/[version]/respond`.
    pub async fn contracts_versions_respond(
        &self,
        id: &str,
        version: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/documents/contracts/{}/versions/{}/respond", encode_path(id), encode_path(version));
        self.transport.post(&path, body).await
    }
    // </generated-openapi>

}
