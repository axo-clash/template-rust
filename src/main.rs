use actix_web::{App, HttpServer};
use controllers::bot_controller::{get_bot_info, play};

mod controllers;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_bot_info).service(play))
        .bind("0.0.0.0:6000")
        .unwrap()
        .run()
        .await
}
