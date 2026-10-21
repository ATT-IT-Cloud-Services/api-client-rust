//! Services example for ignite-api-client
//!
//! Run: cargo run --example services

use ignite_api_client::{ApiError, ClientKind, IgniteClient, IgniteClientConfig, IgniteError};

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
            .client_kind(ClientKind::Api),
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
    let list = client.services.list(20, None).await?;
    let items = list.services.or(list.servers).unwrap_or_default();
    println!("Services: {}", items.len());

    if let Some(first) = items.first() {
        let detail = client.services.find(&first.id).await?;
        println!(
            "First service: {:?}",
            detail.server.as_ref().map(|s| &s.name)
        );

        let ids: Vec<String> = items.iter().take(5).map(|s| s.id.clone()).collect();
        let statuses = client.services.status_batch(ids).await?;
        println!("Status batch keys: {}", statuses.statuses.len());
    }

    Ok(())
}

#[allow(dead_code)]
fn _api_error_type(err: ApiError) -> ApiError {
    err
}
