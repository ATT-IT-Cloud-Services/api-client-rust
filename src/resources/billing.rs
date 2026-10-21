use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{
    AddPaymentMethodResponse, InvoiceDetailResponse, InvoiceListFilters, InvoiceListResponse,
    InvoicePayBody, InvoicePayResult, NetPointsPreviewRequest, NetPointsPreviewResponse,
    PaymentMethodsResult, SetDefaultPaymentMethodRequest, SubscriptionsResponse, SuccessResponse,
    UpdatePaymentMethodLabelRequest,
};
use crate::transport::{encode_path, HttpTransport};

#[derive(Clone)]
pub struct BillingResource {
    transport: HttpTransport,
}

impl BillingResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn invoices(
        &self,
        filters: InvoiceListFilters,
    ) -> Result<InvoiceListResponse, IgniteError> {
        let mut pairs = vec![("limit", json!(filters.limit.unwrap_or(50)))];
        if let Some(offset) = filters.offset {
            pairs.push(("offset", json!(offset)));
        }
        if let Some(status) = filters.status {
            pairs.push(("status", json!(status)));
        }
        if let Some(service_id) = filters.service_id {
            pairs.push(("serviceId", json!(service_id)));
        }
        let query = query_params(&pairs);
        self.transport.get("/invoices", Some(query)).await
    }

    pub async fn invoices_with_limit(
        &self,
        limit: u64,
    ) -> Result<InvoiceListResponse, IgniteError> {
        self.invoices(InvoiceListFilters {
            limit: Some(limit),
            ..Default::default()
        })
        .await
    }

    pub async fn invoice(&self, id: &str) -> Result<InvoiceDetailResponse, IgniteError> {
        self.transport.get(&format!("/invoices/{id}"), None).await
    }

    pub async fn pay_invoice(
        &self,
        id: &str,
        body: InvoicePayBody,
    ) -> Result<InvoicePayResult, IgniteError> {
        self.transport
            .post(
                &format!("/invoices/{id}/pay"),
                Some(serde_json::to_value(body)?),
            )
            .await
    }

    pub async fn net_points_preview(
        &self,
        order_total: f64,
    ) -> Result<NetPointsPreviewResponse, IgniteError> {
        self.transport
            .post(
                "/netpoints/preview",
                Some(serde_json::to_value(NetPointsPreviewRequest {
                    order_total,
                })?),
            )
            .await
    }

    pub async fn subscriptions(&self) -> Result<SubscriptionsResponse, IgniteError> {
        self.transport.get("/user/subscriptions", None).await
    }

    pub async fn payment_methods(&self) -> Result<PaymentMethodsResult, IgniteError> {
        self.transport.get("/account/payment-methods", None).await
    }

    pub async fn add_payment_method(&self) -> Result<AddPaymentMethodResponse, IgniteError> {
        self.transport
            .post("/account/payment-methods", Some(json!({})))
            .await
    }

    pub async fn set_default_payment_method(
        &self,
        mandate_id: &str,
    ) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .patch(
                "/account/payment-methods",
                Some(serde_json::to_value(SetDefaultPaymentMethodRequest {
                    mandate_id: mandate_id.to_string(),
                })?),
            )
            .await
    }

    pub async fn update_payment_method_label(
        &self,
        mandate_id: &str,
        user_label: &str,
        set_default: bool,
    ) -> Result<SuccessResponse, IgniteError> {
        self.transport
            .patch(
                "/account/payment-methods",
                Some(serde_json::to_value(UpdatePaymentMethodLabelRequest {
                    mandate_id: mandate_id.to_string(),
                    user_label: user_label.to_string(),
                    set_default: Some(set_default),
                })?),
            )
            .await
    }

    pub async fn delete_payment_method(
        &self,
        mandate_id: &str,
    ) -> Result<SuccessResponse, IgniteError> {
        let path = format!("/account/payment-methods/{}", encode(mandate_id));
        self.transport.delete(&path, None, None).await
    }

    // <generated-openapi>
    /// Generated `GET /account/company-transfer`.
    pub async fn company_transfer(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/account/company-transfer", query).await
    }

    /// Generated `POST /account/company-transfer`.
    pub async fn post_company_transfer(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/account/company-transfer", body).await
    }

    /// Generated `GET /account/contract-partner`.
    pub async fn contract_partner(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/account/contract-partner", query).await
    }

    /// Generated `GET /account/legal-documents`.
    pub async fn legal_documents(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/account/legal-documents", query).await
    }

    /// Generated `GET /account/tenant-transfer`.
    pub async fn tenant_transfer(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/account/tenant-transfer", query).await
    }

    /// Generated `POST /account/tenant-transfer`.
    pub async fn post_tenant_transfer(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/account/tenant-transfer", body).await
    }

    /// Generated `POST /invoices`.
    pub async fn create(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/invoices", body).await
    }

    /// Generated `GET /invoices/[id]/credit-note-document`.
    pub async fn credit_note_document(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/invoices/{}/credit-note-document", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /invoices/[id]/dispute`.
    pub async fn dispute(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/invoices/{}/dispute", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /invoices/[id]/document`.
    pub async fn document(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/invoices/{}/document", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /invoices/[id]/documents`.
    pub async fn documents(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/invoices/{}/documents", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `GET /invoices/[id]/dunning-document`.
    pub async fn dunning_document(
        &self,
        id: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/invoices/{}/dunning-document", encode_path(id));
        self.transport.get(&path, query).await
    }

    /// Generated `POST /invoices/[id]/pay/cancel`.
    pub async fn pay_cancel(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/invoices/{}/pay/cancel", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `POST /invoices/[id]/withdraw`.
    pub async fn withdraw(
        &self,
        id: &str,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        let path = format!("/invoices/{}/withdraw", encode_path(id));
        self.transport.post(&path, body).await
    }

    /// Generated `GET /invoices/download-all`.
    pub async fn download_all(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/invoices/download-all", query).await
    }

    /// Generated `GET /netpoints`.
    pub async fn list(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/netpoints", query).await
    }
    // </generated-openapi>
}

fn encode(value: &str) -> String {
    value
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}
