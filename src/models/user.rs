use serde::{Deserialize, Serialize};

use super::common::ExtraFields;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogEntry {
    pub id: String,
    pub action: String,
    pub created_at: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportPinResponse {
    pub support_pin: Option<String>,
    pub pin_expires_at: Option<String>,
    pub pin_time_left: Option<f64>,
    pub pin_cooldown: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogResponse {
    pub success: bool,
    pub logs: Vec<AuditLogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GdprExportCheckResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GdprExportConfirmRequest {
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeysListResponse {
    pub success: bool,
    pub api_keys: Vec<ApiKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyResponse {
    pub success: bool,
    pub api_key: ApiKey,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpRequestingIp {
    pub ip: String,
    pub first_seen_at: String,
    pub last_seen_at: String,
    pub hit_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpCredential {
    pub id: String,
    pub name: String,
    pub token_prefix: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub request_count: u64,
    #[serde(default)]
    pub requesting_ips: Vec<McpRequestingIp>,
    pub ip_count: u64,
    /// Only present in the create response — the token is shown once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpCredentialsListResponse {
    pub success: bool,
    pub mcp_credentials: Vec<McpCredential>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mcp_credentials: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMcpCredentialRequest {
    pub name: String,
    pub scopes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMcpCredentialResponse {
    pub success: bool,
    pub mcp_credential: McpCredential,
}
