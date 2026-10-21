use serde::{Deserialize, Serialize};

use super::common::ExtraFields;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateProfile {
    pub id: String,
    pub code: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateCommission {
    pub id: String,
    pub amount: f64,
    pub status: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateMeResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<AffiliateProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateCommissionsResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions: Option<Vec<AffiliateCommission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutRequest {
    pub id: String,
    pub amount: f64,
    pub status: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutRequestsResponse {
    pub success: bool,
    pub requests: Vec<PayoutRequest>,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateTaxInfoResponse {
    pub success: bool,
    pub tax_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<serde_json::Value>,
}
