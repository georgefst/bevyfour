use std::f32::consts::PI;

use bevy::{
    asset::Assets,
    camera::Camera2d,
    color::Color,
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::{
        primitives::{Circle, RegularPolygon},
        Quat,
    },
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    state::state::State,
    transform::components::Transform,
};

use crate::{
    core::{
        traits::BoundedIndex, Board, BoardCell, CellMaterial, Cursor, GameState, Player, Position,
        SelectedColumn,
    },
    p2, BOARDHEIGHT as BH, BOARDWIDTH as BW,
};

const CELL_SIZE: f32 = 80.0;
const X_OFF: f32 = 6.0;
const Y_OFF: f32 = 5.0;
const GREY: Color = Color::srgb(0.2, 0.2, 0.8);
const RED: Color = Color::srgb(0.9, 0.1, 0.1);
const YELLOW: Color = Color::srgb(0.9, 0.8, 0.1);

pub fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let circle = meshes.add(Circle::new(CELL_SIZE / 2.0 - 2.0));
    let x_off = -(X_OFF * CELL_SIZE) / 2.0;
    let y_off = -(Y_OFF * CELL_SIZE) / 2.0;

    for x in 0..BW {
        for y in 0..BH {
            let fx = x_off + x as f32 * CELL_SIZE;
            let fy = y_off + y as f32 * CELL_SIZE;
            let mat = materials.add(ColorMaterial::from_color(GREY));

            commands.spawn((
                Mesh2d(circle.clone()),
                MeshMaterial2d(mat.clone()),
                Transform::from_xyz(fx, fy, 0.0),
                BoardCell,
                Position(p2!(x, y)),
                CellMaterial(mat),
            ));
        }
    }

    let triangle = meshes.add(RegularPolygon::new(CELL_SIZE / 3.0, 3));
    let cursor_mat = materials.add(ColorMaterial::from_color(RED));
    let cursor_x = x_off + (BW / 2) as f32 * CELL_SIZE;
    let cursor_y = (Y_OFF * CELL_SIZE) / 2.0 + CELL_SIZE;

    commands.spawn((
        Mesh2d(triangle),
        MeshMaterial2d(cursor_mat.clone()),
        Transform::from_xyz(cursor_x, cursor_y, 0.0).with_rotation(Quat::from_rotation_z(PI)),
        Cursor,
        CellMaterial(cursor_mat),
    ));
}

pub fn render_board(
    board: Res<Board>,
    mut query: Query<(&Position, &mut CellMaterial), With<BoardCell>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (pos, cell_mat) in &mut query {
        materials.get_mut(&cell_mat.0).expect("idk").color = match board[*pos.point()] {
            Some(Player::One) => RED,
            Some(Player::Two) => YELLOW,
            _ => GREY,
        };
    }
}

pub fn render_cursor(
    selected: Res<SelectedColumn>,
    state: Res<State<GameState>>,
    mut query: Query<(&mut Transform, &CellMaterial), With<Cursor>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let x_off = -(X_OFF * CELL_SIZE) / 2.0;
    let color = match state.get() {
        GameState::Incomplete(Player::One) => RED,
        GameState::Incomplete(Player::Two) => YELLOW,
        _ => GREY,
    };

    for (mut transform, cell_mat) in &mut query {
        transform.translation.x = x_off + selected.value() as f32 * CELL_SIZE;
        materials
            .get_mut(&cell_mat.0)
            .expect("cursor material")
            .color = color;
    }
}
