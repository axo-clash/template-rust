use crate::models::action::Action;
use crate::models::bot_info::BotInfoDTO;
use crate::models::game_bot::GameBotDTO;

pub struct BotService {}

impl BotService {
    pub fn get_bot_info() -> BotInfoDTO {
        BotInfoDTO {
            name: "Rust Template Bot".to_string(),
            version: "1.0.0".to_string(),
        }
    }

    pub fn play(_game: &GameBotDTO) -> Action {
        use rand::prelude::*;
        let mut rng = rand::rng();

        let actions = [
            Action::MoveLeft,
            Action::MoveRight,
            Action::MoveUp,
            Action::MoveDown,
            Action::ShootLeft,
            Action::ShootRight,
            Action::ShootUp,
            Action::ShootDown,
        ];

        *actions.choose(&mut rng).unwrap_or(&Action::Unknown)
    }
}
