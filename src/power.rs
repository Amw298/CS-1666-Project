extern crate rogue_sdl;

use crate::gamedata::*;

use sdl2::rect::Rect;

pub enum PowerType {
    None,
    Fireball,
    Slimeball,
    Shield,
    Dash,
    Rock,
}

pub struct Power {
    pos: Rect,
    src: Rect,
    power_type: PowerType,
    collected: bool,
    damage: i32, 
}

impl Power {
    pub fn new(pos: Rect, power_type: PowerType) -> Power {
        let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        let collected = false;
        let damage: i32; 
        match power_type {
            PowerType::Rock => { damage = 8; }
            PowerType::Fireball => { damage = 5; }
            PowerType::Slimeball => { damage = 3; }
            _ => { damage = 2; }
        }
        Power {
            pos,
            src,
            power_type,
            collected,
            damage, 
        }
    }

    pub fn x(&self) -> i32 {
        self.pos.x
    }

    pub fn y(&self) -> i32 {
        self.pos.y
    }

    pub fn pos(&self) -> Rect {
        self.pos
    }

    pub fn src(&self) -> Rect {
        self.src
    }

    pub fn collected(&self) -> bool {
        self.collected
    }

    pub fn set_collected(&mut self) {
        self.collected = true;
    }

    pub fn power_type(&self) -> &PowerType {
        &self.power_type
    }
}