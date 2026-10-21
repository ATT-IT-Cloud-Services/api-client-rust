use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatingIp {
    pub id: String,
    pub address: String,
    pub ip_version: u64,
    pub status: String,
    pub location_id: String,
    pub location_name: String,
    pub assigned_service_id: Option<String>,
    pub assigned_service_name: Option<String>,
    pub monthly_price: f64,
    pub next_billing_at: Option<String>,
    pub subscription_status: Option<String>,
    pub cancel_at_period_end: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatingIpVersionOffer {
    pub monthly_price: f64,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatingIpLocationOffer {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<FloatingIpVersionOffer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<FloatingIpVersionOffer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatingIpListResponse {
    pub success: bool,
    pub floating_ips: Vec<FloatingIp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatingIpOptionsResponse {
    pub success: bool,
    pub locations: Vec<FloatingIpLocationOffer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatingIpOrderRequest {
    pub location_id: String,
    pub ip_version: u64,
    pub billing_cycle: u64,
    pub payment_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatingIpOrderResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floating_ip: Option<FloatingIp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatingIpAssignRequest {
    pub service_id: String,
}
