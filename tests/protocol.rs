use serde_json::json;
use template_rust::models::{
    action::Action, coord::CoordDTO, game_bot::GameBotDTO, play_response::PlayResponseDTO,
};

#[test]
fn response_can_return_a_classic_action_or_a_targeted_trap() {
    let classic = PlayResponseDTO {
        action: Action::MoveUp,
        coordinates: None,
    };
    assert_eq!(
        serde_json::to_value(classic).unwrap(),
        json!({"action":"MOVE_UP"})
    );
    let trap = PlayResponseDTO {
        action: Action::PlaceTrap,
        coordinates: Some(CoordDTO {
            x: Some(3),
            y: Some(4),
        }),
    };
    assert_eq!(
        serde_json::to_value(trap).unwrap(),
        json!({"action":"PLACE_TRAP","coordinates":{"x":3,"y":4}})
    );
}

#[test]
fn new_fields_decode_without_interpreting_the_game_rules() {
    for seat in ["FIRST_BOT", "SECOND_BOT"] {
        let value = json!({"bot":{"botType":seat,"data":{"nextTrapTurn":20}},"game":{
            "currentTurn":19,"rules":{"startingLife":10,"responseTimeoutMs":2000,"maxTurns":100,"maxForbiddenActions":10},
            "board":[[{"coordinates":{"x":3,"y":4},"content":seat,
                "trap":{"placedBy":"FIRST_BOT","placedAtTurn":10,"armedFromTurn":11,"expiresAfterTurn":20},
                "items":[{"type":"BULLET_UP","owner":{"botType":"SECOND_BOT"}}]}]],
            "actions":[{"action":"PLACE_TRAP","coordinates":{"x":3,"y":4},"createdDate":"2026-10-17T12:00:00Z"}]}});
        let state: GameBotDTO = serde_json::from_value(value).unwrap();
        assert_eq!(state.bot.unwrap().data.unwrap().next_trap_turn, Some(20));
        let game = state.game.unwrap();
        assert_eq!(game.current_turn, Some(19));
        assert_eq!(game.rules.unwrap().max_turns, Some(100));
        let cell = &game.board.as_ref().unwrap()[0][0];
        let trap = cell.trap.as_ref().unwrap();
        assert_eq!(trap.placed_at_turn, 10);
        assert_eq!(trap.armed_from_turn, 11);
        assert_eq!(trap.expires_after_turn, 20);
        assert_eq!(cell.items.as_ref().unwrap().len(), 1);
        assert_eq!(game.actions.unwrap()[0].action, Some(Action::PlaceTrap));
    }
}

#[test]
fn old_payloads_without_new_fields_are_still_accepted() {
    let state: GameBotDTO = serde_json::from_value(
        json!({"bot":{"data":{}},"game":{"currentTurn":8,"board":[[{"trap":null}]]}}),
    )
    .unwrap();
    assert_eq!(state.bot.unwrap().data.unwrap().next_trap_turn, None);
    assert!(state.game.unwrap().board.unwrap()[0][0].trap.is_none());
}
