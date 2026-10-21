use std::collections::HashMap;

use serde_json::{json, Value};

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{
    ActionDispatchResponse, ProviderViewResponse, ServiceDetailResponse, ServiceListResponse,
    StatusBatchRequest, StatusBatchResponse,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct ServicesResource {
    transport: HttpTransport,
}

impl ServicesResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn find(&self, id: &str) -> Result<ServiceDetailResponse, IgniteError> {
        self.transport.get(&format!("/services/{id}"), None).await
    }

    pub async fn list(
        &self,
        limit: u64,
        view: Option<&str>,
    ) -> Result<ServiceListResponse, IgniteError> {
        let mut pairs = vec![("limit", json!(limit))];
        if let Some(v) = view {
            pairs.push(("view", json!(v)));
        }
        let query = query_params(&pairs);
        self.transport.get("/services", Some(query)).await
    }

    pub async fn view(&self, id: &str) -> Result<ProviderViewResponse, IgniteError> {
        let path = format!("/services/{}/view", urlencoding_encode(id));
        self.transport.get(&path, None).await
    }

    pub async fn action(
        &self,
        id: &str,
        key: &str,
        body: Option<HashMap<String, Value>>,
    ) -> Result<ActionDispatchResponse, IgniteError> {
        let path = format!(
            "/services/{}/actions/{}",
            urlencoding_encode(id),
            urlencoding_encode(key)
        );
        let payload = body.unwrap_or_default();
        self.transport
            .post(&path, Some(Value::Object(payload.into_iter().collect())))
            .await
    }

    pub async fn status_batch(&self, ids: Vec<String>) -> Result<StatusBatchResponse, IgniteError> {
        self.transport
            .post(
                "/services/status-batch",
                Some(serde_json::to_value(StatusBatchRequest { ids })?),
            )
            .await
    }

    pub async fn storage_history(
        &self,
        id: &str,
        hours: u64,
    ) -> Result<serde_json::Value, IgniteError> {
        let path = format!("/services/{}/ploi/storage-history", urlencoding_encode(id));
        let query = query_params(&[("hours", json!(hours))]);
        self.transport.get(&path, Some(query)).await
    }

    pub async fn site_dns(
        &self,
        id: &str,
        body: Option<HashMap<String, Value>>,
    ) -> Result<serde_json::Value, IgniteError> {
        let path = format!("/services/{}/ploi/dns", urlencoding_encode(id));
        match body {
            Some(payload) => {
                self.transport
                    .post(&path, Some(Value::Object(payload.into_iter().collect())))
                    .await
            }
            None => self.transport.get(&path, None).await,
        }
    }

    pub async fn hosting_credentials(&self, id: &str) -> Result<serde_json::Value, IgniteError> {
        let path = format!("/services/{}/plesk/credentials", urlencoding_encode(id));
        self.transport.get(&path, None).await
    }

    pub async fn rotate_hosting_credentials(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, IgniteError> {
        let path = format!("/services/{}/plesk/credentials", urlencoding_encode(id));
        self.transport.post(&path, Some(json!({}))).await
    }

    pub async fn hosting_domains(&self, id: &str) -> Result<serde_json::Value, IgniteError> {
        let path = format!("/services/{}/plesk/domains", urlencoding_encode(id));
        self.transport.get(&path, None).await
    }

    pub async fn refresh_hosting_domains(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, IgniteError> {
        let path = format!("/services/{}/plesk/domains", urlencoding_encode(id));
        self.transport.post(&path, Some(json!({}))).await
    }

    // <generated-openapi>
    /// Generated `POST /services`.
    pub async fn create(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/services", body).await
    }

    /// Generated `DELETE /services/[id]`.
    pub async fn remove(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /services/[id]`.
    pub async fn update(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /services/[id]/action`.
    pub async fn post_action(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/action", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/actions/[key]`.
    pub async fn actions(
        &self,
        id: &str,
        key: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/actions/{}", encode_path(id), encode_path(key));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/activity`.
    pub async fn activity(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/activity", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/allocations`.
    pub async fn allocations(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/allocations", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/apply-downgrade`.
    pub async fn apply_downgrade(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/apply-downgrade", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/backup-storage`.
    pub async fn backup_storage(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backup-storage", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/backup-storage/namespaces`.
    pub async fn backup_storage_namespaces(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backup-storage/namespaces", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /services/[id]/backup-storage/namespaces/[...path]`.
    pub async fn delete_backup_storage_namespaces(
        &self,
        id: &str,
        wildcard: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backup-storage/namespaces/{}", encode_path(id), wildcard.trim_start_matches('/'));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /services/[id]/backups`.
    pub async fn backups(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backups", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/backups`.
    pub async fn post_backups(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backups", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /services/[id]/backups/[backupId]`.
    pub async fn delete_backups(
        &self,
        id: &str,
        backup_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backups/{}", encode_path(id), encode_path(backup_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `POST /services/[id]/backups/[backupId]`.
    pub async fn post_backups_id_backup_id(
        &self,
        id: &str,
        backup_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backups/{}", encode_path(id), encode_path(backup_id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/backups/[backupId]/download`.
    pub async fn backups_download(
        &self,
        id: &str,
        backup_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backups/{}/download", encode_path(id), encode_path(backup_id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/backups/prune`.
    pub async fn backups_prune(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backups/prune", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/backups/prune`.
    pub async fn post_backups_prune(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backups/prune", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/backups/prune/[jobId]`.
    pub async fn get_backups_prune(
        &self,
        id: &str,
        job_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/backups/prune/{}", encode_path(id), encode_path(job_id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/bandwidth-usage`.
    pub async fn bandwidth_usage(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/bandwidth-usage", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/billing-cycles`.
    pub async fn billing_cycles(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/billing-cycles", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/console`.
    pub async fn console(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/console", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/console`.
    pub async fn post_console(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/console", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/console/command`.
    pub async fn console_command(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/console/command", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/console/embed`.
    pub async fn console_embed(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/console/embed", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/console/stream`.
    pub async fn console_stream(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/console/stream", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/content/install`.
    pub async fn content_install(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/content/install", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/content/jobs`.
    pub async fn content_jobs(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/content/jobs", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/content/jobs/[jobId]`.
    pub async fn get_content_jobs(
        &self,
        id: &str,
        job_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/content/jobs/{}", encode_path(id), encode_path(job_id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/content/projects/[projectId]`.
    pub async fn content_projects(
        &self,
        id: &str,
        project_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/content/projects/{}", encode_path(id), encode_path(project_id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/content/search`.
    pub async fn content_search(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/content/search", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/content/sources`.
    pub async fn content_sources(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/content/sources", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/contract-documents`.
    pub async fn contract_documents(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/contract-documents", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/contract-documents/[filename]`.
    pub async fn get_contract_documents(
        &self,
        id: &str,
        filename: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/contract-documents/{}", encode_path(id), encode_path(filename));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/datacenter/cabling`.
    pub async fn datacenter_cabling(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/datacenter/cabling", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/datacenter/devices`.
    pub async fn datacenter_devices(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/datacenter/devices", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/datacenter/devices`.
    pub async fn post_datacenter_devices(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/datacenter/devices", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /services/[id]/datacenter/devices/[deviceId]`.
    pub async fn delete_datacenter_devices(
        &self,
        id: &str,
        device_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/datacenter/devices/{}", encode_path(id), encode_path(device_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /services/[id]/datacenter/devices/[deviceId]`.
    pub async fn update_datacenter_devices(
        &self,
        id: &str,
        device_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/datacenter/devices/{}", encode_path(id), encode_path(device_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /services/[id]/datacenter/rack`.
    pub async fn datacenter_rack(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/datacenter/rack", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/disk-usage`.
    pub async fn disk_usage(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/disk-usage", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/domain/withdraw`.
    pub async fn domain_withdraw(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/domain/withdraw", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/eggs`.
    pub async fn eggs(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/eggs", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/features`.
    pub async fn features(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/features", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/files`.
    pub async fn files(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/files", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/files`.
    pub async fn post_files(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/files", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/firewall`.
    pub async fn firewall(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/firewall", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/firewall`.
    pub async fn post_firewall(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/firewall", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /services/[id]/firewall/[ruleId]`.
    pub async fn delete_firewall(
        &self,
        id: &str,
        rule_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/firewall/{}", encode_path(id), encode_path(rule_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /services/[id]/firewall/[ruleId]`.
    pub async fn update_firewall(
        &self,
        id: &str,
        rule_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/firewall/{}", encode_path(id), encode_path(rule_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /services/[id]/game-auth`.
    pub async fn game_auth(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/game-auth", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/game-auth`.
    pub async fn post_game_auth(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/game-auth", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/game/eula`.
    pub async fn game_eula(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/game/eula", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/game/eula`.
    pub async fn post_game_eula(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/game/eula", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /services/[id]/ips/cancel`.
    pub async fn delete_ips_cancel(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/ips/cancel", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `POST /services/[id]/ips/cancel`.
    pub async fn ips_cancel(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/ips/cancel", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/keyhelp/credentials`.
    pub async fn keyhelp_credentials(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/keyhelp/credentials", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/keyhelp/credentials`.
    pub async fn post_keyhelp_credentials(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/keyhelp/credentials", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/keyhelp/domains`.
    pub async fn keyhelp_domains(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/keyhelp/domains", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/keyhelp/domains`.
    pub async fn post_keyhelp_domains(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/keyhelp/domains", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/keyhelp/reset`.
    pub async fn keyhelp_reset(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/keyhelp/reset", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/keyhelp/setup`.
    pub async fn keyhelp_setup(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/keyhelp/setup", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/keyhelp/setup`.
    pub async fn post_keyhelp_setup(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/keyhelp/setup", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/labels`.
    pub async fn labels(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/labels", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/labels`.
    pub async fn post_labels(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/labels", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /services/[id]/labels/[labelId]`.
    pub async fn delete_labels(
        &self,
        id: &str,
        label_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/labels/{}", encode_path(id), encode_path(label_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /services/[id]/labels/[labelId]`.
    pub async fn update_labels(
        &self,
        id: &str,
        label_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/labels/{}", encode_path(id), encode_path(label_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `PUT /services/[id]/labels/assignments`.
    pub async fn replace_labels_assignments(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/labels/assignments", encode_path(id));
        self.transport.put(&path, body).await
    }

    /// Generated `DELETE /services/[id]/mail/aliases`.
    pub async fn delete_mail_aliases(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/aliases", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /services/[id]/mail/aliases`.
    pub async fn mail_aliases(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/aliases", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/mail/aliases`.
    pub async fn post_mail_aliases(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/aliases", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/mail/dns-zones`.
    pub async fn mail_dns_zones(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/dns-zones", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `DELETE /services/[id]/mail/domains`.
    pub async fn delete_mail_domains(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/domains", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /services/[id]/mail/domains`.
    pub async fn mail_domains(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/domains", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/mail/domains`.
    pub async fn post_mail_domains(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/domains", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/mail/domains/apply-dns`.
    pub async fn mail_domains_apply_dns(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/domains/apply-dns", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/mail/domains/dns`.
    pub async fn mail_domains_dns(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/domains/dns", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/mail/domains/verify-dns`.
    pub async fn mail_domains_verify_dns(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/domains/verify-dns", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /services/[id]/mail/mailboxes`.
    pub async fn delete_mail_mailboxes(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/mailboxes", encode_path(id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /services/[id]/mail/mailboxes`.
    pub async fn mail_mailboxes(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/mailboxes", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /services/[id]/mail/mailboxes`.
    pub async fn update_mail_mailboxes(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/mailboxes", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /services/[id]/mail/mailboxes`.
    pub async fn post_mail_mailboxes(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/mail/mailboxes", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/metrics`.
    pub async fn metrics(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/metrics", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/monitoring/config`.
    pub async fn monitoring_config(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/config", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /services/[id]/monitoring/config`.
    pub async fn update_monitoring_config(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/config", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /services/[id]/monitoring/history`.
    pub async fn monitoring_history(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/history", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/monitoring/push/device-token`.
    pub async fn monitoring_push_device_token(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/push/device-token", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/monitoring/push/public-key`.
    pub async fn monitoring_push_public_key(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/push/public-key", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/monitoring/push/subscribe`.
    pub async fn monitoring_push_subscribe(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/push/subscribe", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/monitoring/score`.
    pub async fn monitoring_score(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/score", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/monitoring/targets`.
    pub async fn monitoring_targets(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/targets", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/monitoring/targets`.
    pub async fn post_monitoring_targets(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/targets", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `DELETE /services/[id]/monitoring/targets/[targetId]`.
    pub async fn delete_monitoring_targets(
        &self,
        id: &str,
        target_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/targets/{}", encode_path(id), encode_path(target_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `PATCH /services/[id]/monitoring/targets/[targetId]`.
    pub async fn update_monitoring_targets(
        &self,
        id: &str,
        target_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/monitoring/targets/{}", encode_path(id), encode_path(target_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /services/[id]/notifications`.
    pub async fn notifications(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/notifications", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/players`.
    pub async fn players(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/players", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/players/action`.
    pub async fn players_action(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/players/action", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/plesk/credentials`.
    pub async fn plesk_credentials(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/plesk/credentials", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/plesk/domains`.
    pub async fn plesk_domains(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/plesk/domains", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/plesk/domains`.
    pub async fn post_plesk_domains(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/plesk/domains", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/plesk/reset`.
    pub async fn plesk_reset(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/plesk/reset", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/plesk/setup`.
    pub async fn plesk_setup(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/plesk/setup", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/plesk/setup`.
    pub async fn post_plesk_setup(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/plesk/setup", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/ploi/dns`.
    pub async fn ploi_dns(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/ploi/dns", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/reinstall`.
    pub async fn reinstall(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/reinstall", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/renew`.
    pub async fn renew(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/renew", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/settings`.
    pub async fn settings(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/settings", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /services/[id]/settings`.
    pub async fn update_settings(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/settings", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /services/[id]/settings`.
    pub async fn post_settings(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/settings", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `PUT /services/[id]/settings`.
    pub async fn replace_settings(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/settings", encode_path(id));
        self.transport.put(&path, body).await
    }

    /// Generated `GET /services/[id]/sftp`.
    pub async fn sftp(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sftp", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/sftp/ensure-password`.
    pub async fn sftp_ensure_password(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sftp/ensure-password", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/sftp/rotate-password`.
    pub async fn sftp_rotate_password(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sftp/rotate-password", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/sla`.
    pub async fn sla(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sla", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/sla`.
    pub async fn post_sla(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sla", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/sla/accept`.
    pub async fn sla_accept(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sla/accept", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/sla/amend`.
    pub async fn sla_amend(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sla/amend", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/sla/cancel`.
    pub async fn sla_cancel(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sla/cancel", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/sla/decline`.
    pub async fn sla_decline(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sla/decline", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/sla/documents/[docId]`.
    pub async fn sla_documents(
        &self,
        id: &str,
        doc_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sla/documents/{}", encode_path(id), encode_path(doc_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/sla/revoke-cancel`.
    pub async fn sla_revoke_cancel(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sla/revoke-cancel", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/sla/withdraw`.
    pub async fn sla_withdraw(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/sla/withdraw", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/startup-variables`.
    pub async fn startup_variables(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/startup-variables", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /services/[id]/startup-variables`.
    pub async fn update_startup_variables(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/startup-variables", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `PUT /services/[id]/startup-variables`.
    pub async fn replace_startup_variables(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/startup-variables", encode_path(id));
        self.transport.put(&path, body).await
    }

    /// Generated `GET /services/[id]/status`.
    pub async fn status(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/status", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/subscription`.
    pub async fn subscription(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/subscription", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/subscription/accelerate`.
    pub async fn subscription_accelerate(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/subscription/accelerate", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `PATCH /services/[id]/subscription/autopay`.
    pub async fn update_subscription_autopay(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/subscription/autopay", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `PATCH /services/[id]/subscription/billing-address`.
    pub async fn update_subscription_billing_address(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/subscription/billing-address", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /services/[id]/subscription/cancel`.
    pub async fn subscription_cancel(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/subscription/cancel", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/subscription/extend`.
    pub async fn subscription_extend(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/subscription/extend", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `PATCH /services/[id]/subscription/interval`.
    pub async fn update_subscription_interval(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/subscription/interval", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /services/[id]/subscription/reactivate`.
    pub async fn subscription_reactivate(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/subscription/reactivate", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/subscription/withdraw`.
    pub async fn subscription_withdraw(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/subscription/withdraw", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/teamspeak`.
    pub async fn teamspeak(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/teamspeak", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/teamspeak`.
    pub async fn post_teamspeak(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/teamspeak", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/templates`.
    pub async fn templates(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/templates", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/top-ips`.
    pub async fn top_ips(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/top-ips", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/traffic`.
    pub async fn traffic(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/traffic", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /services/[id]/upgrade`.
    pub async fn upgrade(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/upgrade", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/upgrade`.
    pub async fn post_upgrade(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/upgrade", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /services/[id]/upgrade-preview`.
    pub async fn upgrade_preview(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/upgrade-preview", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /services/[id]/usage-export`.
    pub async fn usage_export(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/usage-export", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /services/[id]/usage-export`.
    pub async fn post_usage_export(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/services/{}/usage-export", encode_path(id));
        self.transport.post(&path, body).await
    }
    // </generated-openapi>
}

fn urlencoding_encode(value: &str) -> String {
    value
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}
