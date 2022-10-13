use std::{any::Any, fmt::Display};

use super::{
    character::{Bullet, Character, CharacterInfo, Gun, Health},
    direction::Direction,
    map::Map,
    pos::Pos,
    traits::{Attack, Movable, WithCharacter, WithHealth, WithId},
};

pub trait DynPlayer: WithHealth + WithId + Attack + Movable + WithCharacter {}

#[derive(Debug)]
pub struct Player {
    id: u8,
    typ: PlayerType,
    character: Character,
}

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Action {
    Move { direction: Direction, range: u8 },
    Attack { direction: Direction },
    Reload,
    Nothing,
}

impl Player {
    pub fn new_player(id: u8, character: Character) -> Self {
        Self {
            id,
            typ: PlayerType::Player,
            character,
        }
    }

    pub fn new_default_player(id: u8, position: Pos) -> Self {
        Self::new_player(
            id,
            Character::new(
                1,
                position,
                Health::new(100),
                Gun::new(2, Bullet::new(5, 20)),
            ),
        )
    }

    pub fn new_bot(id: u8, character: Character) -> Self {
        let mut bot = Self::new_player(id, character);
        bot.typ = PlayerType::Bot;
        bot
    }
}

#[derive(Debug)]
enum PlayerType {
    Bot,
    Player,
}

impl Movable for Player {
    fn get_position(&self) -> super::pos::Pos {
        self.character.get_position()
    }

    fn shift(&mut self, direction: Direction) {
        self.character.shift(direction);
    }

    fn get_speed(&self) -> u8 {
        self.character.get_speed()
    }
}

impl Attack for Player {
    fn attack(&mut self, direction: Direction) -> Option<Bullet> {
        return self.character.gun.shoot(self.get_position(), direction);
    }
    fn reloading(&mut self) {
        self.character.gun.reloading_update();
    }
}

impl WithHealth for Player {
    fn get_damage(&mut self, damage: u8) {
        self.character.health.get_damage(damage);
    }
    fn alive(&self) -> bool {
        self.character.health.alive()
    }
}

impl WithId for Player {
    fn get_id(&self) -> u8 {
        self.id
    }
}

impl WithCharacter for Player {
    fn character(&self) -> Character {
        self.character.clone()
    }
}

impl DynPlayer for Player {}

#[derive(Serialize)]
pub struct PlayerInfo {
    pub character: CharacterInfo,
}

impl PlayerInfo {
    pub fn new(player: &Player) -> Self {
        Self {
            character: CharacterInfo::new(&player.character()),
        }
    }

    pub fn without_pos(&mut self) {
        self.character.without_pos();
    }
}

impl Display for PlayerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player Info: {:?}", self.character)
    }
}
