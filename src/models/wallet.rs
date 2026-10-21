use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoTopupConfig {
    pub id: String,
    pub is_active: bool,
    pub amount: f64,
    pub day_of_month: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_charged_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_charge_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_verified_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_redeem_voucher: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoTopupCreateBody {
    pub amount: f64,
    pub day_of_month: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_redeem_voucher: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_terms: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_privacy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_expiry: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletVoucher {
    pub id: String,
    pub amount: f64,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoTopupListResponse {
    pub success: bool,
    pub configs: Vec<AutoTopupConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherListResponse {
    pub success: bool,
    pub vouchers: Vec<WalletVoucher>,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoTopupUpdateBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoTopupCreateResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<AutoTopupConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_checkout_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred_first_charge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_charge_on: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoTopupUpdateResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<AutoTopupConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedeemVoucherRequest {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemVoucherResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}
