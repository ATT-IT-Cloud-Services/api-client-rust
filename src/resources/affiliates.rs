use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{
    AffiliateCommissionsResponse, AffiliateMeResponse, AffiliateTaxInfoResponse,
    PayoutRequestsResponse,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct AffiliatesResource {
    transport: HttpTransport,
}

impl AffiliatesResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn me(&self) -> Result<AffiliateMeResponse, IgniteError> {
        self.transport.get("/affiliates/me", None).await
    }

    pub async fn commissions(
        &self,
        limit: u64,
    ) -> Result<AffiliateCommissionsResponse, IgniteError> {
        let query = query_params(&[("limit", json!(limit))]);
        self.transport
            .get("/affiliates/me/commissions", Some(query))
            .await
    }

    pub async fn payout_requests(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<PayoutRequestsResponse, IgniteError> {
        let query = query_params(&[("limit", json!(limit)), ("offset", json!(offset))]);
        self.transport
            .get("/affiliates/me/payout-requests", Some(query))
            .await
    }

    pub async fn tax_info(&self) -> Result<AffiliateTaxInfoResponse, IgniteError> {
        self.transport.get("/affiliates/me/tax-info", None).await
    }

    // <generated-openapi>
    /// Generated `POST /affiliates/me/convert-to-credit`.
    pub async fn me_convert_to_credit(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/affiliates/me/convert-to-credit", body).await
    }

    /// Generated `GET /affiliates/me/coupon-usage`.
    pub async fn me_coupon_usage(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/affiliates/me/coupon-usage", query).await
    }

    /// Generated `POST /affiliates/me/credit-note/[id]/confirm`.
    pub async fn me_credit_note_confirm(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/affiliates/me/credit-note/{}/confirm", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /affiliates/me/credit-note/preview`.
    pub async fn me_credit_note_preview(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/affiliates/me/credit-note/preview", body).await
    }

    /// Generated `GET /affiliates/me/credit-notes`.
    pub async fn me_credit_notes(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/affiliates/me/credit-notes", query).await
    }

    /// Generated `GET /affiliates/me/credit-notes/[id]/download`.
    pub async fn me_credit_notes_download(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/affiliates/me/credit-notes/{}/download", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /affiliates/me/payout-method`.
    pub async fn update_me_payout_method(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.patch("/affiliates/me/payout-method", body).await
    }

    /// Generated `POST /affiliates/me/payout-requests`.
    pub async fn me_payout_requests(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/affiliates/me/payout-requests", body).await
    }

    /// Generated `PUT /affiliates/me/tax-info`.
    pub async fn replace_me_tax_info(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.put("/affiliates/me/tax-info", body).await
    }

    /// Generated `POST /affiliates/validate`.
    pub async fn validate(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/affiliates/validate", body).await
    }
    // </generated-openapi>
}
