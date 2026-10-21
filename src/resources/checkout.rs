use crate::error::IgniteError;
use crate::models::{
    CalculatePayload, CartCalculateResponse, CartValidateRequest, CartValidateResponse,
    CheckoutBootstrap, CheckoutSubmitPayload, CheckoutSubmitResponse, CountriesResponse,
};
use crate::transport::HttpTransport;
use serde_json::Value;

#[derive(Clone)]
pub struct CheckoutResource {
    transport: HttpTransport,
}

impl CheckoutResource {
    pub(crate) fn new(transport: HttpTransport) -> Self {
        Self { transport }
    }

    pub async fn bootstrap(&self) -> Result<CheckoutBootstrap, IgniteError> {
        self.transport.get("/checkout/bootstrap", None).await
    }

    pub async fn countries(&self) -> Result<CountriesResponse, IgniteError> {
        self.transport.get("/public/countries", None).await
    }

    pub async fn validate(
        &self,
        request: CartValidateRequest,
    ) -> Result<CartValidateResponse, IgniteError> {
        self.transport
            .post("/cart/validate", Some(serde_json::to_value(request)?))
            .await
    }

    pub async fn calculate(
        &self,
        payload: CalculatePayload,
    ) -> Result<CartCalculateResponse, IgniteError> {
        self.transport
            .post("/cart/calculate", Some(serde_json::to_value(payload)?))
            .await
    }

    pub async fn submit(
        &self,
        payload: CheckoutSubmitPayload,
    ) -> Result<CheckoutSubmitResponse, IgniteError> {
        self.transport
            .post("/checkout", Some(serde_json::to_value(payload)?))
            .await
    }

    // <generated-openapi>
    /// Generated `POST /cart/upgrade`.
    pub async fn upgrade(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/cart/upgrade", body).await
    }

    /// Generated `POST /checkout/offers/validate`.
    pub async fn offers_validate(
        &self,
        body: Option<Value>,
    ) -> Result<Value, IgniteError> {
        self.transport.post("/checkout/offers/validate", body).await
    }
    // </generated-openapi>
}
