use bevy::{
    ecs::system::Res,
    state::state::{NextState, State},
};

use crate::{
    core::{Axis, Board, GameState, Player, PointC4},
    p2, HEIGHTRANGE as HR, WIDTHRANGE as WR,
};

pub fn is_incomplete(state: Res<State<GameState>>) -> bool {
    matches!(state.get(), GameState::Incomplete(_))
}

pub fn drop_piece(board: &mut Board, x: usize, p: Player) {
    *board[x]
        .iter_mut()
        .find(|cell| cell.is_none())
        .expect("drop in full col") = Some(p);
}

pub fn validate(board: &Board) -> Option<Player> {
    WR.map(|x| match_ray(board, p2!(x, 0).ray(Axis::Vertical.ord_pair())))
        .chain(WR.map(|x| match_ray(board, p2!(x, 0).ray(Axis::DiagonalA.ord_pair()))))
        .chain(WR.map(|x| match_ray(board, p2!(x, 0).ray(Axis::DiagonalB.ord_pair()))))
        .chain(HR.map(|y| match_ray(board, p2!(y, 0).ray(Axis::Horizontal.ord_pair()))))
        .find(|check| check.is_some())?
}

fn match_ray(board: &Board, ray: impl Iterator<Item = PointC4>) -> Option<Player> {
    match ray.fold((Player::None, 0, 0), |(prev, cons, max), p| {
        println!("{p:?}");
        if let Some(player) = board[p] {
            if player == prev {
                (player, cons + 1, max.max(cons + 1))
            } else {
                (player, 1, max)
            }
        } else {
            (Player::None, 0, max)
        }
    }) {
        (out, _, 4..) => Some(out),
        _ => None,
    }
}

pub fn execute_drop(
    col: usize,
    board: &mut Board,
    current_player: Player,
    next_state: &mut NextState<GameState>,
) {
    drop_piece(board, col, current_player);
    if let Some(winner) = validate(board) {
        next_state.set(GameState::Won(winner));
    } else {
        next_state.set(GameState::Incomplete(current_player.next()));
    }
}
