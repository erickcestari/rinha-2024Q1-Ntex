mod controllers;
mod errors;
mod models;
mod services;

use controllers::{create_transaction, get_extrato};
use dotenv::dotenv;
use ntex::web;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub type PgPool = Pool<Postgres>;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port_str = env::var("PORT").expect("PORT must be set");
    let port: u16 = port_str
        .parse()
        .expect("Failed to parse PORT string to integer");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");

    //let pool = Arc::new(pool);
    let app_factory = move || {
        web::App::new()
            .state(pool.clone())
            .service(create_transaction)
            .service(get_extrato)
    };

    web::HttpServer::new(app_factory)
        .bind(("localhost", port))?
        .run()
        .await
}
