use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct PodsResource {
    transport: HttpTransport,
}

impl PodsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `DELETE /pods/[id]`.
    pub async fn remove(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/[id]`.
    pub async fn find(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /pods/[id]`.
    pub async fn update(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /pods/[id]/actions`.
    pub async fn actions(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/actions", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/actions`.
    pub async fn post_actions(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/actions", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /pods/[id]/allocations`.
    pub async fn delete_allocations(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/allocations", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `POST /pods/[id]/allocations`.
    pub async fn allocations(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/allocations", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/backups`.
    pub async fn backups(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/backups", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/backups`.
    pub async fn post_backups(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/backups", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/backups/[backupId]/download`.
    pub async fn backups_download(
        &self,
        id: &str,
        backup_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/backups/{}/download", encode_path(id), encode_path(backup_id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /pods/[id]/deploy`.
    pub async fn deploy(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/deploy", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `DELETE /pods/[id]/domains`.
    pub async fn delete_domains(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/domains", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/[id]/domains`.
    pub async fn domains(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/domains", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/domains`.
    pub async fn post_domains(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/domains", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/env`.
    pub async fn env(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/env", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PUT /pods/[id]/env`.
    pub async fn replace_env(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/env", encode_path(id));
        self.transport.put(&path, body).await
    }

    /// Generated `GET /pods/[id]/files`.
    pub async fn files(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/files", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/files`.
    pub async fn post_files(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/files", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /pods/[id]/forwards`.
    pub async fn delete_forwards(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/forwards", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/[id]/forwards`.
    pub async fn forwards(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/forwards", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /pods/[id]/forwards`.
    pub async fn update_forwards(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/forwards", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /pods/[id]/forwards`.
    pub async fn post_forwards(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/forwards", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/logs`.
    pub async fn logs(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/logs", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/logs`.
    pub async fn post_logs(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/logs", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/platform`.
    pub async fn platform(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/platform", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PUT /pods/[id]/platform`.
    pub async fn replace_platform(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/platform", encode_path(id));
        self.transport.put(&path, body).await
    }

    /// Generated `DELETE /pods/[id]/ports`.
    pub async fn delete_ports(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/ports", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/[id]/ports`.
    pub async fn ports(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/ports", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /pods/[id]/ports`.
    pub async fn update_ports(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/ports", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /pods/[id]/ports`.
    pub async fn post_ports(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/ports", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/revisions`.
    pub async fn revisions(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/revisions", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/revisions`.
    pub async fn post_revisions(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/revisions", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/schedules`.
    pub async fn schedules(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/schedules", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/schedules`.
    pub async fn post_schedules(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/schedules", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/stats`.
    pub async fn stats(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/stats", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/transfer`.
    pub async fn transfer(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/transfer", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/volume`.
    pub async fn volume(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/volume", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/volume`.
    pub async fn post_volume(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/volume", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/[id]/volumes`.
    pub async fn volumes(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/volumes", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/[id]/volumes`.
    pub async fn post_volumes(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/{}/volumes", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/access`.
    pub async fn access(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/access", query).await
    }

    /// Generated `GET /pods/api-keys`.
    pub async fn api_keys(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/api-keys", query).await
    }

    /// Generated `POST /pods/api-keys`.
    pub async fn post_api_keys(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/api-keys", body).await
    }

    /// Generated `DELETE /pods/api-keys/[id]`.
    pub async fn delete_api_keys(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/api-keys/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/audit`.
    pub async fn audit(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/audit", query).await
    }

    /// Generated `GET /pods/billing`.
    pub async fn billing(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/billing", query).await
    }

    /// Generated `GET /pods/cost-centers`.
    pub async fn cost_centers(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/cost-centers", query).await
    }

    /// Generated `POST /pods/cost-centers`.
    pub async fn post_cost_centers(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/cost-centers", body).await
    }

    /// Generated `GET /pods/events`.
    pub async fn events(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/events", query).await
    }

    /// Generated `POST /pods/events`.
    pub async fn post_events(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/events", body).await
    }

    /// Generated `DELETE /pods/ips/[id]`.
    pub async fn delete_ips(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/ips/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /pods/ips/[id]`.
    pub async fn update_ips(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/ips/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /pods/metrics`.
    pub async fn metrics(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/metrics", query).await
    }

    /// Generated `POST /pods/metrics`.
    pub async fn post_metrics(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/metrics", body).await
    }

    /// Generated `GET /pods/notification-channels`.
    pub async fn notification_channels(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/notification-channels", query).await
    }

    /// Generated `POST /pods/notification-channels`.
    pub async fn post_notification_channels(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/notification-channels", body).await
    }

    /// Generated `GET /pods/notifications`.
    pub async fn notifications(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/notifications", query).await
    }

    /// Generated `POST /pods/notifications`.
    pub async fn post_notifications(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/notifications", body).await
    }

    /// Generated `GET /pods/packages`.
    pub async fn packages(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/packages", query).await
    }

    /// Generated `POST /pods/packages`.
    pub async fn post_packages(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/packages", body).await
    }

    /// Generated `GET /pods/projects`.
    pub async fn projects(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/projects", query).await
    }

    /// Generated `POST /pods/projects`.
    pub async fn post_projects(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/projects", body).await
    }

    /// Generated `DELETE /pods/projects/[id]`.
    pub async fn delete_projects(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/projects/[id]`.
    pub async fn get_projects(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /pods/projects/[id]`.
    pub async fn update_projects(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `DELETE /pods/projects/[id]/backup-policy`.
    pub async fn delete_projects_backup_policy(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/backup-policy", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/projects/[id]/backup-policy`.
    pub async fn projects_backup_policy(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/backup-policy", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PUT /pods/projects/[id]/backup-policy`.
    pub async fn replace_projects_backup_policy(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/backup-policy", encode_path(id));
        self.transport.put(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/billing-report`.
    pub async fn projects_billing_report(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/billing-report", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/bulk`.
    pub async fn projects_bulk(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/bulk", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /pods/projects/[id]/certificates`.
    pub async fn delete_projects_certificates(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/certificates", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/projects/[id]/certificates`.
    pub async fn projects_certificates(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/certificates", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/certificates`.
    pub async fn post_projects_certificates(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/certificates", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/compose`.
    pub async fn projects_compose(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/compose", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/compose`.
    pub async fn post_projects_compose(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/compose", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /pods/projects/[id]/databases`.
    pub async fn delete_projects_databases(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/databases", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/projects/[id]/databases`.
    pub async fn projects_databases(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/databases", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/databases`.
    pub async fn post_projects_databases(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/databases", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /pods/projects/[id]/gateway-tokens`.
    pub async fn delete_projects_gateway_tokens(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/gateway-tokens", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/projects/[id]/gateway-tokens`.
    pub async fn projects_gateway_tokens(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/gateway-tokens", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/gateway-tokens`.
    pub async fn post_projects_gateway_tokens(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/gateway-tokens", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/git`.
    pub async fn projects_git(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/git", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/git`.
    pub async fn post_projects_git(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/git", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/images`.
    pub async fn projects_images(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/images", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/images`.
    pub async fn post_projects_images(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/images", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/ips`.
    pub async fn projects_ips(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/ips", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/ips`.
    pub async fn post_projects_ips(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/ips", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/ips/available`.
    pub async fn projects_ips_available(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/ips/available", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /pods/projects/[id]/jobs`.
    pub async fn projects_jobs(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/jobs", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/jobs`.
    pub async fn post_projects_jobs(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/jobs", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /pods/projects/[id]/listeners`.
    pub async fn delete_projects_listeners(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/listeners", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/projects/[id]/listeners`.
    pub async fn projects_listeners(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/listeners", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/listeners`.
    pub async fn post_projects_listeners(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/listeners", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/networks`.
    pub async fn projects_networks(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/networks", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/networks`.
    pub async fn post_projects_networks(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/networks", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/pods`.
    pub async fn projects_pods(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/pods", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/pods`.
    pub async fn post_projects_pods(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/pods", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/reference`.
    pub async fn projects_reference(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/reference", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `DELETE /pods/projects/[id]/runtime-groups`.
    pub async fn delete_projects_runtime_groups(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/runtime-groups", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/projects/[id]/runtime-groups`.
    pub async fn projects_runtime_groups(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/runtime-groups", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/runtime-groups`.
    pub async fn post_projects_runtime_groups(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/runtime-groups", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /pods/projects/[id]/storage-backends`.
    pub async fn delete_projects_storage_backends(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/storage-backends", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/projects/[id]/storage-backends`.
    pub async fn projects_storage_backends(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/storage-backends", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/storage-backends`.
    pub async fn post_projects_storage_backends(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/storage-backends", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/templates`.
    pub async fn projects_templates(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/templates", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/templates`.
    pub async fn post_projects_templates(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/templates", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/transfer`.
    pub async fn projects_transfer(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/transfer", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/transfer`.
    pub async fn post_projects_transfer(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/transfer", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/usage`.
    pub async fn projects_usage(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/usage", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /pods/projects/[id]/volumes`.
    pub async fn projects_volumes(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/volumes", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/volumes`.
    pub async fn post_projects_volumes(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/volumes", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/projects/[id]/workflow`.
    pub async fn projects_workflow(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/workflow", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/projects/[id]/workflow`.
    pub async fn post_projects_workflow(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/projects/{}/workflow", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/rdns-zones`.
    pub async fn rdns_zones(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/rdns-zones", query).await
    }

    /// Generated `GET /pods/regions`.
    pub async fn regions(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/regions", query).await
    }

    /// Generated `GET /pods/registry`.
    pub async fn registry(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/registry", query).await
    }

    /// Generated `GET /pods/ssh-keys`.
    pub async fn ssh_keys(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/ssh-keys", query).await
    }

    /// Generated `POST /pods/ssh-keys`.
    pub async fn post_ssh_keys(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/ssh-keys", body).await
    }

    /// Generated `DELETE /pods/ssh-keys/[id]`.
    pub async fn delete_ssh_keys(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/ssh-keys/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /pods/ssh-keys/[id]`.
    pub async fn update_ssh_keys(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/ssh-keys/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /pods/tenants`.
    pub async fn tenants(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/tenants", query).await
    }

    /// Generated `POST /pods/tenants`.
    pub async fn post_tenants(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/tenants", body).await
    }

    /// Generated `DELETE /pods/tenants/[id]`.
    pub async fn delete_tenants(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/tenants/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `DELETE /pods/tenants/[id]/access`.
    pub async fn delete_tenants_access(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/tenants/{}/access", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/tenants/[id]/access`.
    pub async fn tenants_access(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/tenants/{}/access", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/tenants/[id]/access`.
    pub async fn post_tenants_access(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/tenants/{}/access", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /pods/tenants/[id]/transfer-keys`.
    pub async fn delete_tenants_transfer_keys(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/tenants/{}/transfer-keys", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /pods/tenants/[id]/transfer-keys`.
    pub async fn tenants_transfer_keys(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/tenants/{}/transfer-keys", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/tenants/[id]/transfer-keys`.
    pub async fn post_tenants_transfer_keys(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/tenants/{}/transfer-keys", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/transfers/[id]`.
    pub async fn transfers(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/transfers/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/transfers/[id]`.
    pub async fn post_transfers(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/transfers/{}", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/tree`.
    pub async fn tree(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/tree", query).await
    }

    /// Generated `GET /pods/wallet`.
    pub async fn wallet(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/wallet", query).await
    }

    /// Generated `POST /pods/wallet`.
    pub async fn post_wallet(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/wallet", body).await
    }

    /// Generated `GET /pods/webhooks`.
    pub async fn webhooks(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/pods/webhooks", query).await
    }

    /// Generated `POST /pods/webhooks`.
    pub async fn post_webhooks(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/pods/webhooks", body).await
    }

    /// Generated `DELETE /pods/webhooks/[id]`.
    pub async fn delete_webhooks(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/webhooks/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `POST /pods/webhooks/[id]`.
    pub async fn post_webhooks_id(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/webhooks/{}", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /pods/webhooks/[id]/deliveries`.
    pub async fn webhooks_deliveries(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/webhooks/{}/deliveries", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /pods/webhooks/[id]/rotate-secret`.
    pub async fn webhooks_rotate_secret(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/pods/webhooks/{}/rotate-secret", encode_path(id));
        self.transport.post(&path, body).await
    }
    // </generated-openapi>

}
