use std::{fmt::Display, u8};

use ez_colorize::ColorizeDisplay;
use serde::Serialize;

use super::{direction::Direction, pos::Pos, traits::Movable};
#[derive(Debug, Clone)]
pub struct Character {
    pub speed: u8,
    pub pos: Pos,
    pub health: Health,
    pub gun: Gun,
}

#[derive(Serialize, Debug)]
pub struct CharacterInfo {
    pub health: u8,
    pub gun_reloading_await: u8,
    pub bullet_damage: u8,
    pub bullet_range: u8,
    pub pos: Option<Pos>,
}

impl CharacterInfo {
    pub fn new(c: &Character) -> Self {
        Self {
            health: c.health.current,
            gun_reloading_await: c.gun.reload_awaiting,
            bullet_damage: c.gun.bullet.damage,
            bullet_range: c.gun.bullet.range,
            pos: Some(c.pos),
        }
    }

    pub fn without_pos(&mut self) {
        self.pos = None;
    }
}

impl Character {
    pub const fn new(speed: u8, pos: Pos, health: Health, gun: Gun) -> Self {
        Self {
            speed,
            pos,
            health,
            gun,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Health {
    pub max: u8,
    pub current: u8,
}

impl Health {
    pub const fn new(max: u8) -> Self {
        Self { max, current: max }
    }

    pub fn alive(&self) -> bool {
        self.current > 0
    }

    pub fn get_damage(&mut self, damage: u8) {
        if self.current > damage {
            self.current -= damage;
        } else {
            self.current = 0;
        }
    }
}
#[derive(Debug, Clone)]
pub struct Gun {
    reload_time: u8,
    reload_awaiting: u8,
    bullet: Bullet,
}

impl Gun {
    pub const fn new(reloading: u8, bullet: Bullet) -> Self {
        Self {
            reload_time: reloading,
            reload_awaiting: 0,
            bullet,
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.reload_awaiting == 0
    }

    pub fn reloading_update(&mut self) {
        if self.can_shoot() {
            return;
        }
        self.reload_awaiting -= 1;
    }

    pub fn shoot(&mut self, from: Pos, direction: Direction) -> Option<Bullet> {
        if self.can_shoot() {
            self.reload_awaiting = self.reload_time;
            let mut bullet = self.bullet.clone();
            bullet.direction = direction;
            if let Some(pos) = from.shift(direction) {
                bullet.pos = pos;
                return Some(bullet);
            }
        }
        return None;
    }
}

#[derive(Clone, Debug)]
pub struct Bullet {
    pos: Pos,
    pub range: u8,
    pub direction: Direction,
    pub damage: u8,
}

impl Bullet {
    pub fn new(range: u8, damage: u8) -> Self {
        Self {
            pos: (0, 0).into(),
            range,
            direction: Direction::None,
            damage,
        }
    }

    pub fn use_up(&mut self) -> u8 {
        let dmg = self.damage;
        self.used();
        dmg
    }

    fn used(&mut self) {
        self.range = 0;
        self.damage = 0;
        self.direction = Direction::None;
    }

    pub fn can_move(&self) -> bool {
        self.range > 0
    }
}

impl Movable for Bullet {
    fn get_position(&self) -> Pos {
        self.pos
    }

    fn shift(&mut self, direction: Direction) {
        if self.can_move() {
            if let Some(new_pos) = self.pos.shift(direction) {
                self.range -= 1;
                self.pos = new_pos;
                return;
            }
            self.range = 0;
        }
    }

    fn get_speed(&self) -> u8 {
        u8::MAX
    }

    fn get_direction(&self) -> Direction {
        self.direction
    }
}

impl super::traits::Movable for Character {
    fn get_position(&self) -> Pos {
        self.pos
    }

    fn shift(&mut self, direction: super::direction::Direction) {
        if let Some(new_pos) = self.pos.shift(direction) {
            self.pos = new_pos
        }
    }

    fn get_speed(&self) -> u8 {
        self.speed
    }
}

impl Display for Gun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Gun| attack await: {}/{}, {}]",
            self.reload_awaiting.red(),
            self.reload_time.green(),
            self.bullet
        )
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}, [Speed: {}]]",
            self.health,
            self.gun,
            self.pos,
            self.speed.green()
        )
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Hlth: {}/{}]", self.current.red(), self.max.green())
    }
}

impl Display for Bullet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::None => {
                write!(
                    f,
                    "[Bullet| dmg: {}, range: {}]",
                    self.damage.green(),
                    self.range.green()
                )
            }
            _ => {
                write!(
                    f,
                    "[Bullet| dmg: {}, direct: {}, range: {}, pos: {}]",
                    self.damage.green(),
                    self.direction.green(),
                    self.range.green(),
                    self.pos
                )
            }
        }
    }
}
