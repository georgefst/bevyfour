use std::ops::{Index, IndexMut};

use crate::{
    core::{Board, Column, Ordinal, Player, PointC4, Position, Row, SelectedColumn},
    p2, BOARDHEIGHT as BH, BOARDWIDTH as BW,
};

pub trait BoundedIndex: Sized + Copy {
    const BOUND: usize;
    fn value(&self) -> usize;
    fn from_value(i: usize) -> Self;
    fn delta(ord: Ordinal) -> i32;

    fn bounded(i: i32) -> Self {
        Self::from_value(0.max(i).min(Self::BOUND as i32) as usize)
    }

    fn try_shift(&self, ord: Ordinal) -> Option<Self> {
        let target = self.value() as i32 + Self::delta(ord);
        (0..Self::BOUND).find_map(|i| (i as i32 == target).then_some(Self::from_value(i)))
    }

    fn shifted(&self, ord: Ordinal) -> Self {
        Self::bounded(self.value() as i32 + Self::delta(ord))
    }

    fn shift(&mut self, ord: Ordinal) {
        *self = Self::shifted(self, ord)
    }

    fn set(&mut self, i: usize) {
        *self = Self::from_value(i)
    }
}

impl BoundedIndex for Column {
    const BOUND: usize = BW;
    fn value(&self) -> usize {
        self.0
    }
    fn from_value(i: usize) -> Self {
        assert!((0..Self::BOUND).contains(&i));
        Self(i)
    }
    fn delta(ord: Ordinal) -> i32 {
        ord.ydelta()
    }
}

impl BoundedIndex for Row {
    const BOUND: usize = BH;
    fn value(&self) -> usize {
        self.0
    }
    fn from_value(i: usize) -> Self {
        assert!((0..Self::BOUND).contains(&i));
        Self(i)
    }
    fn delta(ord: Ordinal) -> i32 {
        ord.xdelta()
    }
}

impl BoundedIndex for SelectedColumn {
    const BOUND: usize = BW;
    fn value(&self) -> usize {
        self.0
    }
    fn from_value(i: usize) -> Self {
        assert!((0..Self::BOUND).contains(&i));
        Self(i)
    }
    fn delta(ord: Ordinal) -> i32 {
        ord.xdelta()
    }
}

impl Position {
    pub fn point(&self) -> &PointC4 {
        &self.0
    }

    pub fn shift(&mut self, ords: (Ordinal, Ordinal)) {
        self.0 = self.0.shifted(&ords)
    }

    pub fn set(&mut self, (x, y): (usize, usize)) {
        self.0 = p2!(x, y)
    }
}

impl Index<PointC4> for Board {
    type Output = Option<Player>;
    fn index(&self, p: PointC4) -> &Self::Output {
        &self.0[p.col.0][p.row.0]
    }
}

impl IndexMut<PointC4> for Board {
    fn index_mut(&mut self, p: PointC4) -> &mut Self::Output {
        &mut self.0[p.col.0][p.row.0]
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
