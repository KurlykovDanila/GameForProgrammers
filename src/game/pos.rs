use std::fmt::Display;

use ez_colorize::ColorizeDisplay;
use serde::{Deserialize, Serialize};

use super::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Pos: ({}, {})]", self.x.green(), self.y.green())
    }
}

impl Pos {
    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn shift(&self, direction: Direction) -> Option<Self> {
        return match direction {
            Direction::Top => {
                let new_y = self.y.checked_add(1)?;
                Some(Pos::new(self.x, new_y))
            }
            Direction::Bottom => {
                let new_y = self.y.checked_sub(1)?;
                Some(Pos::new(self.x, new_y))
            }
            Direction::Right => {
                let new_x = self.x.checked_add(1)?;
                Some(Pos::new(new_x, self.y))
            }
            Direction::Left => {
                let new_x = self.x.checked_sub(1)?;
                Some(Pos::new(new_x, self.y))
            }
            _ => None,
        };
    }
}

impl Into<(u8, u8)> for Pos {
    fn into(self) -> (u8, u8) {
        (self.x, self.y)
    }
}

impl Into<Pos> for (u8, u8) {
    fn into(self) -> Pos {
        Pos {
            x: self.0,
            y: self.1,
        }
    }
}
