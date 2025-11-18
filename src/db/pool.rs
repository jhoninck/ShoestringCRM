use sqlx::PgPool;
use crate::config::AppConfig;

pub async fn create_pool(cfg: &AppConfig) -> anyhow::Result<PgPool> {
    let pool = PgPool::connect(&cfg.database_url).await?;
    Ok(pool)
}
