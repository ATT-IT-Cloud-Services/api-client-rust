use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{ExtraFields, Pagination};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: String,
    pub subject: String,
    pub status: String,
    pub priority: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketListResponse {
    pub success: bool,
    pub tickets: Vec<Ticket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketDetailResponse {
    pub success: bool,
    pub ticket: Ticket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeBaseArticle {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeBaseResponse {
    pub success: bool,
    pub articles: Vec<KnowledgeBaseArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketRequest {
    pub subject: String,
    pub message: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTicketResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyTicketRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardTicketsResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalBandwidthUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_gb: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalBandwidthResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TotalBandwidthUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceNotificationsResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<HashMap<String, Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSourceCounts {
    #[serde(rename = "SERVICE")]
    pub service: u64,
    #[serde(rename = "POOL")]
    pub pool: u64,
    #[serde(rename = "WALLET")]
    pub wallet: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficSourcesSummary {
    pub wallet_balance_tb: f64,
    pub wallet_used_tb: f64,
    pub service_source_counts: ServiceSourceCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficSourcesResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<TrafficSourcesSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_sources: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyOffersResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<Value>>,
}
