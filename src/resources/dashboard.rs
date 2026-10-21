use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::query_params;
use crate::error::IgniteError;
use crate::models::{
    DashboardTicketsResponse, MaintenanceNotificationsResponse, MonthlyOffersResponse,
    ServiceListResponse, TotalBandwidthResponse, TrafficSourcesResponse,
};
use crate::transport::HttpTransport;

#[derive(Clone)]
pub struct DashboardResource {
    transport: HttpTransport,
}

impl DashboardResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn services(&self, limit: u64) -> Result<ServiceListResponse, IgniteError> {
        let query = query_params(&[("limit", json!(limit))]);
        self.transport.get("/services", Some(query)).await
    }

    pub async fn tickets(
        &self,
        status: &str,
        limit: u64,
    ) -> Result<DashboardTicketsResponse, IgniteError> {
        let query = query_params(&[("status", json!(status)), ("limit", json!(limit))]);
        self.transport.get("/tickets", Some(query)).await
    }

    pub async fn total_bandwidth(&self) -> Result<TotalBandwidthResponse, IgniteError> {
        self.transport.get("/services/total-bandwidth", None).await
    }

    pub async fn maintenance_notifications(
        &self,
    ) -> Result<MaintenanceNotificationsResponse, IgniteError> {
        self.transport
            .get("/services/maintenance-notifications", None)
            .await
    }

    pub async fn traffic_sources(&self) -> Result<TrafficSourcesResponse, IgniteError> {
        self.transport.get("/dashboard/traffic-sources", None).await
    }

    pub async fn monthly_offers(&self) -> Result<MonthlyOffersResponse, IgniteError> {
        self.transport.get("/user/monthly-offers", None).await
    }

    // <generated-openapi>
    /// Generated `GET /dashboard/contract-payment-alert`.
    pub async fn contract_payment_alert(
        &self,
        query: Option<HashMap<String, String>>,
    ) -> Result<Value, IgniteError> {
        self.transport.get("/dashboard/contract-payment-alert", query).await
    }
    // </generated-openapi>
}
