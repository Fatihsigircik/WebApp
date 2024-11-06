use actix_web::{App, HttpServer};
mod routing;
mod controller;
mod model;
mod db;
mod services;
mod cfg;
mod helper;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
    App::new().configure(routing::init_routes)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
