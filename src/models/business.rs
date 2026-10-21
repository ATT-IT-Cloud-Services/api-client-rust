use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessFundContract {
    pub id: String,
    pub commitment_amount: f64,
    pub price: f64,
    pub overcharge_fee_percent: f64,
    pub binding_months: u64,
    pub binding_end_date: Option<String>,
    pub status: String,
    pub current_usage: f64,
    pub current_overcharge: f64,
    pub last_reset_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessFundOffer {
    pub id: String,
    pub commitment_amount: f64,
    pub price: f64,
    pub overcharge_fee_percent: f64,
    pub binding_months: u64,
    pub admin_note: Option<String>,
    pub offered_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessFundStatusResponse {
    pub success: bool,
    pub contract: Option<BusinessFundContract>,
    pub pending_offer: Option<BusinessFundOffer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessBillingResponse {
    pub success: bool,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestFundRequest {
    pub amount: f64,
    pub binding_months: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestFundResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BusinessFundPaymentMethod {
    Mollie,
    Guthaben,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptFundRequest {
    pub payment_method: BusinessFundPaymentMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptFundResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
}
