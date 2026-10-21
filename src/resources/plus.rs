use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct PlusResource {
    transport: HttpTransport,
}

impl PlusResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `POST /plus/cancel`.
    pub async fn cancel(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/plus/cancel", body).await
    }

    /// Generated `GET /plus/forecast`.
    pub async fn forecast(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/plus/forecast", query).await
    }

    /// Generated `GET /plus/invoices`.
    pub async fn invoices(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/plus/invoices", query).await
    }

    /// Generated `GET /plus/pooling`.
    pub async fn pooling(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/plus/pooling", query).await
    }

    /// Generated `PUT /plus/pooling`.
    pub async fn replace_pooling(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.put("/plus/pooling", body).await
    }

    /// Generated `POST /plus/pricing-preview`.
    pub async fn pricing_preview(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/plus/pricing-preview", body).await
    }

    /// Generated `POST /plus/reactivate`.
    pub async fn reactivate(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/plus/reactivate", body).await
    }

    /// Generated `POST /plus/security-plus`.
    pub async fn security_plus(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/plus/security-plus", body).await
    }

    /// Generated `GET /plus/status`.
    pub async fn status(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/plus/status", query).await
    }

    /// Generated `POST /plus/subscribe`.
    pub async fn subscribe(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/plus/subscribe", body).await
    }

    /// Generated `GET /plus/wallet`.
    pub async fn wallet(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/plus/wallet", query).await
    }

    /// Generated `POST /plus/wallet`.
    pub async fn post_wallet(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/plus/wallet", body).await
    }
    // </generated-openapi>

}
