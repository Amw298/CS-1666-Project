extern crate rogue_sdl;
use rogue_sdl::{Game, SDLCore};
//use sdl2::audio::AudioSpecDesired;
//use sdl2::audio::AudioSpecWAV;
//use sdl2::audio::AudioCVT;
//use sdl2::audio::AudioCallback;
use std::time::Duration;
use std::time::Instant;
//use std::cmp;
use std::collections::HashSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState};
use sdl2::rect::{Rect, Point};
use sdl2::image::LoadTexture;
use sdl2::render::{Texture};//,TextureCreator};
use rand::Rng;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
//use std::env;
use std::path::Path;
mod background;
mod credits;
mod enemy;
mod gamedata;
mod gold;
mod power;
mod player;
mod projectile;
mod room;
mod map;
mod ui;
mod crateobj;
mod rigidbody;

use crate::gamedata::*;
use crate::background::*;
use crate::player::*;
use crate::enemy::*;
use crate::projectile::*;
use crate::power::*;
use crate::map::*;
use crate::crateobj::*;
use crate::gold::*;

pub struct ROGUELIKE {
	core: SDLCore,
	game_data: GameData,
}

// CREATE GAME
impl Game for ROGUELIKE  {

	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		let game_data = GameData::new();
		Ok(ROGUELIKE{ core, game_data, })
	}

	fn run(&mut self) -> Result<(), String> {
		// CREATE GAME CONSTANTS
        let texture_creator = self.core.wincan.texture_creator();
		let mut rng = rand::thread_rng();

		// AUDIO SYSTEM
		let frequency = 44_100;
		let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
		let channels = DEFAULT_CHANNELS; // Stereo
		let chunk_size = 1_024;
		sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
		let _mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;
		// Number of mixing channels available for sound effect `Chunk`s to play simultaneously.
		sdl2::mixer::allocate_channels(4);
	
		if DEBUG {
			let n = sdl2::mixer::get_chunk_decoders_number();
			println!("available chunk(sample) decoders: {}", n);
			for i in 0..n {
				println!("  decoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
			}
			println!("available music decoders: {}", n);
			let n = sdl2::mixer::get_music_decoders_number();
			for i in 0..n {
				println!("  decoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
			}
			println!("query spec => {:?}", sdl2::mixer::query_spec());
		}

		let path = Path::new("./music/Rampage.wav");
		let music = sdl2::mixer::Music::from_file(path)?;
		music.play(1)?;

		// CREATE PLAYER SHOULD BE MOVED TO player.rs
		// create player 
		let mut player = player::Player::new(
			(CENTER_W as f64, CENTER_H as f64),
			texture_creator.load_texture("images/player/slime_sheet.png")?,
		);
		// create ui
		let mut ui = ui::UI::new(
			Rect::new(
				(10) as i32 *(TILE_SIZE_64 as f64 *1.2) as i32,
				(CAM_H-(TILE_SIZE_64 as f64 *1.2) as u32) as i32,
				(TILE_SIZE_64 as f64 *1.2) as u32,
				(TILE_SIZE_64 as f64 *1.2) as u32,
			), 
			texture_creator.load_texture("images/ui/heart.png")?,
		);
		// LOAD TEXTURES
		// projectile textures
		let mut ability_textures: Vec<Texture> = Vec::<Texture>::with_capacity(5);
		let bullet = texture_creator.load_texture("images/abilities/bullet.png")?; 
		let enemy_bullet = texture_creator.load_texture("images/abilities/enemy_bullet.png")?;
		let fireball = texture_creator.load_texture("images/abilities/old_fireball.png")?;
		let shield = texture_creator.load_texture("images/abilities/shield.png")?;
		
		ability_textures.push(bullet);
		ability_textures.push(fireball);
		ability_textures.push(enemy_bullet);
		ability_textures.push(shield);

		// object textures
		let mut crate_textures: Vec<Texture> = Vec::<Texture>::with_capacity(5);
		let crate_texture = texture_creator.load_texture("images/objects/crate.png")?; 
		crate_textures.push(crate_texture);
		
		let coin_texture = texture_creator.load_texture("images/ui/gold_coin.png")?;
		let fireball_texture = texture_creator.load_texture("images/abilities/fireball_pickup.png")?;
		let slimeball_texture = texture_creator.load_texture("images/abilities/slimeball_pickup.png")?;
		let shield_texture = texture_creator.load_texture("images/abilities/shield_pickup.png")?;
		let sword_texture = texture_creator.load_texture("images/player/sword_l.png")?;
		let health_texture = texture_creator.load_texture("images/ui/heart.png")? ;
		let health_upgrade_texture = texture_creator.load_texture("images/ui/heart.png")? ;

		// MAIN GAME LOOP
		'gameloop: loop {
			// CREATE MAPS
			let background = background::Background::new(
				texture_creator.load_texture("images/background/bb.png")?,
				texture_creator.load_texture("images/background/floor_tile_1.png")?,
				texture_creator.load_texture("images/background/floor_tile_2.png")?,
				texture_creator.load_texture("images/background/tile.png")?,
				texture_creator.load_texture("images/background/skull.png")?,
				texture_creator.load_texture("images/background/upstairs.png")?,
				texture_creator.load_texture("images/background/downstairs.png")?,
				self.game_data.rooms[self.game_data.current_room].xwalls,
				self.game_data.rooms[self.game_data.current_room].ywalls,
				Rect::new(
					(0 + ((TILE_SIZE_CAM / 2) as i32)) - ((CAM_W / 2) as i32),
					(0 + ((TILE_SIZE_CAM / 2) as i32)) - ((CAM_H / 2) as i32),
					CAM_W,
					CAM_H,
				),
			);
			let mut map_data = map::Map::new(self.game_data.current_floor, background);
			map_data.create_map();

			// set starting position
			player.set_x((map_data.starting_position.0 as i32 * TILE_SIZE as i32 - (CAM_W - 2*TILE_SIZE_PLAYER) as i32 / 2) as f64);
			player.set_y((map_data.starting_position.1 as i32 * TILE_SIZE as i32 - (CAM_H - 2*TILE_SIZE_PLAYER) as i32 / 2) as f64);

			// reset arrays
			self.game_data.crates = Vec::<Crate>::with_capacity(0);
			self.game_data.dropped_powers = Vec::<Power>::with_capacity(0);
			self.game_data.gold = Vec::<Gold>::with_capacity(0);
			self.game_data.player_projectiles = Vec::<Projectile>::with_capacity(0);
			self.game_data.enemy_projectiles = Vec::<Projectile>::with_capacity(0);
			// OBJECT GENERATION
			if DEVELOP {
				let pos = Rect::new(
					player.x() as i32 -200 + rng.gen_range(1..10),
					player.y() as i32 -200 + rng.gen_range(0..10),
					TILE_SIZE,
					TILE_SIZE
				);
				self.game_data.crates.push(crateobj::Crate::new(pos));
			}

			// create enemies
			let mut enemies: Vec<Enemy> = Vec::new();
			let mut rngt = Vec::new();

			let mut enemy_count = 0;
			let max_h = MAP_SIZE_H + ((self.game_data.current_floor-1)*30) as usize; 
			let max_w = MAP_SIZE_W + ((self.game_data.current_floor-1)*30) as usize;
			for h in 0..max_h {
				for w in 0..max_w {
					if map_data.enemy_and_object_spawns[h][w] == 0 {
						continue;
					}
					match map_data.enemy_and_object_spawns[h][w] {
						1 => {
							let e = enemy::Enemy::new(
								Rect::new(
									w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
									h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
									TILE_SIZE_CAM,
									TILE_SIZE_CAM
								),
								texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
								EnemyType::Melee,
								enemy_count,
							);
							enemies.push(e);
							rngt.push(rng.gen_range(1..5));
							enemy_count += 1;
						}
						2 => {
							let e = enemy::Enemy::new(
								Rect::new(
									w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
									h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
									TILE_SIZE_CAM,
									TILE_SIZE_CAM
								),
								texture_creator.load_texture("images/enemies/ranged_enemy.png")?,
								EnemyType::Ranged,
								enemy_count,
							);
							enemies.push(e);
							rngt.push(rng.gen_range(1..5));
							enemy_count += 1;
						}
						3 => {
							let c = crateobj::Crate::new(
								Rect::new(
									w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) /2,
									h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) /2,
									TILE_SIZE_CAM,
									TILE_SIZE_CAM
								)
							);
							self.game_data.crates.push(c);
						}
						4 => {
							let e = enemy::Enemy::new(
								Rect::new(
									w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
									h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
									TILE_SIZE_CAM,
									TILE_SIZE_CAM
								),
								texture_creator.load_texture("images/enemies/Shield_skeleton.png")?,
								EnemyType::Skeleton,
								enemy_count,
							);
							enemies.push(e);
							rngt.push(rng.gen_range(1..5));
							enemy_count += 1;
						}
						_ => {}
					}
				}
			}

			let mut all_frames = 0;
			let last_time = Instant::now();

			// INDIVIDUAL LEVEL LOOP
			'level: loop {
				for event in self.core.event_pump.poll_iter() {
					match event {
						Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
						_ => {},
					}
				}
				// fps calculations
				let mut fps_avg: f64 = 60.0; 
				all_frames += 1;
				let elapsed = last_time.elapsed();
				if elapsed > Duration::from_secs(1) {
					fps_avg = (all_frames as f64) / elapsed.as_secs_f64();
					self.game_data.set_speed_limit(fps_avg.recip() * SPEED_LIMIT);
					self.game_data.set_accel_rate(fps_avg.recip() * ACCEL_RATE);
				}
				// reset frame values
				player.set_x_delta(0);
				player.set_y_delta(0);
				self.core.wincan.copy(&map_data.background.black, None, None)?;

				// GET INPUT
				let mousestate= self.core.event_pump.mouse_state();
				let keystate: HashSet<Keycode> = self.core.event_pump
					.keyboard_state()
					.pressed_scancodes()
					.filter_map(Keycode::from_scancode)
					.collect();
				if keystate.contains(&Keycode::E){
					let mpos = Rect::new(map_data.ending_position.0 as i32 * TILE_SIZE as i32 - (CAM_W - TILE_SIZE) as i32 / 2, 
					map_data.ending_position.1 as i32 * TILE_SIZE as i32 - (CAM_H - TILE_SIZE) as i32 / 2, 
					TILE_SIZE, TILE_SIZE);
					let ppos = Rect::new(player.x() as i32, player.y() as i32, TILE_SIZE_CAM, TILE_SIZE_CAM);
					if check_collision(&ppos, &mpos) {
						println!("c: {} {}", player.x(), player.y());
						println!("c: {} {}", mpos.x, mpos.y);
						break 'level
					}
				}
				ROGUELIKE::check_inputs(self, &keystate, mousestate, &mut player, fps_avg, &mut map_data)?;

				// UPDATE BACKGROUND
				ROGUELIKE::draw_background(self, &player, &mut map_data.background, map_data.map)?;

				// UPDATE PLAYER
				player.update_player(&self.game_data, map_data.map, &mut self.core)?;
				ROGUELIKE::draw_player(self, fps_avg, &mut player, map_data.background.get_curr_background());

				// UPDATE ENEMIES
				rngt = ROGUELIKE::update_enemies(self, &mut rngt, &mut enemies, &player,map_data.map);
				// UPDATE ATTACKS
				// Should be switched to take in array of active fireballs, bullets, etc.
				ROGUELIKE::update_projectiles(&mut self.game_data.player_projectiles, &mut self.game_data.enemy_projectiles);
				ROGUELIKE::draw_enemy_projectile(self, &ability_textures, &player);	
				ROGUELIKE::draw_player_projectile(self, &ability_textures,  &player, mousestate)?;	
				ROGUELIKE::draw_weapon(self, &player, &sword_texture);
				
				// UPDATE INTERACTABLES
				// function to check explosive barrels stuff like that should go here. placed for ordering.
				ROGUELIKE::update_drops(self, &mut enemies, &mut player, &mut map_data, &coin_texture,
										&fireball_texture, &slimeball_texture, &shield_texture, 
										&health_texture, &health_upgrade_texture);

				// CHECK COLLISIONS
				ROGUELIKE::check_collisions(self, &mut player, &mut enemies, &mut map_data, &crate_textures);
				if player.is_dead(){break 'gameloop;}

				// UPDATE UI
				ui.update_ui(&player, &mut self.core, &map_data)?;
				
				// UPDATE FRAME
				self.core.wincan.present();
			}
		}
		self.game_data.current_floor += 1;
		// Out of game loop, return Ok
		Ok(()) 
	}
}

