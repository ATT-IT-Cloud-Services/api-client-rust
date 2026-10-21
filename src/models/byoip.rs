use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByoipAssignmentService {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByoipAssignment {
    pub id: String,
    pub assigned_prefix: String,
    pub is_primary: bool,
    pub service: Option<ByoipAssignmentService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByoipNetwork {
    pub id: String,
    pub prefix: String,
    pub prefix_version: u64,
    pub asn_number: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub location_id: Option<String>,
    pub setup_fee_gross: f64,
    pub monthly_fee_gross: f64,
    pub current_period_end: Option<String>,
    pub cancel_at_period_end: bool,
    pub ddos_protection_provider: String,
    pub tb_wallet_balance_tb: f64,
    pub tb_wallet_used_tb: f64,
    pub created_at: String,
    pub assignments: Vec<ByoipAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByoipStatusResponse {
    pub success: bool,
    pub networks: Vec<ByoipNetwork>,
    pub allowed_location_ids: Vec<String>,
    pub wallet_price_per_tb_gross: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByoipAssignRequest {
    pub network_id: String,
    pub service_id: String,
    pub assigned_prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByoipCancelRequest {
    pub network_id: String,
}
