extern crate rogue_sdl;

use std::time::Duration;
use std::time::Instant;

use sdl2::image::LoadTexture;
use sdl2::render::WindowCanvas;

use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::rect::Point;

use rogue_sdl::{Game, SDLCore};

const TILE_SIZE: u32 = 64;
const ATTACK_LENGTH: u32 = TILE_SIZE + (TILE_SIZE / 2);
const COOLDOWN: u128 = 250;
const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub struct Player<'a> {
	delta: Rect, 
	vel: Rect, 
	pos: Rect,
	height: u32,
	width: u32,
	src: Rect,
	attack_box: Rect,
	attack_timer: Instant,
	texture_l: Texture<'a>,
    texture_r: Texture<'a>,
	texture_a_l: Texture<'a>,
	texture_a_r: Texture<'a>,
	pub facing_left: bool,
	pub facing_right: bool,
	pub is_still: bool,
	pub hp: f32,
	pub is_attacking: bool,
}

impl<'a> Player<'a> {
	pub fn new(pos: Rect, texture_l: Texture<'a>, texture_r: Texture<'a>, texture_a_l: Texture<'a>, texture_a_r: Texture<'a>) -> Player<'a> {
		let delta = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let vel = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let height = 32;
		let width = 32;
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        let facing_left = false;
		let facing_right = false;
		let is_still = true;
		let hp = 100.0;
		let is_attacking = false;
		let attack_box = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let attack_timer = Instant::now();
		Player {
			delta, 
			vel, 
			pos,
			height,
			width,
			src,
			attack_box,
			attack_timer,
			texture_l,
            texture_r,
			texture_a_l,
			texture_a_r,
            facing_left,
			facing_right,
			is_still,
			hp,
			is_attacking,
		}
	}

	// player x values
	pub fn set_x(&mut self, x:i32){
		self.pos.x = x;
	}
	pub fn x(&self) -> i32 {
		return self.pos.x;
	}
	pub fn set_x_vel(&mut self, x:i32){
		self.vel.x = x;
	}
	pub fn x_vel(&self) -> i32 {
		return self.vel.x;
	}
	pub fn set_x_delta(&mut self, x:i32){
		self.delta.x = x;
	}
	pub fn x_delta(&self) -> i32 {
		return self.delta.x;
	}
	pub fn width(&self) -> u32 {
		self.pos.width()
	}
	
	// player y values
	pub fn set_y(&mut self, y:i32){
		self.pos.y = y;
	}
	pub fn y(&self) -> i32 {
		return self.pos.y;
	}
	pub fn set_y_vel(&mut self, y:i32){
		self.vel.y = y;
	}
	pub fn y_vel(&self) -> i32 {
		return self.vel.y;
	}
	pub fn set_y_delta(&mut self, y:i32){
		self.delta.y = y;
	}
	pub fn y_delta(&self) -> i32 {
		return self.delta.y;
	}
	pub fn height(&self) -> u32 {
		self.pos.height()
	}

	pub fn update_pos(&mut self, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.set_x((self.x() + self.x_vel()).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_y((self.y() + self.y_vel()).clamp(y_bounds.0, y_bounds.1));
	}
	/*
	pub fn draw(&mut self, core: &mut SDLCore, count: &i32, f_display: &i32){
		if *(self.is_still()) {
			if *(self.facing_right()) {
				core.wincan.copy(self.texture_a_r(), self.src(), self.pos()).unwrap();
			} else {
				core.wincan.copy(self.texture_a_l(), self.src(), self.pos()).unwrap();
			}

			//display animation when not moving
			match count {
				count if count < f_display => { self.set_src(0 as i32, 0 as i32); }
				count if count < &(f_display * 2) => { self.set_src(64 as i32, 0 as i32); }
				count if count < &(f_display * 3) => { self.set_src(128 as i32, 0 as i32); }
				count if count < &(f_display * 4) => { self.set_src(0 as i32, 64 as i32); }
				count if count < &(f_display * 5) => { self.set_src(64 as i32, 64 as i32); }
				count if count < &(f_display * 6) => { self.set_src(128 as i32, 64 as i32); }
				count if count < &(f_display * 7) => { self.set_src(0 as i32, 128 as i32); }
				count if count < &(f_display * 8) => { self.set_src(64 as i32, 128 as i32); }
				count if count < &(f_display * 9) => { self.set_src(128 as i32, 128 as i32); }
				count if count < &(f_display * 10) => { self.set_src(0 as i32, 192 as i32); }
				count if count < &(f_display * 11) => { self.set_src(64 as i32, 192 as i32); }
				count if count < &(f_display * 12) => { self.set_src(128 as i32, 192 as i32); }


				_ => { self.set_src(0, 0); }
			}
		} else {
			self.set_src(0, 0);
			if *(self.facing_right()) {
				core.wincan.copy(self.texture_r(), self.src(), self.pos()).unwrap();
			} else {
				core.wincan.copy(self.texture_l(), self.src(), self.pos()).unwrap();
			}
		}
	}
	*/
	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn texture_l(&self) -> &Texture {
		&self.texture_l
	}

    pub fn texture_r(&self) -> &Texture {
        &self.texture_r
    }

    /*pub fn facing_left(&self) -> &bool {
        &self.facing_left
    }*/

	pub fn facing_right(&self) -> &bool {
        &self.facing_right
    }

    pub fn pos(&self) -> Rect {
        self.pos
    }

	pub fn texture_a_l(&self) -> &Texture {
        &self.texture_a_l
    }

	pub fn texture_a_r(&self) -> &Texture {
        &self.texture_a_r
    }

	pub fn is_still(&self) -> &bool {
        &self.is_still
    }

	pub fn is_attacking(&self) -> &bool {
		&self.is_attacking
	}

	pub fn get_attack_timer(&self) -> u128 {
		self.attack_timer.elapsed().as_millis()
	}

	pub fn get_attack_box(&self) -> Rect {
		self.attack_box
	}

	pub fn set_attack_box(&mut self, x: i32, y: i32) {
		if *self.facing_right()
		{
			self.attack_box = Rect::new(x + TILE_SIZE as i32, y as i32, ATTACK_LENGTH, TILE_SIZE);
		} else {
			self.attack_box = Rect::new(x - ATTACK_LENGTH as i32, y as i32, ATTACK_LENGTH, TILE_SIZE);
		}
	}

	pub fn clear_attack_box(&mut self) {
		self.attack_box = Rect::new(self.x() as i32, self.y() as i32, 0, 0);
	}

	pub fn get_cooldown(&self) -> u128 {
		COOLDOWN
	}

	pub fn set_src(&mut self, x: i32, y: i32) {
		self.src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
	}

	pub fn get_hp(&self) -> f32 {
		return self.hp
	}

	pub fn is_dead(&self) -> bool {
		return self.hp <= 0.0;
	}

	pub fn minus_hp(&mut self, dmg: f32) {
		self.hp -= dmg;
	}

	pub fn attack(&mut self) {
		if(self.get_attack_timer() < COOLDOWN)
		{
			return;
		}
		self.is_attacking = true;
		if *self.facing_right()
		{
			self.attack_box = Rect::new(self.x() + TILE_SIZE as i32, self.y() as i32, ATTACK_LENGTH, TILE_SIZE);
		} else {
			self.attack_box = Rect::new(self.x() - ATTACK_LENGTH as i32, self.y() as i32, ATTACK_LENGTH, TILE_SIZE);
		}
		self.attack_timer = Instant::now();
	}

	pub fn cooldown(&mut self) {
		self.is_attacking = false;
		self.clear_attack_box();
	}

	/*pub fn base_attack(&mut self, x: i32, y: i32) {
		self.is_attacking = true;
		// create hitbox with set width and length between player(x,y) and clickpoint(x,y)
		self.attack_box = new Rect::from_center(
			Point::new((self.x() + x)/2, (self.y() + y)/2),
			(self.x() - x).abs(),
			(self.y() - y).abs());
		println!("Attacked!");
	}*/
}