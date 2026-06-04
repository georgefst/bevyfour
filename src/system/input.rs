use bevy::{
    ecs::system::{Res, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    state::state::{NextState, State},
};

use crate::core::{traits::BoundedIndex, Board, GameState, Ordinal, SelectedColumn};

use super::play::execute_drop;

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedColumn>,
    mut board: ResMut<Board>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    [
        (KeyCode::ArrowLeft, Ordinal::W),
        (KeyCode::ArrowRight, Ordinal::E),
    ]
    .into_iter()
    .for_each(|(k, o)| {
        if keys.just_pressed(k) {
            selected.shift(o);
        }
    });

    [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
    ]
    .into_iter()
    .enumerate()
    .for_each(|(i, k)| {
        if keys.just_pressed(k) {
            selected.set(i)
        }
    });

    let GameState::Incomplete(current_player) = *state.get() else {
        panic!("unreachable")
    };

    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
        execute_drop(
            selected.value(),
            &mut board,
            current_player,
            &mut next_state,
        );
    }
}
