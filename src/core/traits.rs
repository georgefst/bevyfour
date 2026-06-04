use std::ops::{Index, IndexMut};


use crate::{
    core::{Board, Column, Ordinal, Player, PointC4, Position, SelectedColumn},
    p2, BOARDHEIGHT as BH,
};

pub trait Idx1 {
    fn idx(&self) -> usize;
    fn shift(&mut self, ord: Ordinal);
    fn set(&mut self, i: usize);
}

pub trait Idx2 {
    fn point(&self) -> &PointC4;
    fn shift(&mut self, ords: (Ordinal, Ordinal));
    fn set(&mut self, xy: (usize, usize));
}

impl Idx1 for SelectedColumn {
    fn idx(&self) -> usize {
        self.0 .0
    }

    fn shift(&mut self, ord: Ordinal) {
        self.0 = self.0.shifted(ord)
    }

    fn set(&mut self, i: usize) {
        self.0 = Column(i)
    }
}

impl Idx2 for Position {
    fn point(&self) -> &PointC4 {
        &self.0
    }

    fn shift(&mut self, ords: (Ordinal, Ordinal)) {
        self.0 = self.0.shifted(&ords)
    }

    fn set(&mut self, (x, y): (usize, usize)) {
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
