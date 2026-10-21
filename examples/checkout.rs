//! Checkout example for ignite-api-client
//!
//! Run: cargo run --example checkout

use std::collections::HashMap;

use ignite_api_client::{
    ApiError, CalculatePayload, CartValidateRequest, ClientKind, IgniteClient, IgniteClientConfig,
    IgniteError,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let base_url =
        std::env::var("IGNITE_API_URL").unwrap_or_else(|_| "https://api.example.test".to_string());

    let token = match std::env::var("IGNITE_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Set IGNITE_TOKEN (session token or API key).");
            std::process::exit(1);
        }
    };

    let client = IgniteClient::new(
        IgniteClientConfig::new(base_url)
            .token(token)
            .client_kind(ClientKind::Desktop),
    )
    .expect("client");

    match run(&client).await {
        Ok(()) => {}
        Err(IgniteError::Api(err)) => {
            eprintln!("API error: {} {}", err.code, err.status);
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}

async fn run(client: &IgniteClient) -> Result<(), IgniteError> {
    let bootstrap = client.checkout.bootstrap().await?;
    println!("Addresses: {}", bootstrap.addresses.len());
    println!("Countries: {}", bootstrap.countries.len());

    let catalog = client.shop.products(Some("en")).await?;
    println!(
        "Product categories: {}",
        catalog.categories.as_ref().map(|c| c.len()).unwrap_or(0)
    );

    let product_id =
        std::env::var("IGNITE_PRODUCT_ID").unwrap_or_else(|_| "example-product-id".to_string());

    let mut item = HashMap::new();
    item.insert("lineId".to_string(), Value::String("line-1".to_string()));
    item.insert("productId".to_string(), Value::String(product_id));
    item.insert(
        "productName".to_string(),
        Value::String("Example Product".to_string()),
    );
    item.insert("quantity".to_string(), json!(1));
    item.insert("billingCycle".to_string(), json!(30));
    item.insert("itemType".to_string(), Value::String("new".to_string()));

    let validation = client
        .checkout
        .validate(CartValidateRequest {
            items: vec![item.clone()],
        })
        .await?;
    println!(
        "Cart valid: {} {} unavailable",
        validation.success,
        validation
            .unavailable
            .as_ref()
            .map(|u| u.len())
            .unwrap_or(0)
    );

    let country_code = bootstrap
        .addresses
        .first()
        .map(|a| a.country_code.clone())
        .unwrap_or_else(|| "DE".to_string());

    let calculation = client
        .checkout
        .calculate(CalculatePayload {
            items: vec![item],
            country_code: Some(country_code),
            has_vat_id: None,
            vat_number: None,
            lang: None,
        })
        .await?;
    println!("Calculated total: {:?}", calculation.total);

    Ok(())
}

#[allow(dead_code)]
fn _api_error_type(err: ApiError) -> ApiError {
    err
}
