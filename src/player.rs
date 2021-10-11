extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::Texture;
const TILE_SIZE: u32 = 64;

pub struct Player<'a> {
	delta: Rect, 
	vel: Rect, 
	pos: Rect,
	src: Rect,
	texture_l: Texture<'a>,
    texture_r: Texture<'a>,
    pub facing_left: bool,
}

impl<'a> Player<'a> {
	pub fn new(pos: Rect, texture_l: Texture<'a>, texture_r: Texture<'a>) -> Player<'a> {
		let delta = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let vel = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        let facing_left = false;
		Player {
			delta, 
			vel, 
			pos,
			src,
			texture_l,
            texture_r,
            facing_left,
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

	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn texture_l(&self) -> &Texture {
		&self.texture_l
	}

    pub fn texture_r(&self) -> &Texture {
        &self.texture_r
    }

    pub fn facing_left(&self) -> &bool {
        &self.facing_left
    }

    pub fn pos(&self) -> Rect {
        self.pos
    }
}