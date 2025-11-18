use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub zitadel_issuer: String,
    pub zitadel_audience: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        // In real code, use dotenv / envy.
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            zitadel_issuer: std::env::var("ZITADEL_ISSUER")?,
            zitadel_audience: std::env::var("ZITADEL_AUDIENCE")?,
        })
    }
}
