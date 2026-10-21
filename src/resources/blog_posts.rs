use serde_json::Value;

use crate::error::IgniteError;
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct BlogPostsResource {
    transport: HttpTransport,
}

impl BlogPostsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    // <generated-openapi>
    /// Generated `POST /blog-posts/[slug]/vote`.
    pub async fn vote(
        &self,
        slug: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/blog-posts/{}/vote", encode_path(slug));
        self.transport.post(&path, body).await
    }
    // </generated-openapi>

}
