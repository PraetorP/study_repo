use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use dotenv::dotenv;

use tokio_postgres::NoTls;
mod config;
mod db;
mod handlers;
mod models;
use handlers::*;


#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    println!(
        "starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .service(get_todos)
            .service(get_items)
            .service(create_todo)
            .service(check_items)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await?;

    Ok(())
}
