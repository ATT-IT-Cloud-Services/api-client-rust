use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::Country;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutAddress {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub street: String,
    pub zip: String,
    pub city: String,
    pub country_code: String,
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetPointsBalance {
    pub points: f64,
    pub eur_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessBillingConfig {
    pub invoice_enabled: bool,
    pub has_active_fund: bool,
    pub anchor_day: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyBillingConfig {
    pub group_id: String,
    pub wallet_balance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_payment_approval: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_balance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_billing_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_locked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedPaymentMethodsSummary {
    pub has_valid: bool,
    pub default_label: Option<String>,
    pub default_mandate_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutBootstrap {
    pub success: bool,
    pub addresses: Vec<CheckoutAddress>,
    pub countries: Vec<Country>,
    pub net_points_balance: Option<NetPointsBalance>,
    pub business_billing_config: Option<BusinessBillingConfig>,
    pub family_billing_config: Option<FamilyBillingConfig>,
    pub saved_payment_methods: Option<SavedPaymentMethodsSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutCartLineCustomization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcores: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutCartLine {
    pub line_id: String,
    pub product_id: String,
    pub product_name: String,
    pub quantity: u64,
    pub billing_cycle: u64,
    pub item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization: Option<CheckoutCartLineCustomization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CartUnavailableItem {
    pub line_id: String,
    pub product_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CartValidateResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable: Option<Vec<CartUnavailableItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculatePayload {
    pub items: Vec<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_vat_id: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CartCalculateItem {
    pub product_id: String,
    pub product_name: String,
    pub total: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CartCalculateResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CartCalculateItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_month_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vat_free: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutPaymentMethod {
    Mollie,
    #[serde(rename = "mollie_saved")]
    MollieSaved,
    Sepa,
    Guthaben,
    Invoice,
    Businessfund,
    #[serde(rename = "family_wallet")]
    FamilyWallet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutBillingAddressInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub street: String,
    pub zip: String,
    pub city: String,
    pub country_code: String,
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutSubmitPayload {
    pub items: Vec<HashMap<String, Value>>,
    pub payment_method: CheckoutPaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_mandate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_points_redeem_eur: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CheckoutBillingAddressInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newsletter_opt_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SepaDetails {
    pub iban: String,
    pub bic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    pub amount: f64,
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutSubmitResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_details: Option<SepaDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartValidateRequest {
    pub items: Vec<HashMap<String, Value>>,
}
