use bevy::{
    ecs::system::{Res, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    state::state::{NextState, State},
};

use crate::board::{Board, GameState};

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut board: ResMut<Board>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let GameState::Incomplete(current_player) = state.get() else {
        return;
    };

    let keycodes = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
    ];

    for (x, key) in keycodes.iter().enumerate() {
        if keys.just_pressed(*key) {
            board.drop(x, *current_player);
            if let Some(winner) = board.validate() {
                next_state.set(GameState::Won(winner));
            } else {
                next_state.set(GameState::Incomplete(current_player.next()));
            }
        }
    }
}
