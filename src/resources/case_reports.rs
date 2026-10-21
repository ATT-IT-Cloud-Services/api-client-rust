use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct CaseReportsResource {
    transport: HttpTransport,
}

impl CaseReportsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `POST /case-reports/damage`.
    pub async fn damage(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/case-reports/damage", body).await
    }

    /// Generated `GET /case-reports/damage/bootstrap`.
    pub async fn damage_bootstrap(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/case-reports/damage/bootstrap", query).await
    }
    // </generated-openapi>

}
