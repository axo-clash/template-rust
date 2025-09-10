use actix_web::{HttpResponse, Result, web};

use crate::models::game_bot::GameBotDTO;
use crate::models::play_response::PlayResponseDTO;
use crate::services::bot_service::BotService;

#[actix_web::get("/info")]
pub async fn get_bot_info() -> Result<HttpResponse> {
    let bot_info = BotService::get_bot_info();
    Ok(HttpResponse::Ok().json(bot_info))
}

#[actix_web::post("/play")]
pub async fn play(game_state: web::Json<GameBotDTO>) -> Result<HttpResponse> {
    let action = BotService::play(&game_state);
    Ok(HttpResponse::Ok().json(PlayResponseDTO { action }))
}
