pub const USER_PRINCIPAL_ID_STORE: &str = "user-principal-id";
pub const USER_PRINCIPAL_JWK: &str = "jwk_key";
pub const REFERRER_STORE: &str = "referrer";
pub const ACCOUNT_CONNECTED_STORE: &str = "account-connected-1";
pub mod local;

pub mod remote;

pub mod auth {
    use web_time::Duration;

    /// Delegation Expiry, 7 days
    pub const DELEGATION_MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24 * 7);
    /// Refresh expiry, 30 days
    pub const REFRESH_MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24 * 30);
    pub const REFRESH_TOKEN_COOKIE: &str = "user-identity";
}
