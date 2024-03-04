use anyhow::Result;
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("migrations");
}

pub async fn run_migration(connection_string: &str) -> Result<()> {
    let (mut client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;
    tokio::spawn(async move { connection.await });
    embedded::migrations::runner()
        .run_async(&mut client)
        .await?;
    return Ok(());
}
