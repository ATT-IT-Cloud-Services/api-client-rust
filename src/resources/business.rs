use serde_json::{json, Value};
use std::collections::HashMap;

use crate::error::IgniteError;
use crate::models::{
    AcceptFundRequest, AcceptFundResponse, BusinessBillingResponse, BusinessFundPaymentMethod,
    BusinessFundStatusResponse, RequestFundRequest, RequestFundResponse, SuccessResponse,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct BusinessResource {
    transport: HttpTransport,
}

impl BusinessResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn fund(&self) -> Result<BusinessFundStatusResponse, IgniteError> {
        self.transport.get("/business/fund", None).await
    }

    pub async fn request_fund(
        &self,
        amount: f64,
        binding_months: u64,
    ) -> Result<RequestFundResponse, IgniteError> {
        self.transport
            .post(
                "/business/fund",
                Some(serde_json::to_value(RequestFundRequest {
                    amount,
                    binding_months,
                })?),
            )
            .await
    }

    pub async fn accept_fund(
        &self,
        offer_id: &str,
        payment_method: BusinessFundPaymentMethod,
    ) -> Result<AcceptFundResponse, IgniteError> {
        let path = format!("/business/fund/{}/accept", encode(offer_id));
        self.transport
            .post(
                &path,
                Some(serde_json::to_value(AcceptFundRequest { payment_method })?),
            )
            .await
    }

    pub async fn cancel_fund(&self, contract_id: &str) -> Result<SuccessResponse, IgniteError> {
        let path = format!("/business/fund/{}/cancel", encode(contract_id));
        self.transport.post(&path, Some(json!({}))).await
    }

    pub async fn billing(&self) -> Result<BusinessBillingResponse, IgniteError> {
        self.transport.get("/business/billing", None).await
    }

    // <generated-openapi>
    /// Generated `DELETE /business/billing`.
    pub async fn delete_billing(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/business/billing", query, body).await
    }

    /// Generated `POST /business/billing`.
    pub async fn post_billing(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/business/billing", body).await
    }

    /// Generated `GET /business/eligibility`.
    pub async fn eligibility(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/business/eligibility", query).await
    }

    /// Generated `POST /business/fund/[id]/cancel`.
    pub async fn fund_cancel(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/business/fund/{}/cancel", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /business/fund/[id]/reject`.
    pub async fn fund_reject(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/business/fund/{}/reject", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /business/fund/special-termination`.
    pub async fn fund_special_termination(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/business/fund/special-termination", query).await
    }

    /// Generated `POST /business/fund/special-termination`.
    pub async fn post_fund_special_termination(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/business/fund/special-termination", body).await
    }

    /// Generated `GET /business/verification`.
    pub async fn verification(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/business/verification", query).await
    }

    /// Generated `POST /business/verification`.
    pub async fn post_verification(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/business/verification", body).await
    }

    /// Generated `GET /business/verification/documents/[id]`.
    pub async fn verification_documents(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/business/verification/documents/{}", encode_path(id));
        self.transport.get(&path, query).await
    }
    // </generated-openapi>
}

fn encode(value: &str) -> String {
    value
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}
