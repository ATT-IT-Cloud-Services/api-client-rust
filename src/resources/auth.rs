use serde_json::Value;
use std::collections::HashMap;

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{
    ChangePasswordRequest, Disable2faRequest, Disable2faResponse, Enable2faRequest, LoginRequest,
    LoginResponse, MeResponse, SessionsResponse, SuccessResponse, TwoFactorEnableResponse,
    TwoFactorSetupResponse, UpdateProfileRequest,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct AuthResource {
    transport: HttpTransport,
}

impl AuthResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn login(&self, data: LoginRequest) -> Result<LoginResponse, IgniteError> {
        self.transport
            .post("/auth/login", Some(serde_json::to_value(data)?))
            .await
    }

    pub async fn me(&self) -> Result<MeResponse, IgniteError> {
        self.transport.get("/auth/me", None).await
    }

    pub async fn update_profile(
        &self,
        data: UpdateProfileRequest,
    ) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .put("/auth/me", Some(serde_json::to_value(data)?))
            .await
    }

    pub async fn logout(&self) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post("/auth/logout", Some(Value::Object(Default::default())))
            .await
    }

    pub async fn logout_all(&self) -> Result<SuccessResponse, IgniteError> {
        self.transport.delete("/auth/logout", None, None).await
    }

    pub async fn change_password(
        &self,
        data: ChangePasswordRequest,
    ) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post("/auth/change-password", Some(serde_json::to_value(data)?))
            .await
    }

    pub async fn sessions(&self) -> Result<SessionsResponse, IgniteError> {
        self.transport.get("/auth/sessions", None).await
    }

    pub async fn revoke_session(&self, id: &str) -> Result<SuccessResponse, IgniteError> {
        let query = query_params(&[("id", Value::String(id.to_string()))]);
        self.transport
            .delete("/auth/sessions", Some(query), None)
            .await
    }

    pub async fn setup_2fa(&self) -> Result<TwoFactorSetupResponse, IgniteError> {
        self.transport.get("/auth/2fa/setup", None).await
    }

    pub async fn enable_2fa(&self, code: &str) -> Result<TwoFactorEnableResponse, IgniteError> {
        self.transport
            .post(
                "/auth/2fa/setup",
                Some(serde_json::to_value(Enable2faRequest {
                    code: code.to_string(),
                })?),
            )
            .await
    }

    pub async fn disable_2fa(
        &self,
        code: &str,
        password: &str,
    ) -> Result<Disable2faResponse, IgniteError> {
        self.transport
            .delete(
                "/auth/2fa/setup",
                None,
                Some(serde_json::to_value(Disable2faRequest {
                    code: code.to_string(),
                    password: password.to_string(),
                })?),
            )
            .await
    }

    // <generated-openapi>
    /// Generated `GET /auth/backup-codes`.
    pub async fn backup_codes(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/auth/backup-codes", query).await
    }

    /// Generated `POST /auth/backup-codes`.
    pub async fn post_backup_codes(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/backup-codes", body).await
    }

    /// Generated `POST /auth/blocklist-appeal`.
    pub async fn blocklist_appeal(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/blocklist-appeal", body).await
    }

    /// Generated `POST /auth/code`.
    pub async fn code(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/code", body).await
    }

    /// Generated `POST /auth/forgot-password`.
    pub async fn forgot_password(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/forgot-password", body).await
    }

    /// Generated `POST /auth/login/begin`.
    pub async fn login_begin(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/login/begin", body).await
    }

    /// Generated `PATCH /auth/me`.
    pub async fn update_me(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.patch("/auth/me", body).await
    }

    /// Generated `GET /auth/oauth/[provider]`.
    pub async fn oauth(
        &self,
        provider: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/auth/oauth/{}", encode_path(provider));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /auth/oauth/[provider]/callback`.
    pub async fn oauth_callback(
        &self,
        provider: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/auth/oauth/{}/callback", encode_path(provider));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /auth/oauth/2fa-complete`.
    pub async fn oauth_two_fa_complete(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/oauth/2fa-complete", body).await
    }

    /// Generated `DELETE /auth/oauth/link/[provider]`.
    pub async fn delete_oauth_link(
        &self,
        provider: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/auth/oauth/link/{}", encode_path(provider));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /auth/oauth/link/[provider]`.
    pub async fn oauth_link(
        &self,
        provider: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/auth/oauth/link/{}", encode_path(provider));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /auth/oauth/providers`.
    pub async fn oauth_providers(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/auth/oauth/providers", query).await
    }

    /// Generated `DELETE /auth/passkeys`.
    pub async fn delete_passkeys(
        &self,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.delete("/auth/passkeys", query, body).await
    }

    /// Generated `GET /auth/passkeys`.
    pub async fn passkeys(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/auth/passkeys", query).await
    }

    /// Generated `POST /auth/passkeys/authenticate`.
    pub async fn passkeys_authenticate(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/passkeys/authenticate", body).await
    }

    /// Generated `GET /auth/passkeys/register`.
    pub async fn passkeys_register(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/auth/passkeys/register", query).await
    }

    /// Generated `POST /auth/passkeys/register`.
    pub async fn post_passkeys_register(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/passkeys/register", body).await
    }

    /// Generated `POST /auth/register`.
    pub async fn register(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/register", body).await
    }

    /// Generated `GET /auth/reset-password`.
    pub async fn reset_password(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/auth/reset-password", query).await
    }

    /// Generated `POST /auth/reset-password`.
    pub async fn post_reset_password(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/reset-password", body).await
    }

    /// Generated `GET /auth/session`.
    pub async fn session(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/auth/session", query).await
    }

    /// Generated `GET /auth/sso/callback`.
    pub async fn sso_callback(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/auth/sso/callback", query).await
    }

    /// Generated `POST /auth/sso/link`.
    pub async fn sso_link(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/sso/link", body).await
    }

    /// Generated `GET /auth/sso/start`.
    pub async fn sso_start(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/auth/sso/start", query).await
    }

    /// Generated `POST /auth/verify-account`.
    pub async fn verify_account(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/verify-account", body).await
    }

    /// Generated `POST /auth/verify-account/resend`.
    pub async fn verify_account_resend(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/verify-account/resend", body).await
    }

    /// Generated `POST /auth/verify-code`.
    pub async fn verify_code(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/verify-code", body).await
    }

    /// Generated `GET /auth/verify-email`.
    pub async fn verify_email(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/auth/verify-email", query).await
    }

    /// Generated `POST /auth/verify-email`.
    pub async fn post_verify_email(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/auth/verify-email", body).await
    }
    // </generated-openapi>
}
