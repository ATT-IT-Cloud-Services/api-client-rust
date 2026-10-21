use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::models::{AddIpv6Request, IpManagerListResponse, SuccessResponse};
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct IpManagerResource {
    transport: HttpTransport,
}

impl IpManagerResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<IpManagerListResponse, IgniteError> {
        self.transport.get("/ip-manager", None).await
    }

    pub async fn update(
        &self,
        id: &str,
        body: HashMap<String, Value>,
    ) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .patch(
                &format!("/ip-manager/{id}"),
                Some(Value::Object(body.into_iter().collect())),
            )
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .delete(&format!("/ip-manager/{id}"), None, None)
            .await
    }

    pub async fn add_ipv6(&self, body: AddIpv6Request) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post("/ip-manager", Some(serde_json::to_value(body)?))
            .await
    }
}
