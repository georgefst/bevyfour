use std::ops::{Index, IndexMut};

use bevy::{
    asset::Handle,
    ecs::{component::Component, resource::Resource, system::Res},
    sprite_render::ColorMaterial,
    state::state::{State, States},
};
use smart_default::SmartDefault;

use crate::{BOARDHEIGHT as BH, BOARDWIDTH as BW, HEIGHTRANGE as HR, WIDTHRANGE as WR};

// states
#[derive(States, Debug, SmartDefault, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Incomplete(Player),
    Won(Player),
    Draw,
}

// resources
#[derive(Resource, SmartDefault, Clone)]
pub struct Board(pub [[Option<Player>; BH]; BW]);

// entities
#[derive(Component)]
pub struct BoardCell;

// components
#[derive(Component)]
pub struct Position(pub Point2);

#[derive(Component)]
pub struct CellMaterial(pub Handle<ColorMaterial>);

// game logic
macro_rules! p2 {
    ($x:expr, $y:expr) => {
        Point2 { x: $x, y: $y }
    };
}

impl Board {
    pub fn drop(&mut self, x: usize, p: Player) {
        *self[x]
            .iter_mut()
            .find(|cell| cell.is_none())
            .expect("drop in full col") = Some(p);
    }

    pub fn validate(&self) -> Option<Player> {
        WR.map(|x| self.match_ray(p2!(x, 0).ray(Delta::Vertical)))
            .chain(WR.map(|x| self.match_ray(p2!(x, 0).ray(Delta::DiagonalA))))
            .chain(WR.map(|x| self.match_ray(p2!(x, 0).ray(Delta::DiagonalB))))
            .chain(HR.map(|y| self.match_ray(p2!(y, 0).ray(Delta::Horizontal))))
            .find(|check| check.is_some())?
    }

    fn match_ray(&self, ray: impl Iterator<Item = Point2>) -> Option<Player> {
        match ray.fold((Player::None, 0, 0), |(prev, cons, max), p| {
            if let Some(player) = self[p] {
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
}

pub fn is_incomplete(state: Res<State<GameState>>) -> bool {
    matches!(state.get(), GameState::Incomplete(_))
}

// non ECS structures
#[derive(Debug, SmartDefault, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    #[default]
    One,
    Two,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2 {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Delta {
    Horizontal,
    Vertical,
    DiagonalA,
    DiagonalB,
}

// simple impls
impl Player {
    pub fn next(&self) -> Self {
        if Self::One == *self {
            Self::Two
        } else {
            Self::One
        }
    }
}

impl Position {
    pub fn p(&self) -> Point2 {
        self.0
    }

    pub fn x(&self) -> usize {
        self.0.x
    }

    pub fn y(&self) -> usize {
        self.0.y
    }
}

impl Point2 {
    fn ray(&self, delta: Delta) -> impl Iterator<Item = Self> {
        let (dx, dy) = match delta {
            Delta::Horizontal => (0, 1),
            Delta::Vertical => (1, 0),
            Delta::DiagonalA => (1, 1),
            Delta::DiagonalB => (-1, 1),
        };
        (0..).map_while(move |i| Self::try_with(&self, &(dx * i, dy * i)))
    }

    fn try_with(&self, (dx, dy): &(i32, i32)) -> Option<Self> {
        if let (Ok(x), Ok(y)) = (
            usize::try_from(self.x as i32 + dx),
            usize::try_from(self.y as i32 + dy),
        ) {
            if x < BW && y < BH {
                Some(p2!(x, y))
            } else {
                None
            }
        } else {
            None
        }
    }
}

// stdlib trait impls
impl Index<Point2> for Board {
    type Output = Option<Player>;
    fn index(&self, p: Point2) -> &Self::Output {
        &self.0[p.x][p.y]
    }
}

impl IndexMut<Point2> for Board {
    fn index_mut(&mut self, p: Point2) -> &mut Self::Output {
        &mut self.0[p.x][p.y]
    }
}

impl Index<usize> for Board {
    type Output = [Option<Player>; BH];
    fn index(&self, col: usize) -> &Self::Output {
        &self.0[col]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, col: usize) -> &mut Self::Output {
        &mut self.0[col]
    }
}
