use super::{
    character::{Bullet, Character},
    direction::Direction,
    pos::Pos,
};

pub trait Movable {
    fn get_position(&self) -> Pos;
    fn shift(&mut self, direction: Direction);
    fn get_speed(&self) -> u8;
    fn get_direction(&self) -> Direction {
        Direction::None
    }
}

pub trait WithCharacter {
    fn character(&self) -> &Character;
}

pub trait Attack {
    fn attack(&mut self, direction: Direction) -> Option<Bullet>;
    fn reloading(&mut self);
}

pub trait WithHealth {
    fn get_damage(&mut self, damage: u8);
    fn alive(&self) -> bool;
}

pub trait WithId {
    fn get_id(&self) -> u8;
}

pub trait IntoBytes<const N: usize> {
    fn into_bytes(&self) -> [u8; N];
}
