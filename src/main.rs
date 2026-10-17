use actix_web::{App, HttpServer};

use template_rust::controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "9000".to_string());
    let server = HttpServer::new(|| App::new().configure(controllers::bot_controller::bot_routes))
        .bind(format!("0.0.0.0:{}", port))?;
    println!("Application démarrée sur http://0.0.0.0:{}", port);
    server.run().await
}
