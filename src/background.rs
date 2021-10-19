extern crate rogue_sdl;

use sdl2::render::Texture;
use rand::Rng;

const TILE_SIZE: u32 = 64;

pub struct Background<'a> {
	pub texture_0: Texture<'a>,
	pub texture_1: Texture<'a>,
    pub texture_2: Texture<'a>,
	pub x_tiles: i32,
	pub y_tiles: i32,
	pub tiles: Vec<i32>,
}

impl<'a> Background<'a> {
	pub fn new(texture_0: Texture<'a>, texture_1: Texture<'a>, texture_2: Texture<'a>, x_tiles: i32, y_tiles: i32) -> Background<'a> {
		let mut tiles: Vec<i32> = Vec::with_capacity((x_tiles as usize)*(y_tiles as usize));
		Background {
			texture_0,
			texture_1,
            texture_2,
			x_tiles,
			y_tiles,
			tiles,
		}
	}

	pub fn create_map(&mut self, tile_x:i32, tile_y:i32) {
		let mut tiles: Vec<i32> = Vec::with_capacity((tile_x*tile_y) as usize);	// per room tile mapping
		let mut tiles: Vec<i32>
		println!("\n{} * {} = {}", tile_x, tile_y, tiles.capacity());
		self.x_tiles=tile_x;
		self.y_tiles=tile_y;
		
		let mut n = 0;
		for i in 1..(tile_x*tile_y) {
			let num = rand::thread_rng().gen_range(0..2);
			println!("{}",n);
			tiles[n] = num;
			n+=1;
			println!("{}: {}", i, num);
		}
	}
}