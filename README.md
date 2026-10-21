# ignite-api-client

Official ignite API client (Rust).

Machine-facing SDK — no localized error messages. API failures return `ApiError` with a machine-readable `code`, HTTP `status`, and raw `payload`. Map `code` to your own i18n layer.

## Requirements

- Rust 1.75+
- Tokio runtime for async examples

## Add dependency

From the monorepo:

```toml
[dependencies]
ignite-api-client = { git = "https://git.zyneonstudios.com/elizon/api-client-rust.git" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

From the monorepo:

```toml
[dependencies]
ignite-api-client = { path = "../public-api/rust" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick start

```rust
use ignite_api_client::{
    ApiError, ClientKind, IgniteClient, IgniteClientConfig, IgniteError, LoginRequest,
};

#[tokio::main]
async fn main() -> Result<(), IgniteError> {
    let client = IgniteClient::new(
        IgniteClientConfig::new("https://api.example.test").client_kind(ClientKind::Api),
    )?;

    let login = client.auth.login(LoginRequest {
        email: "you@example.com".into(),
        password: "secret".into(),
        two_factor_code: None,
        remember_me: true,
    }).await?;

    if let Some(token) = login.token {
        client.set_token(Some(token));
    }

    let services = client.services.list(50, None).await?;
    println!("Services: {:?}", services.services);

    Ok(())
}
```

## Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `base_url` | `String` | required | ignite API base URL |
| `token` | `Option<String>` | `None` | Bearer session token or API key |
| `client_kind` | `ClientKind` | `Api` | Sent as `X-Ignite-Client` header |
| `headers` | `HashMap<String, String>` | empty | Extra headers on every request |
| `sandbox` | `bool` | `false` | Enable sandbox mode (sends `x-sandbox: true`) |
| `sandbox_error` | `Option<String>` | `None` | Force a sandbox error code (`x-sandbox-error`) |
| `sandbox_status` | `Option<u16>` | `None` | HTTP status for a forced sandbox error (`x-sandbox-status`, default 400) |

### Client kind

Some financially sensitive endpoints (checkout, invoice payment, business fund) require `ClientKind::Desktop`. The server rejects `Mobile` clients for these actions.

### API keys

Create an API key via `client.user.create_api_key("name")`, then pass the secret as `token`:

```rust
let client = IgniteClient::new(
    IgniteClientConfig::new("https://api.example.test")
        .token(std::env::var("IGNITE_API_KEY").ok()),
)?;
```

### Sandbox mode

For integration testing, enable `sandbox` to hit the API without a valid API key or session. The API returns randomly generated placeholder data and performs no real reads, writes, or provider calls. Force an error response with `sandbox_error` (and optionally `sandbox_status`) to exercise your error handling:

```rust
let client = IgniteClient::new(
    IgniteClientConfig::new("https://api.example.test").sandbox(true),
)?;

let failing = IgniteClient::new(
    IgniteClientConfig::new("https://api.example.test")
        .sandbox(true)
        .sandbox_error("rate_limited")
        .sandbox_status(429),
)?;
```

Paths are host-root on `https://api.example.test` (e.g. `/services`). Legacy `/api/*` redirects with HTTP 308; this client does not use the prefix.

## Resources

Typed helpers remain on the original namespaces. Generated methods cover every public OpenAPI operation and return `serde_json::Value`.

| Namespace | Description |
|-----------|-------------|
| `auth` | Login, sessions, 2FA, OAuth, passkeys |
| `services` | VMs, containers, provider actions |
| `billing` | Invoices, subscriptions, payment methods |
| `checkout` | Cart validation, calculation, submit |
| `shop` | Product catalog |
| `dashboard` | Overview aggregates |
| `domains` | Domain list, DNS records |
| `subdomains` | Subdomain management |
| `ip_manager` | IP manager entries |
| `ssh_keys` | SSH key management |
| `support` | Tickets, knowledge base, feedback |
| `user` | Audit log, API keys, support PIN |
| `wallet` | Auto top-up, vouchers |
| `business` | Business fund, billing |
| `affiliates` | Affiliate profile, commissions |
| `family` | Family groups, invites |
| `byoip` | BYOIP networks |
| `floating_ips` | Floating IP management |
| `team` | Teams, members, groups |
| `container` | Container projects and workloads |
| `storage` | Object storage |
| `plus` | Plus subscription |
| `permissions` | Delegated access |
| `addresses` | Account addresses |
| `public_api` | Public catalog, abuse, reviews |
| `health` / `ping` / `open_api` | System endpoints |

## Error handling

```rust
match client.billing.pay_invoice("inv_123", body).await {
    Ok(result) => { /* ... */ }
    Err(IgniteError::Api(err)) => {
        eprintln!("code={} status={}", err.code, err.status);
        // err.payload contains the raw API body
    }
    Err(err) => eprintln!("transport error: {err}"),
}
```

## Examples

```bash
cd public-api/rust

# Auth (email/password)
IGNITE_EMAIL=you@example.com IGNITE_PASSWORD=secret cargo run --example auth

# Services (token or API key)
IGNITE_TOKEN=ek_... cargo run --example services

# Checkout flow
IGNITE_TOKEN=... cargo run --example checkout

# Pay invoice
IGNITE_TOKEN=... IGNITE_INVOICE_ID=inv_... cargo run --example pay_invoice
```

## Build & test

```bash
cd public-api/rust
cargo build
cargo test
```

## Generate from OpenAPI

From the repo root:

```bash
bun run docs:api
bun run sdk:rust
```

`docs:api` must run first (`apps/backend/app/openapi.generated.json`).

## CI & releases

The client is versioned in `public-api/rust` of the monorepo. Bump `"version"` in `Cargo.toml` when you want a new major/minor/patch line.

## Sync error codes

From the repo root:

```bash
node scripts/sync-rust-error-codes.mjs
```

Or:

```bash
bun run sync:rust-error-codes
```

## License

See [LICENSE.md](LICENSE.md). Unmodified use in your own projects is permitted; modification is not.
