use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{
    ApiKeysListResponse, AuditLogResponse, CreateApiKeyRequest, CreateApiKeyResponse,
    GdprExportCheckResponse, GdprExportConfirmRequest, SuccessResponse, SupportPinResponse,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct UserResource {
    transport: HttpTransport,
}

impl UserResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn audit_log(&self, limit: u64) -> Result<AuditLogResponse, IgniteError> {
        let query = query_params(&[("limit", json!(limit))]);
        self.transport.get("/user/audit-log", Some(query)).await
    }

    pub async fn gdpr_export_check(&self) -> Result<GdprExportCheckResponse, IgniteError> {
        self.transport.delete("/user/gdpr-export", None, None).await
    }

    pub async fn gdpr_export_confirm(
        &self,
        password: &str,
    ) -> Result<SuccessResponse, IgniteError> {
        let query = query_params(&[("confirm", json!("true"))]);
        self.transport
            .delete(
                "/user/gdpr-export",
                Some(query),
                Some(serde_json::to_value(GdprExportConfirmRequest {
                    password: password.to_string(),
                })?),
            )
            .await
    }

    pub async fn api_keys(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<ApiKeysListResponse, IgniteError> {
        let query = query_params(&[("page", json!(page)), ("pageSize", json!(page_size))]);
        self.transport.get("/api-keys", Some(query)).await
    }

    pub async fn create_api_key(&self, name: &str) -> Result<CreateApiKeyResponse, IgniteError> {
        self.transport
            .post(
                "/api-keys",
                Some(serde_json::to_value(CreateApiKeyRequest {
                    name: name.to_string(),
                })?),
            )
            .await
    }

    pub async fn revoke_api_key(&self, id: &str) -> Result<SuccessResponse, IgniteError> {
        let query = query_params(&[("id", json!(id))]);
        self.transport.delete("/api-keys", Some(query), None).await
    }

    pub async fn support_pin(&self) -> Result<SupportPinResponse, IgniteError> {
        self.transport.get("/user/support-pin", None).await
    }

    pub async fn generate_support_pin(&self) -> Result<SupportPinResponse, IgniteError> {
        self.transport
            .post("/user/support-pin", Some(json!({})))
            .await
    }

    // <generated-openapi>
    /// Generated `GET /api-keys/usage`.
    pub async fn usage(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/api-keys/usage", query).await
    }

    /// Generated `GET /user/autopay`.
    pub async fn autopay(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/autopay", query).await
    }

    /// Generated `PATCH /user/autopay`.
    pub async fn update_autopay(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.patch("/user/autopay", body).await
    }

    /// Generated `GET /user/business-data-review`.
    pub async fn business_data_review(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/business-data-review", query).await
    }

    /// Generated `POST /user/change-email`.
    pub async fn change_email(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/change-email", body).await
    }

    /// Generated `POST /user/change-email/verify-new`.
    pub async fn change_email_verify_new(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/change-email/verify-new", body).await
    }

    /// Generated `POST /user/change-email/verify-old`.
    pub async fn change_email_verify_old(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/change-email/verify-old", body).await
    }

    /// Generated `POST /user/date-of-birth`.
    pub async fn date_of_birth(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/date-of-birth", body).await
    }

    /// Generated `DELETE /user/discord`.
    pub async fn delete_discord(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/user/discord", query, body).await
    }

    /// Generated `GET /user/discord`.
    pub async fn discord(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/discord", query).await
    }

    /// Generated `PATCH /user/discord`.
    pub async fn update_discord(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.patch("/user/discord", body).await
    }

    /// Generated `POST /user/discord`.
    pub async fn post_discord(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/discord", body).await
    }

    /// Generated `GET /user/dpa`.
    pub async fn dpa(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/dpa", query).await
    }

    /// Generated `POST /user/dpa`.
    pub async fn post_dpa(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/dpa", body).await
    }

    /// Generated `POST /user/dpa/cancel`.
    pub async fn dpa_cancel(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/dpa/cancel", body).await
    }

    /// Generated `GET /user/dpa/documents/[id]`.
    pub async fn dpa_documents(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/dpa/documents/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /user/gdpr-export`.
    pub async fn gdpr_export(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/gdpr-export", query).await
    }

    /// Generated `GET /user/gdpr-export/history`.
    pub async fn gdpr_export_history(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/gdpr-export/history", query).await
    }

    /// Generated `GET /user/gdpr-export/history/[id]`.
    pub async fn get_gdpr_export_history(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/gdpr-export/history/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /user/id-verification/[id]/decline`.
    pub async fn id_verification_decline(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/id-verification/{}/decline", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /user/id-verification/[id]/enjyn-restart`.
    pub async fn id_verification_enjyn_restart(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/id-verification/{}/enjyn-restart", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /user/id-verification/[id]/enjyn-start`.
    pub async fn id_verification_enjyn_start(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/id-verification/{}/enjyn-start", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /user/id-verification/[id]/method`.
    pub async fn id_verification_method(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/id-verification/{}/method", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /user/id-verification/[id]/refuse`.
    pub async fn id_verification_refuse(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/id-verification/{}/refuse", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /user/id-verification/[id]/upload`.
    pub async fn id_verification_upload(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/id-verification/{}/upload", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /user/id-verification/enforcement-status`.
    pub async fn id_verification_enforcement_status(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/id-verification/enforcement-status", query).await
    }

    /// Generated `POST /user/id-verification/enjyn-callback`.
    pub async fn id_verification_enjyn_callback(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/id-verification/enjyn-callback", body).await
    }

    /// Generated `POST /user/id-verification/start`.
    pub async fn id_verification_start(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/id-verification/start", body).await
    }

    /// Generated `GET /user/id-verification/status`.
    pub async fn id_verification_status(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/id-verification/status", query).await
    }

    /// Generated `GET /user/newsletter-settings`.
    pub async fn newsletter_settings(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/newsletter-settings", query).await
    }

    /// Generated `POST /user/newsletter-settings`.
    pub async fn post_newsletter_settings(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/newsletter-settings", body).await
    }

    /// Generated `GET /user/notifications`.
    pub async fn notifications(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/notifications", query).await
    }

    /// Generated `PATCH /user/notifications`.
    pub async fn update_notifications(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.patch("/user/notifications", body).await
    }

    /// Generated `GET /user/notifications/[id]`.
    pub async fn get_notifications(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/notifications/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `PATCH /user/notifications/[id]`.
    pub async fn patch_update_notifications(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/user/notifications/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /user/profile-review`.
    pub async fn profile_review(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/profile-review", query).await
    }

    /// Generated `POST /user/profile-review/confirm`.
    pub async fn profile_review_confirm(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/profile-review/confirm", body).await
    }

    /// Generated `DELETE /user/push/device-token`.
    pub async fn delete_push_device_token(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/user/push/device-token", query, body).await
    }

    /// Generated `POST /user/push/device-token`.
    pub async fn push_device_token(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/push/device-token", body).await
    }

    /// Generated `GET /user/push/preferences`.
    pub async fn push_preferences(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/push/preferences", query).await
    }

    /// Generated `PATCH /user/push/preferences`.
    pub async fn update_push_preferences(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.patch("/user/push/preferences", body).await
    }

    /// Generated `GET /user/push/public-key`.
    pub async fn push_public_key(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/push/public-key", query).await
    }

    /// Generated `DELETE /user/push/subscribe`.
    pub async fn delete_push_subscribe(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/user/push/subscribe", query, body).await
    }

    /// Generated `POST /user/push/subscribe`.
    pub async fn push_subscribe(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/push/subscribe", body).await
    }

    /// Generated `PATCH /user/settings`.
    pub async fn update_settings(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.patch("/user/settings", body).await
    }

    /// Generated `DELETE /user/sso`.
    pub async fn delete_sso(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/user/sso", query, body).await
    }

    /// Generated `GET /user/sso`.
    pub async fn sso(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/sso", query).await
    }

    /// Generated `PATCH /user/sso`.
    pub async fn update_sso(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.patch("/user/sso", body).await
    }

    /// Generated `POST /user/sso`.
    pub async fn post_sso(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/sso", body).await
    }

    /// Generated `POST /user/subscriptions`.
    pub async fn subscriptions(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/user/subscriptions", body).await
    }

    /// Generated `GET /user/support-phone`.
    pub async fn support_phone(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/user/support-phone", query).await
    }
    // </generated-openapi>
}