pub fn main() -> Result<(), String> {
    rogue_sdl::runner(TITLE, ROGUELIKE::init);
	//credits::run_credits()
	Ok(())
}

// check collision
fn check_collision(a: &Rect, b: &Rect) -> bool {
	if a.bottom() < b.top()
		|| a.top() > b.bottom()
		|| a.right() < b.left()
		|| a.left() > b.right()
	{
		false
	}
	else {
		true
	}
}

// Create map
impl ROGUELIKE {
	// draw background
	pub fn draw_background(&mut self, player: &Player, background: &mut Background, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();
		let floor = texture_creator.load_texture("images/background/floor_tile_1.png")?;
		let shop = texture_creator.load_texture("images/background/floor_tile_maroon.png")?;
		let tile = texture_creator.load_texture("images/background/tile.png")?;
		let moss_tile = texture_creator.load_texture("images/background/moss_tile.png")?;
		let upstairs = texture_creator.load_texture("images/background/upstairs.png")?;
		let downstairs = texture_creator.load_texture("images/background/downstairs.png")?;
		background.set_curr_background(player.x(), player.y(), player.width(), player.height());

		let h_bounds_offset = (player.y() / TILE_SIZE as f64) as i32;
		let w_bounds_offset = (player.x() / TILE_SIZE as f64) as i32;
	
		if !DEVELOP {
			for h in 0..(CAM_H / TILE_SIZE) + 1 {
				for w in 0..(CAM_W / TILE_SIZE) + 1 {
					let src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64);
					let pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (player.x() % TILE_SIZE as f64) as i32,
										(h as i32 + 0 as i32) * TILE_SIZE as i32 - (player.y() % TILE_SIZE as f64) as i32,
										TILE_SIZE, TILE_SIZE);
					if h as i32 + h_bounds_offset < 0 ||
					   w as i32 + w_bounds_offset < 0 ||
					   h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
					   w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
					   map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
						continue;
					} else{
						let num = map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize];
						match num {
							1 => { self.core.wincan.copy_ex(&floor, src, pos, 0.0, None, false, false).unwrap(); }, 		// floor tiles
							2 => { self.core.wincan.copy_ex(&tile, src, pos, 0.0, None, false, false).unwrap(); },  		// tile tiles
							5 => { self.core.wincan.copy_ex(&moss_tile, src, pos, 0.0, None, false, false).unwrap(); },  		// tile tiles
							6 => { self.core.wincan.copy_ex(&shop, src, pos, 0.0, None, false, false).unwrap(); },  	// shop tile
							3 => { self.core.wincan.copy_ex(&upstairs, src, pos, 0.0, None, false, false).unwrap(); },  	// upstairs tile
							_ => { self.core.wincan.copy_ex(&downstairs, src, pos, 0.0, None, false, false).unwrap(); },  	// downstairs tile
						}
					}					
				}
			}
		} else {
			let tiles = &self.game_data.rooms[self.game_data.current_room].tiles;
			let mut n = 0;
			for i in 0..self.game_data.rooms[0].xwalls.1+1 {
				for j in 0..self.game_data.rooms[0].ywalls.1+1 {
					if tiles[n].0 {
						let t = background.get_tile_info(tiles[n].1, i, j, player.x(), player.y());
						self.core.wincan.copy_ex(t.0, t.1, t.2, 0.0, None, false, false).unwrap();
					}
					n+=1;
				}
			}
		}
		Ok(())
	}
	
	// update enemies
	pub fn update_enemies(&mut self, rngt: &mut Vec<i32>, enemies: &mut Vec<Enemy>, player: &Player,map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> Vec<i32> {
		let mut i = 0;
		for enemy in enemies {
			if enemy.is_alive(){
				enemy.check_attack(&mut self.game_data, (player.x(), player.y()));
				// direction changer
				if self.game_data.frame_counter.elapsed().as_millis() % 120 as u128 == 0 as u128 || enemy.force_move(map) { 
					rngt[i] = rand::thread_rng().gen_range(1..5);
				}
				let t = enemy.update_enemy(&self.game_data, rngt, i, (player.x(), player.y()), map);
				self.core.wincan.copy_ex(enemy.txtre(), enemy.src(), t, 0.0, None, enemy.facing_right, false).unwrap();
				i += 1;
			}
		}
		return rngt.to_vec();
	}

	pub fn update_crates(&mut self, crate_textures: &Vec<Texture>, player: &Player, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]){
		for c in self.game_data.crates.iter_mut(){
			c.update_crates( &mut self.core, crate_textures, player, map);
		}
	}
	
	pub fn update_drops(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, map_data: &mut Map, coin_texture: &Texture,
						fireball_texture: &Texture, slimeball_texture: &Texture, shield_texture: &Texture, 
						health_texture: &Texture, health_upgrade_texture: &Texture) {
		//add enemy drops to game
		for enemy in enemies {
			if !enemy.is_alive() && enemy.has_item() {
				if enemy.has_coin() { self.game_data.gold.push(enemy.drop_coin()); }
				self.game_data.dropped_powers.push(enemy.drop_power());
			}
		}
		// draw uncollected coins
		for coin in self.game_data.gold.iter_mut() {
			if !coin.collected() {
				let pos = Rect::new(coin.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
									coin.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE, TILE_SIZE);
				self.core.wincan.copy_ex(&coin_texture, coin.src(), pos, 0.0, None, false, false).unwrap();
			}
		}
		// draw powers
		for power in self.game_data.dropped_powers.iter_mut() {
			if !power.collected() {
				let pos = Rect::new(power.x() as i32 + (CENTER_W - player.x() as i32),
									power.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE, TILE_SIZE);
				match power.power_type() {
					PowerType::Fireball => {
						self.core.wincan.copy_ex(&fireball_texture, power.src(), pos, 0.0, None, false, false).unwrap();
					},
					PowerType::Slimeball => {
						self.core.wincan.copy_ex(&slimeball_texture, power.src(), pos, 0.0, None, false, false).unwrap();
					},
					PowerType::Shield => {
						self.core.wincan.copy_ex(&shield_texture, power.src(), pos, 0.0, None, false, false).unwrap();
					},
					_ => {},
				}
			}
		}
		// draw shop items
		let mut i=0; 
		while i< map_data.shop_spawns.len() {
			let src = Rect::new(0,0,TILE_SIZE_64,TILE_SIZE_64); 
			let pos = Rect::new((map_data.shop_spawns[i].1 as i32) * TILE_SIZE as i32 - player.x() as i32,
								(map_data.shop_spawns[i].0 as i32) * TILE_SIZE as i32 - player.y() as i32,
								TILE_SIZE, TILE_SIZE);
			match map_data.shop_items[i].0 {
				ShopItems::Fireball => {
					self.core.wincan.copy_ex(&fireball_texture, src, pos, 0.0, None, false, false).unwrap();
				},
				ShopItems::Slimeball => {
					self.core.wincan.copy_ex(&slimeball_texture, src, pos, 0.0, None, false, false).unwrap();
				},
				ShopItems::Shield => {
					self.core.wincan.copy_ex(&shield_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::HealthUpgrade => {
					self.core.wincan.copy_ex(&health_upgrade_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::Health => {
					self.core.wincan.copy_ex(&health_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				_ => {}
			}
			i+=1; 
		}
	}

	// check input values
	pub fn check_inputs(&mut self, keystate: &HashSet<Keycode>, mousestate: MouseState, mut player: &mut Player, fps_avg: f64, map_data: &mut Map)-> Result<(), String>  {
		// move up
		if keystate.contains(&Keycode::W) {
			player.set_y_delta(player.y_delta() - self.game_data.get_accel_rate() as i32);
		}
		// move left
		if keystate.contains(&Keycode::A) {
			player.set_x_delta(player.x_delta() - self.game_data.get_accel_rate() as i32);
			player.facing_right = false;
		}
		// move down
		if keystate.contains(&Keycode::S) {
			player.set_y_delta(player.y_delta() + self.game_data.get_accel_rate() as i32);
		}
		// move right
		if keystate.contains(&Keycode::D) {
			player.set_x_delta(player.x_delta() + self.game_data.get_accel_rate() as i32);
			player.facing_right = true;
		}
		// basic attack
		if keystate.contains(&Keycode::Space) {
			if !(player.get_attacking()) {
				player.attack();
			}
		}
		// Shoot ranged attack
		if mousestate.left(){
			match player.get_power() {
				PowerType::Fireball => {
					if !player.is_firing && player.get_mana() >= 1 {
						let now = Instant::now();
						let elapsed = now.elapsed().as_millis() / (fps_avg as u128 * 2 as u128); // the bigger this divisor is, the faster the animation plays

						let bullet = player.fire(mousestate.x(), mousestate.y(), self.game_data.get_speed_limit(), ProjectileType::Fireball, elapsed);
						self.game_data.player_projectiles.push(bullet);
					}
				},
				PowerType::Slimeball => {
					if !player.is_firing && player.get_mana() >= 2 {
						let bullet = player.fire(mousestate.x(), mousestate.y(), self.game_data.get_speed_limit(), ProjectileType::Bullet, 0);
						self.game_data.player_projectiles.push(bullet);
					}
				},
				PowerType::Shield => {
					if !player.get_shielded() && player.get_mana() >= 3 {
						player.set_shielded(true);
						// code for placeable shield. 
						//let bullet = player.fire(player.x() as i32, player.y() as i32, 0.0, ProjectileType::Shield, 0);
						//self.game_data.player_projectiles.push(bullet);
					}
				},
				_ => {},
			}
		}
		// Absorb power
		if keystate.contains(&Keycode::E) {
			if player.can_pickup() {
				for drop in self.game_data.dropped_powers.iter_mut() {
					if check_collision(&player.pos(), &drop.pos()) &&
					   !drop.collected() && player.get_pickup_timer() > 1000 {
						drop.set_collected();
						player.reset_pickup_timer();
						match drop.power_type() {
							PowerType::Fireball => {
								player.set_power(PowerType::Fireball);
							},
							PowerType::Slimeball => {
								player.set_power(PowerType::Slimeball);
							},
							PowerType::Shield => {
								player.set_power(PowerType::Shield);
							}
							_ => {}
						}
						break;
					}
				}
				let mut i=0; 
				while i< map_data.shop_spawns.len() {
					let pos = Rect::new((map_data.shop_spawns[i].1 as i32) * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
										(map_data.shop_spawns[i].0 as i32) * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
										TILE_SIZE, TILE_SIZE);
					if check_collision(&player.pos(), &pos) && player.get_pickup_timer() > 1000 && 
					   (player.get_coins() >= map_data.shop_items[i].2 || map_data.shop_items[i].1 == true ) {
						player.reset_pickup_timer();
						if !map_data.shop_items[i].1 { 
							player.sub_coins(map_data.shop_items[i].2); 
						}
						match map_data.shop_items[i].0 {
							ShopItems::Fireball => {
								player.set_power(PowerType::Fireball);
								map_data.shop_items[i].1 = true; 
							},
							ShopItems::Slimeball => {
								player.set_power(PowerType::Slimeball);
								map_data.shop_items[i].1 = true; 
							},
							ShopItems::Shield => {
								player.set_power(PowerType::Shield);
								map_data.shop_items[i].1 = true; 
							}
							ShopItems::HealthUpgrade => {
								if map_data.shop_items[i].1 == false {
									player.upgrade_hp(10); 
									player.plus_hp(10); 
									map_data.shop_items[i].1 = true; 
								} 
							}
							ShopItems::Health => {
								player.plus_hp(10); 
							}
							_ => { }
						}
						break;
					}
					i+=1; 
				}
			}
		}
		// Go to next level
		if keystate.contains(&Keycode::E){
			let mpos = Rect::new(map_data.ending_position.0 as i32 * TILE_SIZE as i32 - (CAM_W - TILE_SIZE) as i32 / 2, 
								 map_data.ending_position.1 as i32 * TILE_SIZE as i32 - (CAM_H - TILE_SIZE) as i32 / 2, 
								 TILE_SIZE, TILE_SIZE);
			let ppos = Rect::new(player.x() as i32, player.y() as i32, TILE_SIZE, TILE_SIZE);
			if check_collision(&ppos, &mpos) {
				println!("c: {} {}", player.x(), player.y());
				println!("c: {} {}", mpos.x, mpos.y);
			}
		}
		// FOR TESTING ONLY: USE TO FOR PRINT VALUES
		if keystate.contains(&Keycode::P) {
			//println!("\nx:{} y:{} ", enemies[0].x() as i32, enemies[0].y() as i32);
			//println!("{} {} {} {}", enemies[0].x() as i32, enemies[0].x() as i32 + (enemies[0].width() as i32), enemies[0].y() as i32, enemies[0].y() as i32 + (enemies[0].height() as i32));
			println!("{} {}", player.x(), player.y());	
			for item in map_data.shop_spawns.iter() {
				let pos = Rect::new((item.1 as i32) * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
									(item.0 as i32) * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
									TILE_SIZE, TILE_SIZE);
				println!("{},{}", pos.x, pos.y);
			}
		}
		Ok(())	
	}
	// update projectiles
	pub fn update_projectiles(player_projectiles: &mut Vec<Projectile>, enemy_projectiles: &mut Vec<Projectile>) {
		for projectile in player_projectiles {
			if projectile.is_active() {
				projectile.update_pos();
			}
		}
		for projectile in enemy_projectiles {
			if projectile.is_active() {
				projectile.update_pos();

			}
		}
	}
	
	// check collisions
	fn check_collisions(&mut self, player: &mut Player, enemies: &mut Vec<Enemy>, map_data: &mut Map, crate_textures: &Vec<Texture>) {
		let map = map_data.map; 	
		for enemy in enemies {
			if !enemy.is_alive() {
				continue;
			}

			// player collision
			if check_collision(&player.pos(), &enemy.pos()) {
				player.minus_hp(5);
			}

			// player projectile collisions
			for projectile in self.game_data.player_projectiles.iter_mut() {
				if check_collision(&projectile.pos(), &enemy.pos())  && projectile.is_active() {
					match enemy.enemy_type {
						EnemyType::Melee =>{
							enemy.projectile_knockback(projectile.x_vel(), projectile.y_vel());
							enemy.minus_hp(projectile.damage);
						}
						EnemyType::Ranged =>{
							enemy.projectile_knockback(projectile.x_vel(), projectile.y_vel());
							enemy.minus_hp(projectile.damage);
						}
						EnemyType::Skeleton=>{}
					}
					projectile.die();
				}
			}

			// player melee collisions
			if player.get_attacking() {
				if check_collision(&player.get_attack_box(), &enemy.pos()) {
					enemy.knockback(player.x().into(), player.y().into());
					enemy.minus_hp(2);
				}
			}
		
			// enemy projectile collisions
			for projectile in self.game_data.enemy_projectiles.iter_mut() {
				if check_collision(&projectile.pos(), &player.pos()) && projectile.is_active() {
					player.minus_hp(5);
					projectile.die();
				}
			}

			// check crate collisions
			for c in self.game_data.crates.iter_mut() {
				if check_collision(&c.pos(), &enemy.pos()) && c.get_magnitude() != 0.0{
					enemy.projectile_knockback(c.x_vel(), c.y_vel());
				}
			}
		}

		for projectile in self.game_data.player_projectiles.iter_mut() {
			projectile.check_bounce(&mut self.game_data.crates, &mut Vec::new(), map);
		}
		for projectile in self.game_data.enemy_projectiles.iter_mut() {
			projectile.check_bounce(&mut self.game_data.crates, &mut self.game_data.player_projectiles, map);
		}
		for coin in self.game_data.gold.iter_mut() {
			if check_collision(&player.pos(), &coin.pos()) {
				if !coin.collected() {
					coin.set_collected();
					player.add_coins(coin.get_gold());
				}
			}
		}
		let mut can_pickup = false;
		for drop in self.game_data.dropped_powers.iter_mut() {
			if check_collision(&player.pos(), &drop.pos()) {
				if !drop.collected() {
					match drop.power_type() {
						PowerType::None => {},
						_ => {
							can_pickup = true;
						}
					}
				}
			}
		}
		let mut i=0; 
		while i< map_data.shop_spawns.len() {
			let pos = Rect::new((map_data.shop_spawns[i].1 as i32) * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
								(map_data.shop_spawns[i].0 as i32) * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
								TILE_SIZE, TILE_SIZE);
			if check_collision(&player.pos(), &pos) && player.get_pickup_timer() > 1000 {
				match map_data.shop_items[i].0 {
					ShopItems::None => { }
					_ => { can_pickup = true; }
				}
				break;
			}
			i += 1; 
		}
		player.set_can_pickup(can_pickup);
		//check collision between crates and player
		for c in self.game_data.crates.iter_mut(){
			if check_collision(&player.pos(), &c.pos()){
				// provide impulse
				c.update_velocity(player.x_vel() as f64 * player.get_mass(), player.y_vel() as f64 * player.get_mass());
			} else {
				c.friction();
			}
		}

		for c in self.game_data.crates.iter_mut(){
			c.update_crates(&mut self.core, &crate_textures, player, map);
		}
	}

	// draw player
	pub fn draw_player(&mut self, fps_avg: f64, player: &mut Player, curr_bg: Rect) {
		// draw player
		player.set_cam_pos(curr_bg.x(), curr_bg.y());
		player.get_frame_display(&mut self.game_data, fps_avg);
		self.core.wincan.copy_ex(player.texture_all(), player.src(), player.get_cam_pos(), 0.0, None, player.facing_right, false).unwrap();
		// draw shield outline on player
		if player.get_shielded() { 
			let texture_creator = self.core.wincan.texture_creator();
			let shield_outline = texture_creator.load_texture("images/abilities/shield_outline.png").unwrap();
			let src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64);
			let pos = Rect::new(if player.facing_right { player.get_cam_pos().x-(TILE_SIZE_CAM/8) as i32 } else { player.get_cam_pos().x-(TILE_SIZE_CAM/4) as i32 }, 
								player.get_cam_pos().y-(TILE_SIZE_CAM/4) as i32, 
								TILE_SIZE_64+TILE_SIZE_CAM/4, 
								TILE_SIZE_64+TILE_SIZE_CAM/4);
			self.core.wincan.copy_ex(&shield_outline, src, pos, 0.0, None, !player.facing_right, false).unwrap(); 
		}
	}

	// draw player projectiles
	pub fn draw_player_projectile(&mut self, ability_textures: &Vec<Texture>, player: &Player, mousestate: MouseState)-> Result<(), String>  {
		for projectile in self.game_data.player_projectiles.iter_mut() {
			if projectile.is_active(){
				match projectile.p_type{
					ProjectileType::Bullet=>{
						self.core.wincan.copy_ex(&ability_textures[0], projectile.src(), projectile.set_cam_pos_large(player), 0.0, None, !projectile.facing_right, false).unwrap();
					}
					ProjectileType::Fireball=>{
						let time = projectile.elapsed;

						let angle = 0.0;
						//println!("{}", angle);
						
						//starting time, how many time for each frame, row of the pic, col of the pic, size of each frame
						let s = ROGUELIKE::display_animation(time, 4, 6, 4, TILE_SIZE);

						if mousestate.x() > player.get_cam_pos().x() && time == 0{
							projectile.facing_right = true;//face right
						}else if mousestate.x() < player.get_cam_pos().x()  && time == 0{
							projectile.facing_right = false;//face left
						}
						/*
						if player.facing_right == false && time == 0{
							projectile.facing_right = false;//face left
						}else if player.facing_right == true && time == 0{
							projectile.facing_right = true;//face right
						}
						*/
						projectile.elapsed += 1;
						self.core.wincan.copy_ex(&ability_textures[1], s, projectile.set_cam_pos_large(player), angle, None, !projectile.facing_right, false).unwrap();
					}
					ProjectileType::Shield => {
						self.core.wincan.copy(&ability_textures[3], projectile.src(), projectile.set_cam_pos(player)).unwrap();
					}
				}	
			}
		}
		Ok(())
	}

	//draw player weapon
	pub fn draw_weapon(&mut self, player: &Player, texture : &Texture){
		let rotation_point;
		let pos;
		let mut angle;

		// weapon animation
		if player.get_attacking() {
			angle = (player.get_attack_timer() * 60 / 250 ) as f64 - 60.0;
		} else { angle = - 60.0; }
		// display weapon
		if player.facing_right{
			pos = Rect::new(player.get_cam_pos().x() + TILE_SIZE_CAM as i32, 
							player.get_cam_pos().y()+(TILE_SIZE_CAM/2) as i32, 
							ATTACK_LENGTH, TILE_SIZE_CAM);
			rotation_point = Point::new(0, (TILE_SIZE_HALF) as i32); //rotation center
		} else{
			pos = Rect::new(player.get_cam_pos().x() - ATTACK_LENGTH as i32, 
							player.get_cam_pos().y()+(TILE_SIZE_CAM/2) as i32, 
							ATTACK_LENGTH, TILE_SIZE_CAM);
			rotation_point = Point::new(ATTACK_LENGTH as i32,  (TILE_SIZE_HALF)  as i32); //rotation center
			angle = -angle;
		}
		self.core.wincan.copy_ex(&texture, None, pos, angle, rotation_point, player.facing_right, false).unwrap();
	}

	pub fn draw_enemy_projectile(&mut self,ability_textures: &Vec<Texture> , player: &Player) {
		for projectile in self.game_data.enemy_projectiles.iter_mut() {
			if projectile.is_active(){
				self.core.wincan.copy(&ability_textures[2], projectile.src(), projectile.set_cam_pos(player)).unwrap();
			}
		}
	}

	pub fn display_animation(start_time: u128, frames: i32, row: i32, col: i32, size: u32) -> Rect {
		let x = (start_time/frames as u128) as i32;
		let mut src_x = 0;
		let mut src_y = 0;

		for i in 0..row{
			if x < col*(i+1) {//1st line
				src_x = (x-i*col)*size as i32;
				src_y = i*size as i32;
				break
			}
		}
		Rect::new(src_x as i32, src_y as i32, size, size)
	}
}