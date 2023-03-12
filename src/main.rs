use crate::server::endpoints::health_check::handle_check_health;
use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use env_logger::Env;
use log::info;
use std::env;

mod db;
mod schema;
mod server;

const PORT: &str = "8080";
const HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() -> Result<()> {
    let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL")?);
    let db_pool = Pool::builder().build(manager)?;
    let db_pool_data = web::Data::new(db_pool);

    env_logger::init_from_env(Env::new().default_filter_or("info"));

    info!("server has launched on http://{HOST}:{PORT}");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_pool_data.clone())
            .service(handle_check_health)
    })
    .bind(("0.0.0.0", PORT.parse()?))?
    .run()
    .await?;

    Ok(())
}
