use std::collections::HashMap;

use serde_json::Value;

use crate::error::IgniteError;
use crate::models::{AddDomainRequest, DnsRecordListResponse, DomainListResponse, SuccessResponse};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct DomainsResource {
    transport: HttpTransport,
}

impl DomainsResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<DomainListResponse, IgniteError> {
        self.transport.get("/domains", None).await
    }

    pub async fn add(&self, domain: &str) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post(
                "/domains/add",
                Some(serde_json::to_value(AddDomainRequest {
                    domain: domain.to_string(),
                })?),
            )
            .await
    }

    pub async fn records(&self, domain_id: &str) -> Result<DnsRecordListResponse, IgniteError> {
        self.transport
            .get(&format!("/domains/{domain_id}/records"), None)
            .await
    }

    pub async fn create_record(
        &self,
        domain_id: &str,
        body: HashMap<String, Value>,
    ) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .post(
                &format!("/domains/{domain_id}/records"),
                Some(Value::Object(body.into_iter().collect())),
            )
            .await
    }

    pub async fn delete_record(
        &self,
        domain_id: &str,
        record_id: &str,
    ) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .delete(
                &format!("/domains/{domain_id}/records/{record_id}"),
                None,
                None,
            )
            .await
    }

    // <generated-openapi>
    /// Generated `DELETE /domains/[domainId]/delete`.
    pub async fn delete_delete(
        &self,
        domain_id: &str,
        query: Option<HashMap<String, String>>,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/domains/{}/delete", encode_path(domain_id));
        self.transport.delete(&path, query, body).await
    }

    /// Generated `GET /domains/[domainId]/export`.
    pub async fn export(
        &self,
        domain_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/domains/{}/export", encode_path(domain_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /domains/[domainId]/import/text`.
    pub async fn import_text(
        &self,
        domain_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/domains/{}/import/text", encode_path(domain_id));
        self.transport.post(&path, body).await
    }

    /// Generated `PATCH /domains/[domainId]/records/[recordId]`.
    pub async fn update_records(
        &self,
        domain_id: &str,
        record_id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/domains/{}/records/{}", encode_path(domain_id), encode_path(record_id));
        self.transport.patch(&path, body).await
    }

    /// Generated `GET /domains/nameservers`.
    pub async fn nameservers(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/domains/nameservers", query).await
    }

    /// Generated `POST /domains/order`.
    pub async fn order(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/domains/order", body).await
    }

    /// Generated `PATCH /domains/order/[id]`.
    pub async fn update_order(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/domains/order/{}", encode_path(id));
        self.transport.patch(&path, body).await
    }

    /// Generated `POST /domains/quote`.
    pub async fn quote(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/domains/quote", body).await
    }

    /// Generated `GET /domains/registrations`.
    pub async fn registrations(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/domains/registrations", query).await
    }

    /// Generated `GET /domains/registrations/[id]`.
    pub async fn get_registrations(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/domains/registrations/{}", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /domains/registrations/[id]/actions`.
    pub async fn registrations_actions(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/domains/registrations/{}/actions", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /domains/registrations/by-service/[serviceId]`.
    pub async fn registrations_by_service(
        &self,
        service_id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/domains/registrations/by-service/{}", encode_path(service_id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /domains/suggest`.
    pub async fn suggest(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/domains/suggest", body).await
    }

    /// Generated `POST /domains/tld-prices`.
    pub async fn tld_prices(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/domains/tld-prices", body).await
    }

    /// Generated `POST /domains/transfer-quote`.
    pub async fn transfer_quote(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/domains/transfer-quote", body).await
    }

    /// Generated `GET /domains/whois`.
    pub async fn whois(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/domains/whois", query).await
    }
    // </generated-openapi>
}
