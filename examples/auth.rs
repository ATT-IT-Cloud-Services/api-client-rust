//! Authentication example for ignite-api-client
//!
//! Run: cargo run --example auth

use ignite_api_client::{
    ApiError, ClientKind, IgniteClient, IgniteClientConfig, IgniteError, LoginRequest,
};

#[tokio::main]
async fn main() {
    let base_url =
        std::env::var("IGNITE_API_URL").unwrap_or_else(|_| "https://api.example.test".to_string());

    let client = IgniteClient::new(IgniteClientConfig::new(base_url).client_kind(ClientKind::Api))
        .expect("client");

    let email = match std::env::var("IGNITE_EMAIL") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Set IGNITE_EMAIL and IGNITE_PASSWORD environment variables.");
            std::process::exit(1);
        }
    };
    let password = std::env::var("IGNITE_PASSWORD").expect("IGNITE_PASSWORD");

    match run(&client, &email, &password).await {
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

async fn run(client: &IgniteClient, email: &str, password: &str) -> Result<(), IgniteError> {
    let login = client
        .auth
        .login(LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
            two_factor_code: None,
            remember_me: true,
        })
        .await?;

    if login.requires_two_factor.unwrap_or(false) {
        println!("Two-factor authentication required.");
        return Ok(());
    }

    if let Some(token) = login.token {
        client.set_token(Some(token));
    }

    let me = client.auth.me().await?;
    println!("Authenticated user: {} {}", me.user.id, me.user.email);

    let sessions = client.auth.sessions().await?;
    println!("Active sessions: {}", sessions.sessions.len());

    Ok(())
}

#[allow(dead_code)]
fn _api_error_type(err: ApiError) -> ApiError {
    err
}
