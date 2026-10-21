//! Sandbox mode example for ignite-api-client
//!
//! Run: cargo run --example sandbox

use ignite_api_client::{ApiError, IgniteClient, IgniteClientConfig, IgniteError};

#[tokio::main]
async fn main() {
    let base_url =
        std::env::var("IGNITE_API_URL").unwrap_or_else(|_| "https://api.example.test".to_string());

    // Sandbox mode needs no API key or session token.
    let client =
        IgniteClient::new(IgniteClientConfig::new(base_url.clone()).sandbox(true)).expect("client");

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

    // Force a specific sandbox error to test error handling.
    let failing = IgniteClient::new(
        IgniteClientConfig::new(base_url)
            .sandbox(true)
            .sandbox_error("rate_limited")
            .sandbox_status(429),
    )
    .expect("client");

    match failing.services.list(20, None).await {
        Ok(_) => {
            eprintln!("Expected a sandbox error, but the request succeeded.");
            std::process::exit(1);
        }
        Err(IgniteError::Api(err)) => {
            println!("Forced sandbox error: {} {}", err.code, err.status);
        }
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}

async fn run(client: &IgniteClient) -> Result<(), IgniteError> {
    let list = client.services.list(20, None).await?;
    let items = list.services.or(list.servers).unwrap_or_default();
    println!("Sandbox services: {}", items.len());

    Ok(())
}

#[allow(dead_code)]
fn _api_error_type(err: ApiError) -> ApiError {
    err
}
