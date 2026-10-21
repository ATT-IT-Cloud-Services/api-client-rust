use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct ThemeAssetsResource {
    transport: HttpTransport,
}

impl ThemeAssetsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `GET /theme-assets/[slug]/[...file]`.
    pub async fn find(
        &self,
        slug: &str,
        file: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/theme-assets/{}/{}", encode_path(slug), file.trim_start_matches('/'));
        self.transport.get(&path, query).await
    }
    // </generated-openapi>

}
