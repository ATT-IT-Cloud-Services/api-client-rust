use crate::config::IgniteClientConfig;
use crate::resources::{
    AuthResource, ServicesResource, BillingResource, CheckoutResource, ShopResource, DashboardResource, DomainsResource, SubdomainsResource, IpManagerResource, SshKeysResource, SupportResource, UserResource, WalletResource, BusinessResource, AffiliatesResource, FamilyResource, ByoipResource, FloatingIpsResource, AddressesResource, BlogPostsResource, ByoasResource, CaseReportsResource, ContainerResource, CouponsResource, DocumentsResource, EventsResource, HealthResource, IntegrationsResource, NewsletterResource, OpenApiResource, PermissionsResource, PingResource, PlusResource, PreordersResource, PublicApiResource, PublicSettingsResource, StorageResource, TeamResource, ThemeAssetsResource, TrafficResource, WebhooksResource,
};
use crate::transport::HttpTransport;

/// Official ignite API client.
///
/// Resource namespaces follow the public OpenAPI document
/// (host-root paths, e.g. /services — not legacy /api/services).
/// Failures return [`crate::ApiError`] with machine-readable `code`, HTTP `status`, and raw `payload`.
pub struct IgniteClient {
    transport: HttpTransport,
    pub auth: AuthResource,
    pub services: ServicesResource,
    pub billing: BillingResource,
    pub checkout: CheckoutResource,
    pub shop: ShopResource,
    pub dashboard: DashboardResource,
    pub domains: DomainsResource,
    pub subdomains: SubdomainsResource,
    pub ip_manager: IpManagerResource,
    pub ssh_keys: SshKeysResource,
    pub support: SupportResource,
    pub user: UserResource,
    pub wallet: WalletResource,
    pub business: BusinessResource,
    pub affiliates: AffiliatesResource,
    pub family: FamilyResource,
    pub byoip: ByoipResource,
    pub floating_ips: FloatingIpsResource,
    pub addresses: AddressesResource,
    pub blog_posts: BlogPostsResource,
    pub byoas: ByoasResource,
    pub case_reports: CaseReportsResource,
    pub container: ContainerResource,
    pub coupons: CouponsResource,
    pub documents: DocumentsResource,
    pub events: EventsResource,
    pub health: HealthResource,
    pub integrations: IntegrationsResource,
    pub newsletter: NewsletterResource,
    pub open_api: OpenApiResource,
    pub permissions: PermissionsResource,
    pub ping: PingResource,
    pub plus: PlusResource,
    pub preorders: PreordersResource,
    pub public_api: PublicApiResource,
    pub public_settings: PublicSettingsResource,
    pub storage: StorageResource,
    pub team: TeamResource,
    pub theme_assets: ThemeAssetsResource,
    pub traffic: TrafficResource,
    pub webhooks: WebhooksResource,
}

impl IgniteClient {
    pub fn new(config: IgniteClientConfig) -> Result<Self, reqwest::Error> {
        let transport = HttpTransport::new(config)?;
        Ok(Self::from_transport(transport))
    }

    fn from_transport(transport: HttpTransport) -> Self {
        Self {
            auth: AuthResource::new(transport.clone()),
            services: ServicesResource::new(transport.clone()),
            billing: BillingResource::new(transport.clone()),
            checkout: CheckoutResource::new(transport.clone()),
            shop: ShopResource::new(transport.clone()),
            dashboard: DashboardResource::new(transport.clone()),
            domains: DomainsResource::new(transport.clone()),
            subdomains: SubdomainsResource::new(transport.clone()),
            ip_manager: IpManagerResource::new(transport.clone()),
            ssh_keys: SshKeysResource::new(transport.clone()),
            support: SupportResource::new(transport.clone()),
            user: UserResource::new(transport.clone()),
            wallet: WalletResource::new(transport.clone()),
            business: BusinessResource::new(transport.clone()),
            affiliates: AffiliatesResource::new(transport.clone()),
            family: FamilyResource::new(transport.clone()),
            byoip: ByoipResource::new(transport.clone()),
            floating_ips: FloatingIpsResource::new(transport.clone()),
            addresses: AddressesResource::new(transport.clone()),
            blog_posts: BlogPostsResource::new(transport.clone()),
            byoas: ByoasResource::new(transport.clone()),
            case_reports: CaseReportsResource::new(transport.clone()),
            container: ContainerResource::new(transport.clone()),
            coupons: CouponsResource::new(transport.clone()),
            documents: DocumentsResource::new(transport.clone()),
            events: EventsResource::new(transport.clone()),
            health: HealthResource::new(transport.clone()),
            integrations: IntegrationsResource::new(transport.clone()),
            newsletter: NewsletterResource::new(transport.clone()),
            open_api: OpenApiResource::new(transport.clone()),
            permissions: PermissionsResource::new(transport.clone()),
            ping: PingResource::new(transport.clone()),
            plus: PlusResource::new(transport.clone()),
            preorders: PreordersResource::new(transport.clone()),
            public_api: PublicApiResource::new(transport.clone()),
            public_settings: PublicSettingsResource::new(transport.clone()),
            storage: StorageResource::new(transport.clone()),
            team: TeamResource::new(transport.clone()),
            theme_assets: ThemeAssetsResource::new(transport.clone()),
            traffic: TrafficResource::new(transport.clone()),
            webhooks: WebhooksResource::new(transport.clone()),
            transport,
        }
    }

    /// Set or clear the Bearer token used for authenticated requests.
    pub fn set_token(&self, token: Option<String>) {
        self.transport.set_token(token);
    }

    pub fn token(&self) -> Option<String> {
        self.transport.token()
    }
}
