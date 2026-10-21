use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{CountriesResponse, ProductsResponse};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct ShopResource {
    transport: HttpTransport,
}

impl ShopResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn products(&self, lang: Option<&str>) -> Result<ProductsResponse, IgniteError> {
        let query = lang.map(|l| query_params(&[("lang", json!(l))]));
        self.transport.get("/products", query).await
    }

    pub async fn countries(&self) -> Result<CountriesResponse, IgniteError> {
        self.transport.get("/public/countries", None).await
    }

    // <generated-openapi>
    /// Generated `GET /categories/[id]/upgrade-config`.
    pub async fn upgrade_config(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/categories/{}/upgrade-config", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /products/[category]`.
    pub async fn find(
        &self,
        category: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/products/{}", encode_path(category));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /products/[category]/[product]`.
    pub async fn get_find(
        &self,
        category: &str,
        product: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/products/{}/{}", encode_path(category), encode_path(product));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /products/[category]/[product]/view`.
    pub async fn view(
        &self,
        category: &str,
        product: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/products/{}/{}/view", encode_path(category), encode_path(product));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /products/[category]/view`.
    pub async fn get_view(
        &self,
        category: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/products/{}/view", encode_path(category));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /products/by-slug/[slug]`.
    pub async fn by_slug(
        &self,
        slug: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/products/by-slug/{}", encode_path(slug));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /products/locations`.
    pub async fn locations(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/products/locations", query).await
    }

    /// Generated `POST /products/proxmox-slot-preview`.
    pub async fn proxmox_slot_preview(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/products/proxmox-slot-preview", body).await
    }

    /// Generated `GET /products/templates`.
    pub async fn templates(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/products/templates", query).await
    }
    // </generated-openapi>
}
