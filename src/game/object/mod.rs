pub mod player;
pub mod enemy;

use tcod::{Color, colors};
use crate::Game;
use super::map::Pos;

/// Can be drawn
pub trait Draw {
    fn draw(&self, con: &mut dyn tcod::Console);
}

/// Can take a turn
pub trait Turn {
    fn turn(&mut self, state: &mut Game);
}

/// Can take damage
pub trait Damage {
    fn take_damage(&mut self, damage: u32);
}

#[derive(Clone)]
pub struct Health {
    pub hp: u32,
    pub ac: u32,
}

impl Damage for Health {
    fn take_damage(&mut self, damage: u32) {
        if damage > self.ac {
            self.hp -= damage - self.ac;
        }
    }
}