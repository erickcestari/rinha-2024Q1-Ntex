mod repositories;
mod services;

use ntex::web;
use services::create_transaction::create_transaction;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(create_transaction))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
