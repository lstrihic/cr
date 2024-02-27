use std::io::Error;

use refinery::AsyncMigrate;
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("../migrations");
}

pub async fn run_migration() -> Result<(), refinery::Error> {
    let (mut client, connection) = tokio_postgres::connect("", NoTls).await.expect("errorcina");
    connection.await.expect("failed to connect to the database");

    let migration_report = embedded::migrations::runner()
        .run_async(&mut client)
        .await?;

    for migration in migration_report.applied_migrations() {
        println!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    return Ok(())
}
