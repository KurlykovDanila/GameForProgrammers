use ez_colorize::ColorizeDisplay;
use serde::Serialize;
use std::fmt::Display;

use super::{direction::Direction, pos::Pos, traits::IntoBytes};

#[derive(Clone, Serialize)]
pub struct Map {
    field: Vec<Cell>,
    size: u8,
}

pub struct MutMap {
    pub field: Vec<Cell>,
    size: u8,
}

impl MutMap {
    pub const fn get_size(&self) -> u8 {
        self.size
    }

    pub fn get_cell(&self, pos: Pos) -> Cell {
        self.field[(pos.x * self.size + pos.y) as usize]
    }

    pub fn immut_map(self) -> Map {
        Map {
            field: self.field,
            size: self.size,
        }
    }
}

impl Map {
    pub fn new_empty(size: u8) -> Self {
        Self {
            size,
            field: vec![Cell::Empty; (size * size) as usize],
        }
    }

    pub fn clone_mut_map(&self) -> MutMap {
        MutMap {
            field: self.field.clone(),
            size: self.get_size(),
        }
    }

    pub const fn get_size(&self) -> u8 {
        self.size
    }

    pub fn get_cell(&self, pos: Pos) -> Cell {
        self.field[(pos.x * self.size + pos.y) as usize]
    }

    fn pos_in_map(&self, pos: Pos) -> bool {
        let (x, y) = pos.into();
        return x < self.size && y < self.size;
    }

    pub fn can_move(&self, from: Pos, direction: Direction) -> CanMove {
        if let Some(pos) = from.shift(direction) {
            if self.pos_in_map(pos) {
                if self.get_cell(pos).can_move() {
                    return CanMove::Yes;
                } else {
                    return CanMove::No(WhyDontCanMove::ImpassableObject {
                        obj: self.get_cell(pos),
                    });
                }
            } else {
                return CanMove::No(WhyDontCanMove::OutOfRange);
            }
        } else {
            return CanMove::No(WhyDontCanMove::PosOverflow);
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                match self.field[(x * self.size + y) as usize] {
                    Cell::Player { .. } => {
                        write!(f, "{} ", "Player".green())?;
                    }
                    Cell::Bullet => {
                        write!(f, "{} ", "Bullet".red())?;
                    }
                    _ => {
                        write!(f, "{:?} ", self.field[(x * self.size + y) as usize])?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub enum CanMove {
    Yes,
    No(WhyDontCanMove),
}

pub enum WhyDontCanMove {
    ImpassableObject { obj: Cell },
    OutOfRange,
    PosOverflow,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Cell {
    Bushes,
    Empty,
    Wall,
    Player,
    Bot,
    Bullet,
}

impl Cell {
    pub const fn can_move(&self) -> bool {
        match self {
            Cell::Bot => false,
            Cell::Bullet => true,
            Cell::Bushes => true,
            Cell::Empty => true,
            Cell::Player => false,
            Cell::Wall => false,
        }
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Cell::Bot => 0,
            Cell::Bullet => 1,
            Cell::Bushes => 2,
            Cell::Empty => 3,
            Cell::Player => 4,
            Cell::Wall => 5,
        }
    }
}

impl IntoBytes<1> for Cell {
    fn into_bytes(&self) -> [u8; 1] {
        [self.as_u8()]
    }
}
