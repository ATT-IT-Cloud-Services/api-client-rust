use crate::error::IgniteError;
use crate::models::{
    CreateSshKeyRequest, GenerateSshKeyRequest, SshKeyGenerateResponse, SshKeyListResponse,
    SuccessResponse,
};
use crate::transport::{encode_path, HttpTransport};
use serde_json::Value;

#[derive(Clone)]
pub struct SshKeysResource {
    transport: HttpTransport,
}

impl SshKeysResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<SshKeyListResponse, IgniteError> {
        self.transport.get("/ssh-keys", None).await
    }

    pub async fn create(&self, body: CreateSshKeyRequest) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post("/ssh-keys", Some(serde_json::to_value(body)?))
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .delete(&format!("/ssh-keys/{id}"), None, None)
            .await
    }

    pub async fn generate(
        &self,
        body: GenerateSshKeyRequest,
    ) -> Result<SshKeyGenerateResponse, IgniteError> {
        self.transport
            .post("/ssh-keys/generate", Some(serde_json::to_value(body)?))
            .await
    }

    // <generated-openapi>
    /// Generated `PATCH /ssh-keys/[id]`.
    pub async fn update(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/ssh-keys/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }
    // </generated-openapi>
}
