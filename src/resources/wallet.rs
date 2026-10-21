use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{
    AutoTopupCreateBody, AutoTopupCreateResponse, AutoTopupListResponse, AutoTopupUpdateBody,
    AutoTopupUpdateResponse, RedeemVoucherRequest, RedeemVoucherResponse, SuccessResponse,
    VoucherListResponse,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct WalletResource {
    transport: HttpTransport,
}

impl WalletResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn bonus_event(&self) -> Result<SuccessResponse, IgniteError> {
        self.transport.get("/wallet/bonus-event", None).await
    }

    pub async fn auto_topup_list(&self) -> Result<AutoTopupListResponse, IgniteError> {
        self.transport.get("/balance/auto-topup", None).await
    }

    pub async fn auto_topup_create(
        &self,
        body: AutoTopupCreateBody,
    ) -> Result<AutoTopupCreateResponse, IgniteError> {
        self.transport
            .post("/balance/auto-topup", Some(serde_json::to_value(body)?))
            .await
    }

    pub async fn auto_topup_update(
        &self,
        id: &str,
        body: AutoTopupUpdateBody,
    ) -> Result<AutoTopupUpdateResponse, IgniteError> {
        let path = format!("/balance/auto-topup/{}", encode(id));
        self.transport
            .patch(&path, Some(serde_json::to_value(body)?))
            .await
    }

    pub async fn auto_topup_delete(&self, id: &str) -> Result<SuccessResponse, IgniteError> {
        let path = format!("/balance/auto-topup/{}", encode(id));
        self.transport.delete(&path, None, None).await
    }

    pub async fn vouchers(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<VoucherListResponse, IgniteError> {
        let query = query_params(&[("limit", json!(limit)), ("offset", json!(offset))]);
        self.transport.get("/wallet/vouchers", Some(query)).await
    }

    pub async fn redeem_voucher(&self, code: &str) -> Result<RedeemVoucherResponse, IgniteError> {
        self.transport
            .post(
                "/wallet/vouchers/redeem",
                Some(serde_json::to_value(RedeemVoucherRequest {
                    code: code.to_string(),
                })?),
            )
            .await
    }

    // <generated-openapi>
    /// Generated `GET /wallet/payout-request`.
    pub async fn payout_request(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/wallet/payout-request", query).await
    }

    /// Generated `POST /wallet/payout-request`.
    pub async fn post_payout_request(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/wallet/payout-request", body).await
    }

    /// Generated `GET /wallet/vouchers/[id]/certificate.pdf`.
    pub async fn vouchers_certificate_pdf(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/wallet/vouchers/{}/certificate.pdf", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /wallet/vouchers/[id]/code`.
    pub async fn vouchers_code(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/wallet/vouchers/{}/code", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /wallet/vouchers/[id]/withdraw`.
    pub async fn vouchers_withdraw(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/wallet/vouchers/{}/withdraw", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /wallet/vouchers/purchase-from-balance`.
    pub async fn vouchers_purchase_from_balance(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/wallet/vouchers/purchase-from-balance", body).await
    }

    /// Generated `GET /wallet/vouchers/purchase-status`.
    pub async fn vouchers_purchase_status(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/wallet/vouchers/purchase-status", query).await
    }

    /// Generated `POST /wallet/vouchers/purchase-with-mandate`.
    pub async fn vouchers_purchase_with_mandate(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/wallet/vouchers/purchase-with-mandate", body).await
    }

    /// Generated `POST /wallet/vouchers/purchase-with-mollie`.
    pub async fn vouchers_purchase_with_mollie(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/wallet/vouchers/purchase-with-mollie", body).await
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
