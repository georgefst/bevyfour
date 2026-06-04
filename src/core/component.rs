use bevy::{asset::Handle, ecs::component::Component, sprite_render::ColorMaterial};

use super::r#type::PointC4;

#[derive(Component)]
pub struct Position(pub PointC4);

#[derive(Component)]
pub struct CellMaterial(pub Handle<ColorMaterial>);
