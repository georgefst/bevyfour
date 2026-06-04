use std::iter::successors;

use smart_default::SmartDefault;

use crate::{BOARDHEIGHT as BH, BOARDWIDTH as BW};

#[macro_export]
macro_rules! p2 {
    ($x:expr, $y:expr) => {
        $crate::core::PointC4 {
            col: $crate::core::Column($x),
            row: $crate::core::Row($y),
        }
    };
}

#[derive(Debug, SmartDefault, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    #[default]
    One,
    Two,
    None,
}

#[derive(Debug, SmartDefault, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Column(#[default(BW / 2)] pub usize);

#[derive(Debug, SmartDefault, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Row(#[default(0)] pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointC4 {
    pub col: Column,
    pub row: Row,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    Horizontal,
    Vertical,
    DiagonalA,
    DiagonalB,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ordinal {
    N,
    E,
    S,
    W,
}

pub type Delta1 = i32;
pub type Delta2 = (i32, i32);

impl Ordinal {
    pub fn ydelta(&self) -> Delta1 {
        match self {
            Ordinal::N => 1,
            Ordinal::S => -1,
            Ordinal::W | Ordinal::E => 0,
        }
    }

    pub fn xdelta(&self) -> Delta1 {
        match self {
            Ordinal::E => 1,
            Ordinal::W => -1,
            Ordinal::N | Ordinal::S => 0,
        }
    }
}

impl Axis {
    pub const fn ord_pair(&self) -> (Ordinal, Ordinal) {
        match self {
            Axis::Horizontal => (Ordinal::W, Ordinal::E),
            Axis::Vertical => (Ordinal::N, Ordinal::S),
            Axis::DiagonalA => (Ordinal::N, Ordinal::E),
            Axis::DiagonalB => (Ordinal::N, Ordinal::W),
        }
    }
}

impl Player {
    pub fn next(&self) -> Self {
        if Self::One == *self {
            Self::Two
        } else {
            Self::One
        }
    }
}

impl Column {
    fn bounded(i: i32) -> Self {
        Self(0.max(i).min(BW as i32) as usize)
    }

    pub fn try_shift(&self, ord: Ordinal) -> Option<Self> {
        (0..BW).find_map(|i| (i as i32 == self.0 as i32 + ord.ydelta()).then_some(Self(i)))
    }

    pub fn shifted(&self, ord: Ordinal) -> Self {
        Self::bounded(self.0 as i32 + ord.xdelta())
    }
}

impl Row {
    fn bounded(i: i32) -> Self {
        Self(0.max(i).min(BH as i32) as usize)
    }

    pub fn try_shift(&self, ord: Ordinal) -> Option<Self> {
        (0..BH).find_map(|i| (i as i32 == self.0 as i32 + ord.xdelta()).then_some(Self(i)))
    }

    pub fn shifted(&self, ord: Ordinal) -> Self {
        Self::bounded(self.0 as i32 + ord.ydelta())
    }
}

impl PointC4 {
    pub fn ray(&self, ords: (Ordinal, Ordinal)) -> impl Iterator<Item = Self> {
        successors(Some(*self), move |p| p.try_shift(&ords))
    }

    pub fn try_shift(&self, (col_ord, row_ord): &(Ordinal, Ordinal)) -> Option<Self> {
        match (self.col.try_shift(*col_ord), self.row.try_shift(*row_ord)) {
            (Some(col), Some(row)) => Some(Self { col, row }),
            _ => None,
        }
    }

    pub fn shifted(&self, (col_ord, row_ord): &(Ordinal, Ordinal)) -> Self {
        Self {
            col: self.col.shifted(*col_ord),
            row: self.row.shifted(*row_ord),
        }
    }
}
