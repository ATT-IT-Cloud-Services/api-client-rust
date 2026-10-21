use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct PublicApiResource {
    transport: HttpTransport,
}

impl PublicApiResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `POST /public/abuse`.
    pub async fn abuse(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/abuse", body).await
    }

    /// Generated `GET /public/appearance`.
    pub async fn appearance(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/appearance", query).await
    }

    /// Generated `GET /public/appearance/assets/[key]`.
    pub async fn appearance_assets(
        &self,
        key: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/public/appearance/assets/{}", encode_path(key));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /public/blog`.
    pub async fn blog(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/blog", query).await
    }

    /// Generated `GET /public/blog/[slug]`.
    pub async fn get_blog(
        &self,
        slug: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/public/blog/{}", encode_path(slug));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /public/cancellation`.
    pub async fn cancellation(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/cancellation", body).await
    }

    /// Generated `POST /public/case-objection`.
    pub async fn case_objection(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/case-objection", body).await
    }

    /// Generated `POST /public/case-report`.
    pub async fn case_report(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/case-report", body).await
    }

    /// Generated `GET /public/case-status`.
    pub async fn case_status(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/case-status", query).await
    }

    /// Generated `POST /public/domains/starting-price`.
    pub async fn domains_starting_price(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/domains/starting-price", body).await
    }

    /// Generated `GET /public/footer-proxy/status-availability`.
    pub async fn footer_proxy_status_availability(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/footer-proxy/status-availability", query).await
    }

    /// Generated `GET /public/footer-proxy/status-pulse`.
    pub async fn footer_proxy_status_pulse(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/footer-proxy/status-pulse", query).await
    }

    /// Generated `GET /public/legal/[doc]`.
    pub async fn legal(
        &self,
        doc: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/public/legal/{}", encode_path(doc));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /public/navigation`.
    pub async fn navigation(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/navigation", query).await
    }

    /// Generated `GET /public/providers/[id]/catalog`.
    pub async fn providers_catalog(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/public/providers/{}/catalog", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /public/registration-status`.
    pub async fn registration_status(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/registration-status", query).await
    }

    /// Generated `POST /public/report-minor`.
    pub async fn report_minor(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/report-minor", body).await
    }

    /// Generated `GET /public/reviews`.
    pub async fn reviews(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/reviews", query).await
    }

    /// Generated `GET /public/setup`.
    pub async fn setup(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/setup", query).await
    }

    /// Generated `PATCH /public/setup`.
    pub async fn update_setup(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.patch("/public/setup", body).await
    }

    /// Generated `GET /public/setup-status`.
    pub async fn setup_status(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/setup-status", query).await
    }

    /// Generated `GET /public/setup/admin`.
    pub async fn setup_admin(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/setup/admin", query).await
    }

    /// Generated `POST /public/setup/admin`.
    pub async fn post_setup_admin(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/setup/admin", body).await
    }

    /// Generated `POST /public/setup/assets`.
    pub async fn setup_assets(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/setup/assets", body).await
    }

    /// Generated `POST /public/setup/complete`.
    pub async fn setup_complete(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/setup/complete", body).await
    }

    /// Generated `GET /public/setup/legal`.
    pub async fn setup_legal(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/setup/legal", query).await
    }

    /// Generated `PUT /public/setup/legal`.
    pub async fn replace_setup_legal(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.put("/public/setup/legal", body).await
    }

    /// Generated `GET /public/support-hours`.
    pub async fn support_hours(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/support-hours", query).await
    }

    /// Generated `GET /public/support-stats`.
    pub async fn support_stats(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/support-stats", query).await
    }

    /// Generated `POST /public/takedown`.
    pub async fn takedown(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/public/takedown", body).await
    }

    /// Generated `GET /public/tenant`.
    pub async fn tenant(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/public/tenant", query).await
    }
    // </generated-openapi>

}
