mod controllers;
mod services;

use controllers::create_transaction;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use ntex::web;
use std::{env, sync::Arc};

pub type PgPool = diesel::r2d2::Pool<ConnectionManager<diesel::PgConnection>>;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<diesel::PgConnection>::new(db_url);
    let pool = diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let pool = Arc::new(pool);
    let app_factory = move || {
        web::App::new()
            .state(pool.clone())
            .service(create_transaction)
    };

    web::HttpServer::new(app_factory)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
