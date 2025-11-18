use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    // Integrate with sqlx::migrate! or refinery; placeholder for now.
    Ok(())
}
