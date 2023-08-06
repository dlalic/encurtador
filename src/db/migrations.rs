use crate::error::Error;
#[cfg(test)]
use crate::{config::database_url, router::Repo};
use diesel::PgConnection;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
#[cfg(test)]
use dotenv::dotenv;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(conn: &mut PgConnection) -> Result<(), Error> {
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(Error::from)?;
    Ok(())
}

#[cfg(test)]
pub async fn run_migrations_and_test_transactions<F>(f: F) -> Repo
where
    F: FnOnce(&mut PgConnection) + Send + Unpin + 'static,
{
    dotenv().ok();
    let repo = Repo::with_test_transactions(&database_url());
    repo.run(move |mut conn| -> Result<(), Error> {
        run_migrations(&mut conn)?;
        f(&mut conn);
        Ok(())
    })
    .await
    .unwrap();
    repo
}
