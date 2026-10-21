use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct TeamResource {
    transport: HttpTransport,
}

impl TeamResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /team`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/team", query).await
    }

    /// Generated `POST /team`.
    pub async fn create(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/team", body).await
    }

    /// Generated `DELETE /team/[teamId]`.
    pub async fn remove(
        &self,
        team_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}", encode_path(team_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /team/[teamId]`.
    pub async fn find(
        &self,
        team_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}", encode_path(team_id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /team/[teamId]`.
    pub async fn update(
        &self,
        team_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}", encode_path(team_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /team/[teamId]/audit`.
    pub async fn audit(
        &self,
        team_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/audit", encode_path(team_id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /team/[teamId]/groups`.
    pub async fn groups(
        &self,
        team_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/groups", encode_path(team_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /team/[teamId]/groups`.
    pub async fn post_groups(
        &self,
        team_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/groups", encode_path(team_id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /team/[teamId]/groups/[groupId]`.
    pub async fn delete_groups(
        &self,
        team_id: &str,
        group_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/groups/{}", encode_path(team_id), encode_path(group_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /team/[teamId]/groups/[groupId]`.
    pub async fn update_groups(
        &self,
        team_id: &str,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/groups/{}", encode_path(team_id), encode_path(group_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `DELETE /team/[teamId]/groups/[groupId]/members`.
    pub async fn delete_groups_members(
        &self,
        team_id: &str,
        group_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/groups/{}/members", encode_path(team_id), encode_path(group_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `POST /team/[teamId]/groups/[groupId]/members`.
    pub async fn groups_members(
        &self,
        team_id: &str,
        group_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/groups/{}/members", encode_path(team_id), encode_path(group_id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /team/[teamId]/invites`.
    pub async fn invites(
        &self,
        team_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/invites", encode_path(team_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /team/[teamId]/invites`.
    pub async fn post_invites(
        &self,
        team_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/invites", encode_path(team_id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /team/[teamId]/invites/[inviteId]`.
    pub async fn delete_invites(
        &self,
        team_id: &str,
        invite_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/invites/{}", encode_path(team_id), encode_path(invite_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /team/[teamId]/members`.
    pub async fn members(
        &self,
        team_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/members", encode_path(team_id));
        self.transport.get(&path, query).await
    }

    /// Generated `DELETE /team/[teamId]/members/[userId]`.
    pub async fn delete_members(
        &self,
        team_id: &str,
        user_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/members/{}", encode_path(team_id), encode_path(user_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /team/[teamId]/members/[userId]`.
    pub async fn update_members(
        &self,
        team_id: &str,
        user_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/members/{}", encode_path(team_id), encode_path(user_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /team/[teamId]/ownership-transfer`.
    pub async fn ownership_transfer(
        &self,
        team_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/ownership-transfer", encode_path(team_id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /team/[teamId]/payment-methods`.
    pub async fn payment_methods(
        &self,
        team_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/payment-methods", encode_path(team_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /team/[teamId]/payment-methods`.
    pub async fn post_payment_methods(
        &self,
        team_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/payment-methods", encode_path(team_id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /team/[teamId]/projects`.
    pub async fn projects(
        &self,
        team_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/projects", encode_path(team_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /team/[teamId]/projects`.
    pub async fn post_projects(
        &self,
        team_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/projects", encode_path(team_id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /team/[teamId]/projects/[projectId]`.
    pub async fn post_projects_team_id_project_id(
        &self,
        team_id: &str,
        project_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/projects/{}", encode_path(team_id), encode_path(project_id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /team/[teamId]/subscriptions/[subscriptionId]/cancel`.
    pub async fn subscriptions_cancel(
        &self,
        team_id: &str,
        subscription_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/{}/subscriptions/{}/cancel", encode_path(team_id), encode_path(subscription_id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /team/invite/[token]`.
    pub async fn invite(
        &self,
        token: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/invite/{}", encode_path(token));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /team/invite/[token]`.
    pub async fn post_invite(
        &self,
        token: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/team/invite/{}", encode_path(token));
        self.transport.post(&path, body).await
    }
    // </generated-openapi>

}
