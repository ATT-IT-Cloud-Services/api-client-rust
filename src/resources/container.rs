use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct ContainerResource {
    transport: HttpTransport,
}

impl ContainerResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /container/access`.
    pub async fn access(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/container/access", query).await
    }

    /// Generated `GET /container/projects`.
    pub async fn projects(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/container/projects", query).await
    }

    /// Generated `POST /container/projects`.
    pub async fn post_projects(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/container/projects", body).await
    }

    /// Generated `GET /container/projects/[id]`.
    pub async fn get_projects(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/projects/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /container/projects/[id]/domains`.
    pub async fn projects_domains(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/projects/{}/domains", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /container/projects/[id]/import`.
    pub async fn projects_import(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/projects/{}/import", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /container/projects/[id]/networks`.
    pub async fn projects_networks(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/projects/{}/networks", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /container/projects/[id]/ports`.
    pub async fn projects_ports(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/projects/{}/ports", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /container/projects/[id]/templates`.
    pub async fn projects_templates(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/projects/{}/templates", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /container/projects/[id]/templates`.
    pub async fn post_projects_templates(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/projects/{}/templates", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /container/projects/[id]/workloads`.
    pub async fn projects_workloads(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/projects/{}/workloads", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /container/regions`.
    pub async fn regions(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/container/regions", query).await
    }

    /// Generated `GET /container/workloads/[id]`.
    pub async fn workloads(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/workloads/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /container/workloads/[id]/actions`.
    pub async fn workloads_actions(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/workloads/{}/actions", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /container/workloads/[id]/env`.
    pub async fn workloads_env(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/workloads/{}/env", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PUT /container/workloads/[id]/env`.
    pub async fn replace_workloads_env(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/workloads/{}/env", encode_path(id));
        self.transport.put(&path, body).await
    }

    /// Generated `GET /container/workloads/[id]/files`.
    pub async fn workloads_files(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/workloads/{}/files", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /container/workloads/[id]/files`.
    pub async fn post_workloads_files(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/workloads/{}/files", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /container/workloads/[id]/logs`.
    pub async fn workloads_logs(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/workloads/{}/logs", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /container/workloads/[id]/monitoring`.
    pub async fn workloads_monitoring(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/container/workloads/{}/monitoring", encode_path(id));
        self.transport.get(&path, query).await
    }
    // </generated-openapi>

}
