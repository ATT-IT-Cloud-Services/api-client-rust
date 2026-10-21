use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{
    CreateFamilyGroupRequest, CreateFamilyGroupResponse, FamilyBalanceRequestsResponse,
    FamilyGroupResponse, FamilyInviteRequest, FamilyInviteResponse, FamilyInvitesResponse,
    FamilyMembersResponse, SuccessResponse, UpdateFamilyMemberRequest,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct FamilyResource {
    transport: HttpTransport,
}

impl FamilyResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn group(&self) -> Result<FamilyGroupResponse, IgniteError> {
        self.transport.get("/family", None).await
    }

    pub async fn create(&self, name: &str) -> Result<CreateFamilyGroupResponse, IgniteError> {
        self.transport
            .post(
                "/family",
                Some(serde_json::to_value(CreateFamilyGroupRequest {
                    name: name.to_string(),
                })?),
            )
            .await
    }

    pub async fn invites(&self, group_id: &str) -> Result<FamilyInvitesResponse, IgniteError> {
        self.transport
            .get(&format!("/family/{group_id}/invites"), None)
            .await
    }

    pub async fn members(&self, group_id: &str) -> Result<FamilyMembersResponse, IgniteError> {
        self.transport
            .get(&format!("/family/{group_id}/members"), None)
            .await
    }

    pub async fn invite(
        &self,
        group_id: &str,
        email: &str,
        role: &str,
    ) -> Result<FamilyInviteResponse, IgniteError> {
        self.transport
            .post(
                &format!("/family/{group_id}/invites"),
                Some(serde_json::to_value(FamilyInviteRequest {
                    email: email.to_string(),
                    role: role.to_string(),
                })?),
            )
            .await
    }

    pub async fn balance_requests(
        &self,
        group_id: &str,
        status: Option<&str>,
    ) -> Result<FamilyBalanceRequestsResponse, IgniteError> {
        let query = status.map(|s| query_params(&[("status", json!(s))]));
        self.transport
            .get(&format!("/family/{group_id}/balance-requests"), query)
            .await
    }

    pub async fn remove_member(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<SuccessResponse, IgniteError> {
        let path = format!("/family/{group_id}/members/{}", encode(user_id));
        self.transport.delete(&path, None, None).await
    }

    pub async fn update_member(
        &self,
        group_id: &str,
        user_id: &str,
        body: UpdateFamilyMemberRequest,
    ) -> Result<SuccessResponse, IgniteError> {
        let path = format!("/family/{group_id}/members/{}", encode(user_id));
        self.transport
            .patch(&path, Some(serde_json::to_value(body)?))
            .await
    }

    // <generated-openapi>
    /// Generated `DELETE /family/[groupId]`.
    pub async fn remove(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}", encode_path(group_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /family/[groupId]`.
    pub async fn find(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /family/[groupId]`.
    pub async fn update(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}", encode_path(group_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /family/[groupId]/audit`.
    pub async fn audit(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/audit", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /family/[groupId]/auto-topup`.
    pub async fn auto_topup(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/auto-topup", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /family/[groupId]/auto-topup`.
    pub async fn post_auto_topup(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/auto-topup", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /family/[groupId]/balance-requests`.
    pub async fn post_balance_requests(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/balance-requests", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `PATCH /family/[groupId]/balance-requests/[requestId]`.
    pub async fn update_balance_requests(
        &self,
        group_id: &str,
        request_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/balance-requests/{}", encode_path(group_id), encode_path(request_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /family/[groupId]/billing`.
    pub async fn billing(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/billing", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /family/[groupId]/billing`.
    pub async fn update_billing(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/billing", encode_path(group_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /family/[groupId]/billing`.
    pub async fn post_billing(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/billing", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /family/[groupId]/billing/mandates`.
    pub async fn billing_mandates(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/billing/mandates", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /family/[groupId]/billing/mandates`.
    pub async fn update_billing_mandates(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/billing/mandates", encode_path(group_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /family/[groupId]/billing/mandates`.
    pub async fn post_billing_mandates(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/billing/mandates", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /family/[groupId]/billing/mandates/[mandateId]`.
    pub async fn delete_billing_mandates(
        &self,
        group_id: &str,
        mandate_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/billing/mandates/{}", encode_path(group_id), encode_path(mandate_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /family/[groupId]/consent`.
    pub async fn consent(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/consent", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /family/[groupId]/consent`.
    pub async fn update_consent(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/consent", encode_path(group_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /family/[groupId]/consent`.
    pub async fn post_consent(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/consent", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /family/[groupId]/controls/locks`.
    pub async fn controls_locks(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/locks", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /family/[groupId]/controls/locks`.
    pub async fn post_controls_locks(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/locks", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /family/[groupId]/controls/locks/[lockId]`.
    pub async fn delete_controls_locks(
        &self,
        group_id: &str,
        lock_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/locks/{}", encode_path(group_id), encode_path(lock_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /family/[groupId]/controls/resource-limits`.
    pub async fn controls_resource_limits(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/resource-limits", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /family/[groupId]/controls/resource-limits`.
    pub async fn post_controls_resource_limits(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/resource-limits", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /family/[groupId]/controls/resource-limits/[limitId]`.
    pub async fn delete_controls_resource_limits(
        &self,
        group_id: &str,
        limit_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/resource-limits/{}", encode_path(group_id), encode_path(limit_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /family/[groupId]/controls/resource-limits/[limitId]`.
    pub async fn update_controls_resource_limits(
        &self,
        group_id: &str,
        limit_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/resource-limits/{}", encode_path(group_id), encode_path(limit_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /family/[groupId]/controls/time-policies`.
    pub async fn controls_time_policies(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/time-policies", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /family/[groupId]/controls/time-policies`.
    pub async fn post_controls_time_policies(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/time-policies", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /family/[groupId]/controls/time-policies/[policyId]`.
    pub async fn delete_controls_time_policies(
        &self,
        group_id: &str,
        policy_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/time-policies/{}", encode_path(group_id), encode_path(policy_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /family/[groupId]/controls/time-policies/[policyId]`.
    pub async fn update_controls_time_policies(
        &self,
        group_id: &str,
        policy_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/controls/time-policies/{}", encode_path(group_id), encode_path(policy_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `DELETE /family/[groupId]/gdpr`.
    pub async fn delete_gdpr(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/gdpr", encode_path(group_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /family/[groupId]/gdpr`.
    pub async fn gdpr(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/gdpr", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /family/[groupId]/gdpr`.
    pub async fn post_gdpr(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/gdpr", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /family/[groupId]/members`.
    pub async fn post_members(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/members", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /family/[groupId]/messages`.
    pub async fn messages(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/messages", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /family/[groupId]/messages`.
    pub async fn post_messages(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/messages", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `PATCH /family/[groupId]/messages/[messageId]`.
    pub async fn update_messages(
        &self,
        group_id: &str,
        message_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/messages/{}", encode_path(group_id), encode_path(message_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /family/[groupId]/monitoring`.
    pub async fn monitoring(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/monitoring", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /family/[groupId]/payment-approvals`.
    pub async fn payment_approvals(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/payment-approvals", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /family/[groupId]/payment-approvals/[approvalId]`.
    pub async fn update_payment_approvals(
        &self,
        group_id: &str,
        approval_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/payment-approvals/{}", encode_path(group_id), encode_path(approval_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /family/[groupId]/spending-limits`.
    pub async fn spending_limits(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/spending-limits", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /family/[groupId]/spending-limits`.
    pub async fn post_spending_limits(
        &self,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/spending-limits", encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /family/[groupId]/spending-limits/[limitId]`.
    pub async fn delete_spending_limits(
        &self,
        group_id: &str,
        limit_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/spending-limits/{}", encode_path(group_id), encode_path(limit_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /family/[groupId]/spending-limits/[limitId]`.
    pub async fn update_spending_limits(
        &self,
        group_id: &str,
        limit_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/spending-limits/{}", encode_path(group_id), encode_path(limit_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /family/[groupId]/usage`.
    pub async fn usage(
        &self,
        group_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/{}/usage", encode_path(group_id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /family/invite/[token]`.
    pub async fn get_invite(
        &self,
        token: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/invite/{}", encode_path(token));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /family/invite/[token]`.
    pub async fn post_invite(
        &self,
        token: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/family/invite/{}", encode_path(token));
        self.transport.post(&path, body).await
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
