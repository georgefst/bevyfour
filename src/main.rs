#![allow(dead_code)]

use std::ops::Range;

use bevy::{
    app::{App, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::app::AppExtStates,
    DefaultPlugins,
};

use crate::{
    board::{is_incomplete, Board, GameState},
    input::handle_input,
    render::{render_board, spawn_board},
};
mod board;
mod input;
mod render;

// global constants
pub const BOARDWIDTH: usize = 7;
pub const BOARDHEIGHT: usize = 6;
pub const WIDTHRANGE: Range<usize> = 0..BOARDWIDTH;
pub const HEIGHTRANGE: Range<usize> = 0..BOARDHEIGHT;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(GameState::default())
        .insert_resource(Board::default())
        .add_systems(Startup, spawn_board)
        .add_systems(Update, (handle_input.run_if(is_incomplete), render_board))
        .run();
}
