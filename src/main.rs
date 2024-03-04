#![feature(exitcode_exit_method)]
// https://doc.rust-lang.org/stable/rust-by-example/std/rc.html
// https://github.com/sunface/rust-by-practice/tree/master/solutions
// https://lise-henry.github.io/books/trpl2.pdf
// https://google.github.io/comprehensive-rust/control-flow-basics/blocks-and-scopes/scopes.html

use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use anyhow::Result;
use config::{Config, Environment, File};
use log::info;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use web::Data;

use common::{Configuration, Context};
use database::DB;

#[tokio::main]
async fn main() -> Result<()> {
    // logger
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // configuration
    info!("Loading configurations...");
    let config = Config::builder()
        .add_source(File::with_name("config.yml"))
        .add_source(Environment::with_prefix("CR"))
        .build()?;
    let configuration: Configuration = config.try_deserialize().unwrap();
    let configuration = Arc::new(configuration);

    info!("Connecting to database...");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&configuration.db_url())
        .await?;

    // migrations
    info!("Applying migrations");
    migrations::run_migration(&configuration.db_url())
        .await
        .expect("error");

    let db = Arc::new(DB::new(pool));

    info!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Context {
                db: db.clone(),
                config: configuration.clone(),
            }))
            .service(web::scope("/oauth2").configure(idp::oauth2_scope))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
