use serde::{Deserialize, Serialize};

use super::common::ExtraFields;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyGroup {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMember {
    pub id: String,
    pub user_id: String,
    pub role: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyInvite {
    pub id: String,
    pub email: String,
    pub role: String,
    pub status: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyGroupResponse {
    pub success: bool,
    pub group: Option<FamilyGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMembersResponse {
    pub success: bool,
    pub members: Vec<FamilyMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyInvitesResponse {
    pub success: bool,
    pub invites: Vec<FamilyInvite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyBalanceRequest {
    pub id: String,
    pub amount: f64,
    pub status: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyBalanceRequestsResponse {
    pub success: bool,
    pub requests: Vec<FamilyBalanceRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFamilyGroupRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFamilyGroupResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyInviteRequest {
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyInviteResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFamilyMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_billing_mode: Option<String>,
}
