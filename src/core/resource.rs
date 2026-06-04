use bevy::{ecs::resource::Resource, state::state::States};
use smart_default::SmartDefault;

use crate::{core::Column, BOARDHEIGHT as BH, BOARDWIDTH as BW};

use super::r#type::Player;

#[derive(States, Debug, SmartDefault, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Incomplete(Player),
    Won(Player),
    Draw,
}

#[derive(Resource, SmartDefault, Clone)]
pub struct Board(pub [[Option<Player>; BH]; BW]);

#[derive(Resource, SmartDefault)]
pub struct SelectedColumn(#[default(Column(BW / 2))] pub Column);
