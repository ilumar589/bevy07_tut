use sqlx::migrate::{Migrator, MigrateError};
use sqlx::PgPool;

pub async fn migrate(pool: &PgPool) -> Result<(), MigrateError> {
    let migrator= Migrator::new(std::path::Path::new("./migrations")).await?;
    migrator.run(pool).await
}