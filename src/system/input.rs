use bevy::{
    ecs::system::{Res, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    state::state::{NextState, State},
};

use crate::core::{traits::Idx1, Board, GameState, Ordinal, SelectedColumn};

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
    .filter_map(|(k, o)| keys.just_pressed(k).then(|| o))
    .for_each(|o| selected.shift(o));

    let GameState::Incomplete(current_player) = *state.get() else {
        return;
    };

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
    .filter_map(|(i, k)| keys.just_pressed(k).then(|| i))
    .for_each(|i| selected.set(i));

    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
        execute_drop(selected.idx(), &mut board, current_player, &mut next_state);
    }
}
