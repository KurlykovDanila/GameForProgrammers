use std::fmt::Display;

use ez_colorize::ColorizeDisplay;
use serde::{Deserialize, Serialize};

use super::traits::IntoBytes;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Direction {
    Top,
    Bottom,
    Right,
    Left,
    None,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Bottom => write!(f, "{}", "Bot".green()),
            Direction::Top => write!(f, "{}", "Top".green()),
            Direction::Left => write!(f, "{}", "Left".green()),
            Direction::Right => write!(f, "{}", "Right".green()),
            Direction::None => write!(f, "{}", "None".red()),
        }
    }
}

impl IntoBytes<1> for Direction {
    fn into_bytes(&self) -> [u8; 1] {
        [match self {
            Direction::Bottom => 0,
            Direction::Top => 1,
            Direction::Left => 2,
            Direction::Right => 3,
            Direction::None => 4,
        }]
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::None
    }
}
