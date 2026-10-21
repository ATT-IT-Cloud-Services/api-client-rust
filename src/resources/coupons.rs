use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct CouponsResource {
    transport: HttpTransport,
}

impl CouponsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `POST /coupons/preview`.
    pub async fn preview(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/coupons/preview", body).await
    }
    // </generated-openapi>

}
