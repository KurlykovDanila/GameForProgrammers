use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterInfo {
    pub health: u8,
    pub gun_reloading_await: u8,
    pub bullet_damage: u8,
    pub bullet_range: u8,
    pub pos: Option<Pos>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(tag = "direction")]
pub enum Direction {
    Top,
    Bottom,
    Right,
    Left,
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerInfo {
    pub character: CharacterInfo,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Map {
    pub field: Vec<Cell>,
    pub size: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Cell {
    Bushes,
    Empty,
    Wall,
    Player,
    Bot,
    Bullet,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameInfo {
    pub map: Map,
    pub players: Vec<PlayerInfo>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Action {
    Move { direction: Direction, range: u8 },
    Attack { direction: Direction },
    Reload,
    Nothing,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    pub actions: Vec<Action>,
}

impl Response {
    pub fn new(actions: &[Action]) -> Self {
        Self {
            actions: Vec::from(actions.clone()),
        }
    }
}
