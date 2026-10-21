use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::ExtraFields;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderViewIdentity {
    pub name: String,
    pub display_name: Option<String>,
    pub status: Option<String>,
    pub primary_ipv4: Option<String>,
    pub primary_ipv6: Option<String>,
    pub product_name: Option<String>,
    pub node: Option<String>,
    pub region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderViewResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabs: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview_fields: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widgets: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ProviderViewIdentity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDispatchResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceListPagination {
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceListResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Service>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<ServiceListPagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceDetailResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Service>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusBatchRequest {
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusBatchResponse {
    pub success: bool,
    pub statuses: HashMap<String, HashMap<String, Value>>,
}
