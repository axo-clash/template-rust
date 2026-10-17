use crate::models::action::Action;
use crate::models::bot_info::BotInfoDTO;
use crate::models::game_bot::GameBotDTO;
use crate::models::play_response::PlayResponseDTO;

pub struct BotService {}

impl BotService {
    pub fn get_bot_info() -> BotInfoDTO {
        BotInfoDTO {
            name: "Rust Template Bot".to_string(),
            version: "1.0.0".to_string(),
        }
    }

    pub fn play(_game: &GameBotDTO) -> PlayResponseDTO {
        // Replace this placeholder with your own strategy.
        PlayResponseDTO {
            action: Action::ShootUp,
            coordinates: None,
        }
    }
}
