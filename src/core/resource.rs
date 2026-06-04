use bevy::{ecs::resource::Resource, state::state::States};
use smart_default::SmartDefault;

use crate::{BOARDHEIGHT as BH, BOARDWIDTH as BW};

use super::r#type::Player;

#[derive(States, Debug, SmartDefault, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Incomplete(Player),
    Won(Player),
    Draw,
}

#[derive(Resource, SmartDefault, Clone, Copy)]
pub struct Board(pub [[Option<Player>; BH]; BW]);

#[derive(Resource, SmartDefault, Clone, Copy)]
pub struct SelectedColumn(#[default(BW / 2)] pub usize);
