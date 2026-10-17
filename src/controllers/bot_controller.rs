use actix_web::{HttpResponse, Result, web};

use crate::models::game_bot::GameBotDTO;
use crate::services::bot_service::BotService;

#[actix_web::get("/info")]
pub async fn get_bot_info() -> Result<HttpResponse> {
    let bot_info = BotService::get_bot_info();
    Ok(HttpResponse::Ok().json(bot_info))
}

#[actix_web::post("/play")]
pub async fn play(game_state: web::Json<GameBotDTO>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(BotService::play(&game_state)))
}

pub fn bot_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/bots")
            .service(get_bot_info)
            .service(play),
    );
}
