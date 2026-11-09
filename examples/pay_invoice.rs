//! Pay invoice example for ignite-api-client
//!
//! Run: cargo run --example pay_invoice

use ignite_api_client::{
    ApiError, ClientKind, IgniteClient, IgniteClientConfig, IgniteError, InvoicePayBody,
    InvoicePaymentMethod,
};

#[tokio::main]
async fn main() {
    let base_url =
        std::env::var("IGNITE_API_URL").unwrap_or_else(|_| "https://api.example.test".to_string());

    let token = std::env::var("IGNITE_TOKEN").ok();
    let invoice_id = std::env::var("IGNITE_INVOICE_ID").ok();

    if token.is_none() || invoice_id.is_none() {
        eprintln!("Set IGNITE_TOKEN and IGNITE_INVOICE_ID environment variables.");
        std::process::exit(1);
    }

    let client = IgniteClient::new(
        IgniteClientConfig::new(base_url)
            .token(token.unwrap())
            .client_kind(ClientKind::Desktop),
    )
    .expect("client");

    let invoice_id = std::env::var("IGNITE_INVOICE_ID").unwrap();

    match run(&client, &invoice_id).await {
        Ok(()) => {}
        Err(IgniteError::Api(err)) => {
            eprintln!("API error: {} {}", err.code, err.status);
            if err.code == "insufficientBalance" {
                eprintln!("Top up wallet balance or choose a different payment method.");
            }
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}

async fn run(client: &IgniteClient, invoice_id: &str) -> Result<(), IgniteError> {
    let detail = client.billing.invoice(invoice_id).await?;
    // Identifiers, amounts and payment URLs stay out of stdout: the invoice id is
    // personal data, and a checkout URL / payment id is a bearer secret that an
    // example run would otherwise leave in terminal scrollback and CI logs.
    println!("Invoice status: {}", detail.invoice.status);

    let methods = client.billing.payment_methods().await?;
    println!("Saved payment methods: {}", methods.methods.len());

    let payment_method = if methods.methods.is_empty() {
        InvoicePaymentMethod::Mollie
    } else {
        InvoicePaymentMethod::MollieSaved
    };

    let result = client
        .billing
        .pay_invoice(
            invoice_id,
            InvoicePayBody {
                payment_method,
                saved_mandate_id: methods.default_mandate_id,
                amount_from_balance: None,
                net_points_redeem_eur: None,
            },
        )
        .await?;

    if result.checkout_url.is_some() {
        println!("Checkout URL received — open it in a browser (not printed here).");
    } else if result.already_paid.unwrap_or(false) {
        println!("Invoice already paid.");
    } else {
        println!("Payment initiated.");
    }

    Ok(())
}

#[allow(dead_code)]
fn _api_error_type(err: ApiError) -> ApiError {
    err
}
