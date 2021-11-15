extern crate rogue_sdl;
mod enemy;
mod background;
mod player;
mod ui;
mod projectile;
mod credits;
mod gameinfo;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
//use std::time::Duration;
//use std::time::Instant;
use rand::Rng;
use crate::enemy::*;
use crate::projectile::*;
use crate::player::*;
use crate::background::*;
use crate::ui::*;

use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState};
//use sdl2::mouse::MouseButtonIterator;
//use sdl2::mouse::PressedMouseButtonIterator;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
//use sdl2::render::WindowCanvas;
//use sdl2::render::Texture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::render::TextureQuery;


use rogue_sdl::{Game, SDLCore};
use sdl2::video::WindowContext;
use crate::gameinfo::GameData;

// window globals
const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TILE_SIZE: u32 = 64;
const ATTACK_LENGTH: u32 = TILE_SIZE + (TILE_SIZE / 2);

const CENTER_W: i32 = (CAM_W / 2 - TILE_SIZE / 2) as i32;
const CENTER_H: i32 = (CAM_H / 2 - TILE_SIZE / 2) as i32;

//background globals
const BG_W: u32 = 2400;
const BG_H: u32 = 1440;

// game globals
const SPEED_LIMIT: f64 = 200.0;
const ACCEL_RATE: f64 = 200.0;
const STARTING_TIMER: u128 = 1000;

pub struct ROGUELIKE {
	core: SDLCore,
	game_data: GameData,
}

// calculate velocity resistance
fn resist(vel: i32, delta: i32) -> i32 {
	if delta == 0 {
		if vel > 0 {-1}
		else if vel < 0 {1}
		else {delta}
	} else {delta}
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

// CREATE GAME
impl Game for ROGUELIKE {

	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		let game_data = GameData::new();
		Ok(ROGUELIKE{ core, game_data })
	}

	fn run(&mut self) -> Result<(), String> {
        let texture_creator = self.core.wincan.texture_creator();
		let mut rng = rand::thread_rng();

		let mut count = 0;
		let f_display = 15;

		// FPS calculation
		let mut speed_limit_adj = 3.0;
		let mut accel_rate_adj = 0.0;

		// CREATE PLAYER SHOULD BE MOVED TO player.rs
		let mut player = player::Player::new(
			(CENTER_W as f64, CENTER_H as f64),
			texture_creator.load_texture("images/player/slime_sheet.png")?,
		);

		// INITIALIZE ARRAY OF ENEMIES (SHOULD BE MOVED TO room.rs WHEN CREATED)
		let fire_texture = texture_creator.load_texture("images/abilities/fireball.png")?;
		let bullet = texture_creator.load_texture("images/abilities/bullet.png")?;

		let mut enemies: Vec<Enemy> = Vec::with_capacity(5);	// Size is max number of enemies
		let mut rngt = vec![0; enemies.capacity()+1]; // rngt[0] is the timer for the enemys choice of movement
		let mut i=1;
		for _ in 0..enemies.capacity(){
			let num = rng.gen_range(1..5);
			let mut enemy_type: String; 
			match num {
				5 => { enemy_type = String::from("ranged") } 
				4 => { enemy_type = String::from("ranged") } 
				_ => { enemy_type = String::from("melee") } 
			}
			println!("{}", enemy_type);
			let e = enemy::Enemy::new(
				Rect::new(
					(CAM_W/2 - TILE_SIZE/2 + 200 + 5*rng.gen_range(5..20)) as i32,
					(CAM_H/2 - TILE_SIZE/2) as i32 + 5*rng.gen_range(5..20),
					TILE_SIZE,
					TILE_SIZE,
				),
				texture_creator.load_texture("images/enemies/ranged_enemy.png")?,
				enemy_type, 
			);
			enemies.push(e);
			rngt[i] = rng.gen_range(1..5); // decides if an enemy moves
			i+=1;
		}
		// SETUP ARRAY OF PROJECTILES
		// let mut projectiles: Vec<Projectile> = Vec::with_capacity(3);

		/* // CREATE FIREBALL (SHOULD BE MOVED TO fireball.rs WHEN CREATED)
        let mut fireball = ranged_attack::RangedAttack::new(
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
			false,
			false,
			0,
            texture_creator.load_texture("images/abilities/fireball.png")?,
		); */

		// CREATE ROOM 
		let xwalls: (i32, i32) = (1,rng.gen_range(19..27));
		let ywalls: (i32, i32) = (1,rng.gen_range(10..19));
		let xbounds: (i32,i32) = ((xwalls.0*TILE_SIZE as i32), ( (xwalls.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);
		let ybounds: (i32,i32) = ((ywalls.0*TILE_SIZE as i32), ( (ywalls.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);
		
		let mut background = background::Background::new(
			texture_creator.load_texture("images/background/bb.png")?,
			texture_creator.load_texture("images/background/tile.png")?,
			texture_creator.load_texture("images/background/moss_tile.png")?,
			texture_creator.load_texture("images/background/tile.png")?,
			texture_creator.load_texture("images/background/floor_tile_pilar.png")?, 
			xwalls, 
			ywalls, 
		);
<<<<<<< Updated upstream
=======
		let mut map_data = map::Map::new(background);
		map_data.create_map();
		map_data.print_map(map_data.map);

		// set starting position
		player.set_x((map_data.starting_position.0 as i32 * TILE_SIZE as i32 - (CAM_W - TILE_SIZE_CAM) as i32 / 2) as f64);
		player.set_y((map_data.starting_position.1 as i32 * TILE_SIZE as i32 - (CAM_H - TILE_SIZE_CAM) as i32 / 2) as f64);

		let mut enemies: Vec<Enemy> = Vec::new();
		let mut rngt = Vec::new();

		/* let ghost_tex = texture_creator.load_texture("images/enemies/place_holder_enemy.png")?;
		let gellem_tex = texture_creator.load_texture("images/enemies/ranged_enemy.png")?; */

		let mut enemy_count = 0;
		for h in 0..MAP_SIZE_H {
			for w in 0..MAP_SIZE_W {
				if map_data.enemy_and_object_spawns[h][w] == 0 {
					continue;
				}
				if DEBUG { println!("{}, {}", w, h); }
				match map_data.enemy_and_object_spawns[h][w] {
					1 => {
						let e = enemy::Enemy::new(
							Rect::new(
								w as i32 * TILE_SIZE as i32/*  - (player.x() % TILE_SIZE as f64) as i32 */ - (CAM_W as i32 - TILE_SIZE as i32) / 2,
								h as i32 * TILE_SIZE as i32/*  - (player.y() % TILE_SIZE as f64) as i32 */ - (CAM_H as i32 - TILE_SIZE as i32) / 2,
								TILE_SIZE,
								TILE_SIZE
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
								w as i32 * TILE_SIZE as i32/*  - (player.x() % TILE_SIZE as f64) as i32 */ - (CAM_W as i32 - TILE_SIZE as i32) / 2,
								h as i32 * TILE_SIZE as i32/*  - (player.y() % TILE_SIZE as f64) as i32 */ - (CAM_H as i32 - TILE_SIZE as i32) / 2,
								TILE_SIZE,
								TILE_SIZE
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
								TILE_SIZE,
								TILE_SIZE
							)
						);
						self.game_data.crates.push(c);
					}
					//new shield enemy
					4 => {
                        let e = enemy::Enemy::new(
                    		Rect::new(
                    			w as i32 * TILE_SIZE as i32/*  - (player.x() % TILE_SIZE as f64) as i32 */ - (CAM_W as i32 - TILE_SIZE as i32) / 2,
                    			h as i32 * TILE_SIZE as i32/*  - (player.y() % TILE_SIZE as f64) as i32 */ - (CAM_H as i32 - TILE_SIZE as i32) / 2,
                    			TILE_SIZE,
                    			TILE_SIZE
                    		    ),
                    			texture_creator.load_texture("images/enemies/Shield_skeleton.png")?,
                    			EnemyType::Melee,
                    			enemy_count,
                    		);
                    		rngt.push(rng.gen_range(1..5));
                    		enemy_count += 1;
                    	}
					_ => {}
				}
			}
		}
>>>>>>> Stashed changes

		let mut all_frames = 0;
		let mut last_time = Instant::now();

		// obstacles that everything should collide with
		#[allow(unused_variables)]
		let obstacle_pos = background.create_new_map(xwalls, ywalls);

		// MAIN GAME LOOP
		'gameloop: loop {
			all_frames += 1;
			let elapsed = last_time.elapsed();
			if elapsed > Duration::from_secs(1) {
				let mut fps_avg = (all_frames as f64) / elapsed.as_secs_f64();
				//println!("Average FPS: {:.2}", fps_avg);

				fps_avg = fps_avg.recip();
				speed_limit_adj = fps_avg * SPEED_LIMIT;
				//println!("Speed limit adjusted: {}", speed_limit_adj);
				accel_rate_adj = fps_avg * ACCEL_RATE;
				//println!("Acceleration rate adjusted: {}", accel_rate_adj);
			}
			// reset frame
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			player.set_x_delta(0);
			player.set_y_delta(0);

			let mousestate= self.core.event_pump.mouse_state();
			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

				// FOR TESTING ONLY: USE TO FOR PRINT VALUES
				if keystate.contains(&Keycode::P) {
					//println!("\nx:{} y:{} ", enemies[0].x() as i32, enemies[0].y() as i32);
					//println!("{} {} {} {}", enemies[0].x() as i32, enemies[0].x() as i32 + (enemies[0].width() as i32), enemies[0].y() as i32, enemies[0].y() as i32 + (enemies[0].height() as i32));
					//println!("{} {}", player.x(), player.y());
					println!("{}", player.get_hp());
				}
			// CLEAR BACKGROUND
            self.core.wincan.copy(&background.black, None, None)?;

			// UPDATE BACKGROUND
			let cur_bg = Rect::new(
				(player.x() as i32 + ((player.width() / 2) as i32)) - ((CAM_W / 2) as i32),
				(player.y() as i32 + ((player.height() / 2) as i32)) - ((CAM_H / 2) as i32),
				CAM_W,
				CAM_H,
			);
			ROGUELIKE::update_background(self, xwalls, ywalls, &player, &background)?;

			// UPDATE ENEMIES
			if elapsed > Duration::from_secs(2) {
				rngt = ROGUELIKE::update_enemies(self, xbounds, ybounds, rngt, &mut enemies, &mut player, speed_limit_adj);
			}

			// UPDATE PLAYER
			ROGUELIKE::check_inputs(self, &keystate, mousestate, &mut player, accel_rate_adj,speed_limit_adj);
			ROGUELIKE::update_player(xwalls, ywalls, xbounds, ybounds, &mut player, &obstacle_pos, speed_limit_adj);
			self.draw_player(&count, &f_display, &mut player, &cur_bg);
			count = count + 1;
			if count > f_display * 5 {
				count = 0;
			}

			// UPDATE ATTACKS
			// Should be switched to take in array of active fireballs, bullets, etc.
			ROGUELIKE::update_projectiles(&mut player, &mut self.game_data.player_projectiles, &mut self.game_data.enemy_projectiles);
			ROGUELIKE::display_weapon(self, &mut player)?;
			
			// UPDATE OBSTACLES
			// function to check explosive barrels stuff like that should go here. placed for ordering. 			

			// CHECK COLLISIONS
			ROGUELIKE::check_collisions(self, xbounds, ybounds, &mut player, &mut enemies);
			if player.is_dead(){
				break 'gameloop;
			}

			// UPDATE UI
			ROGUELIKE::update_ui(self, &player)?;
			// DRAW PROJECTILES
			ROGUELIKE::draw_projectile(self, &bullet, &player, 0.0);	

			// UPDATE FRAME
			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

pub fn main() -> Result<(), String> {
    rogue_sdl::runner(TITLE, ROGUELIKE::init);
	//credits::run_credits()
	Ok(())
}

// Create map
impl ROGUELIKE {
	pub fn update_background(&mut self, xwalls: (i32,i32), ywalls: (i32,i32), player: &Player, background:& Background) -> Result<(), String> {
		let mut n = 0;
		for i in 0..xwalls.1+1 {
			for j in 0..ywalls.1+1 {
				if background.tiles[n].0 {
					let num = background.tiles[n].1;
					let texture;
					match num {
						7 => { texture = &background.texture_3 } // pillar 
						6 => { texture = &background.texture_2 } // border tiles
						1 => { texture = &background.texture_1 } // slime on tile
						_ => { texture = &background.texture_0 } // regular tile
					}
					// double tile size 
					let src;
					let pos;
					if num==7 {
						src = Rect::new(0, 0, TILE_SIZE*2, TILE_SIZE*2);
						pos = Rect::new(i * TILE_SIZE as i32 + (CENTER_W - player.x() as i32),
											j * TILE_SIZE as i32 + (CENTER_H - player.y() as i32),
											TILE_SIZE*2, TILE_SIZE*2);
					} else {
						src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
						pos = Rect::new(i * TILE_SIZE as i32 + (CENTER_W - player.x() as i32),
											j * TILE_SIZE as i32 + (CENTER_H - player.y() as i32),
											TILE_SIZE, TILE_SIZE);
					}
					self.core.wincan.copy(texture, src, pos)?;
				}
				n+=1;
			}
		}
		Ok(())
	}
	// update enemies
	pub fn update_enemies(&mut self, xbounds: (i32,i32), ybounds: (i32,i32), mut rngt: Vec<i32>, enemies: &mut Vec<Enemy>, player: &mut Player, speed_limit_adj: f64) -> Vec<i32>{
		let mut i = 1;
		let mut rng = rand::thread_rng();
		for enemy in enemies {
			if !enemy.is_alive(){
				continue;
			}

			if enemy.get_fire_timer() > enemy.get_fire_cooldown() && enemy.enemy_type == String::from("ranged") {
				enemy.set_fire_cooldown();
				let fire_chance = rng.gen_range(1..60);
				if fire_chance < 5 { // chance to fire
					enemy.fire(); // sets is firing true
					let vec = vec![player.x() - enemy.x(), 
								   player.y() - enemy.y()];
					let angle = ((vec[0] / vec[1]).abs()).atan();
					let speed: f64 = speed_limit_adj;
					let mut x = &speed * angle.sin();
					let mut y = &speed * angle.cos();
					if vec[0] < 0.0 {
						x *= -1.0;
					}
					if vec[1] < 0.0  {
						y *= -1.0;
					}
					let bullet = projectile::Projectile::new(
						Rect::new(
							enemy.pos().x(),
							enemy.pos().y(),
							TILE_SIZE/2,
							TILE_SIZE/2,
						),
						false,
						false,
						0,
						vec![x,y],
					);
					self.game_data.enemy_projectiles.push(bullet);
				}
			}
			
			// aggro / move
			if rngt[0] > 30 || ROGUELIKE::check_edge(xbounds, ybounds, &enemy){
				rngt[i] = rng.gen_range(1..5);
				rngt[0] = 0;
			}
			let x_d = (enemy.x() - player.x()).powf(2.0);
			let y_d = (enemy.y() - player.y()).powf(2.0);
			let distance = (x_d + y_d).sqrt();
			if enemy.get_stun_timer() > 1000 {
				enemy.set_stunned(false);
			} else {
				enemy.slow_vel(0.1);
				let angle = enemy.angle();
				let mut x = (enemy.get_vel() * -1.0) * angle.sin();
				if enemy.x_flipped {
					x *= -1.0;
				}
				let mut y = (enemy.get_vel() * -1.0) * angle.cos();
				if enemy.y_flipped {
					y *= -1.0;
				}
				enemy.pos.set_x(((enemy.x() + x) as i32).clamp(xbounds.0, xbounds.1));
				enemy.pos.set_y(((enemy.y() + y) as i32).clamp(ybounds.0, ybounds.1));
			}
			if distance > 300.0 {
				enemy.update_pos(rngt[i], xbounds, ybounds);
			} else {
				if enemy.enemy_type == String::from("melee") {
					enemy.aggro(player.x().into(), player.y().into(), xbounds, ybounds, speed_limit_adj);
				} else {
					enemy.flee(player.x().into(), player.y().into(), xbounds, ybounds, speed_limit_adj);
				}
			}
			let pos = Rect::new(enemy.x() as i32 + (CENTER_W - player.x() as i32),
								enemy.y() as i32 + (CENTER_H - player.y() as i32),
								TILE_SIZE, TILE_SIZE);
			self.core.wincan.copy(enemy.txtre(), enemy.src(), pos).unwrap();
			i += 1;
		}
		rngt[0] += 1;
		return rngt;
	}

	// check input values
	pub fn check_inputs(&mut self, keystate: &HashSet<Keycode>, mousestate: MouseState, mut player: &mut Player, accel_rate_adj: f64, speed_limit_adj: f64) {
		// move up
		if keystate.contains(&Keycode::W) {
			player.set_y_delta(player.y_delta() - accel_rate_adj as i32);
		}
		// move left
		if keystate.contains(&Keycode::A) {
			player.set_x_delta(player.x_delta() - accel_rate_adj as i32);
			player.facing_right = false;
		}
		// move down
		if keystate.contains(&Keycode::S) {
			player.set_y_delta(player.y_delta() + accel_rate_adj as i32);
		}
		// move right
		if keystate.contains(&Keycode::D) {
			player.set_x_delta(player.x_delta() + accel_rate_adj as i32);
			player.facing_right = true;
		}
		// basic attack
		if keystate.contains(&Keycode::Space) {
			if !(player.is_attacking) {
				player.attack();
			}
		}
		// Shoot ranged attack
		if mousestate.left(){
			if !player.is_firing && player.get_mana() > 0 {
				player.fire(); 
				let vec = vec![mousestate.x() as f64 - CENTER_W as f64 - (TILE_SIZE/2) as f64, mousestate.y() as f64 - CENTER_H as f64 - (TILE_SIZE/2) as f64];
				let angle = ((vec[0] / vec[1]).abs()).atan();
				let speed: f64 = 3.0* speed_limit_adj;
				let mut x = &speed * angle.sin();
				let mut y = &speed * angle.cos();
				if vec[0] < 0.0 {
					x *= -1.0;
				}
				if vec[1] < 0.0  {
					y *= -1.0;
				}
				let bullet = projectile::Projectile::new(
					Rect::new(
						player.pos().x(),
						player.pos().y(),
						TILE_SIZE/2,
						TILE_SIZE/2,
					),
					false,
					false,
					0,
					vec![x,y],
				);
				self.game_data.player_projectiles.push(bullet);
			}
		}
		//ability
		if keystate.contains(&Keycode::F){
			println!("you found the easter egg");
		}
			
	}

	// update projectiles
	pub fn update_projectiles(player: &mut Player, player_projectiles: &mut Vec<Projectile>, enemy_projectiles: &mut Vec<Projectile>) {
		for projectile in player_projectiles {
			if projectile.is_active() {
				projectile.update_pos((0, (CAM_W - TILE_SIZE) as i32));
			}
		}
		for projectile in enemy_projectiles {
			if projectile.is_active() {
				projectile.update_pos((0, (CAM_W - TILE_SIZE) as i32));
				
			}
		}
	}

	// check collisions
	fn check_collisions(&mut self, xbounds: (i32,i32), ybounds: (i32,i32), player: &mut Player, enemies: &mut Vec<Enemy>) {
		for enemy in enemies {
			if !enemy.is_alive() {
				continue;
			}

			// player collision
			if check_collision(&player.pos(), &enemy.pos()) {
				player.minus_hp(5);
			}
			
			// player projectile collisions
			for projectile in self.game_data.player_projectiles.iter_mut(){
				if check_collision(&projectile.pos(), &enemy.pos())  && projectile.is_active() {
					enemy.knockback(projectile.x().into(), projectile.y().into(), xbounds, ybounds);
					enemy.minus_hp(5.0);
					projectile.die();
				}
			}

			// player melee collisions
			if player.is_attacking {
				if check_collision(&player.get_attack_box(), &enemy.pos()) {
					enemy.knockback(player.x().into(), player.y().into(), xbounds, ybounds);
					enemy.minus_hp(1.0);
				}
			}

			// enemy projectile collisions
			for projectile in self.game_data.enemy_projectiles.iter_mut(){
				if check_collision(&projectile.pos(), &player.pos()) && projectile.is_active() {
					player.minus_hp(5);
					projectile.die();
				}
			}	
		}
		player.set_invincible();
	}

	// update player
	fn update_player(xwalls: (i32,i32), ywalls: (i32,i32), xbounds: (i32,i32), ybounds: (i32,i32), mut player: &mut Player, obstacle_pos: &Vec<(i32,i32)>, speed_limit_adj: f64) {
		// Slow down to 0 vel if no input and non-zero velocity
		player.set_x_delta(resist(player.x_vel() as i32, player.x_delta() as i32));
		player.set_y_delta(resist(player.y_vel() as i32, player.y_delta() as i32));

		// Don't exceed speed limit
		player.set_x_vel((player.x_vel() + player.x_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));
		player.set_y_vel((player.y_vel() + player.y_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));

		// Stay inside the viewing window
		player.set_x((player.x() + player.x_vel() as f64).clamp(0.0, (xwalls.1 * TILE_SIZE as i32) as f64) as f64);
		player.set_y((player.y() + player.y_vel() as f64).clamp(0.0, (ywalls.1 * TILE_SIZE as i32) as f64) as f64);

		for ob in obstacle_pos {
			let obs = Rect::new(ob.0 * TILE_SIZE as i32, ob.1 * TILE_SIZE as i32, TILE_SIZE*2, TILE_SIZE*2);
			if check_collision(&player.pos(), &obs) {
				// collision on object top
				if (player.pos().bottom() >= obs.top()) && (player.pos().bottom() < obs.bottom()) 		// check y bounds
				&& (player.pos().left() > obs.left()) && (player.pos().right() < obs.right()) {			// prevent x moves
					player.set_y((player.y() + player.y_vel() as f64).clamp(0.0, ((ob.1 - 1) * TILE_SIZE as i32) as f64));
				// collision on object bottom
				} else if (player.pos().top() < obs.bottom()) && (player.pos().top() > obs.top()) 		// check y bounds
				&& (player.pos().left() > obs.left()) && (player.pos().right() < obs.right()) {			// prevent x moves
					player.set_y((player.y() + player.y_vel() as f64).clamp(((ob.1 + 2) * TILE_SIZE as i32) as f64, (ywalls.1 * TILE_SIZE as i32) as f64) as f64);
				// collision on object left 
				} else if (player.pos().right() > obs.left()) && (player.pos().right() < obs.right())	// check x bounds
					   && (player.pos().top() > obs.top()) && (player.pos().bottom() < obs.bottom()) {	// prevent y moves
					player.set_x((player.x() + player.x_vel() as f64).clamp(0.0, ((ob.0-1) * TILE_SIZE as i32) as f64));
					// collision on object right
				} else if (player.pos().left() < obs.right()) && (player.pos().left() > obs.left()) 	// check x bounds
					   && (player.pos().top() > obs.top()) && (player.pos().bottom() < obs.bottom()) {	// prevent y moves
					player.set_x((player.x() + player.x_vel() as f64).clamp(((ob.0 + 2) * TILE_SIZE as i32) as f64,
					(xwalls.1 * TILE_SIZE as i32) as f64));
				}
			}
		}

		player.update_pos(xbounds, ybounds);

		if player.is_attacking { player.set_attack_box(player.x() as i32, player.y() as i32); }

		if player.get_attack_timer() > player.get_cooldown() {
			player.set_cooldown();
		}
		if player.get_fire_timer() > player.get_fire_cooldown() {
			player.set_fire_cooldown();
		}

		player.restore_mana();
	}

	//update background
	pub fn unused_background(&mut self, player: &mut Player, background: &mut Background) -> Result<(), String> {
		let cur_bg = Rect::new(
			((player.x() as i32 + ((player.width() / 2) as i32)) - ((CAM_W / 2) as i32)).clamp(0, (BG_W - CAM_W) as i32),
			((player.y() as i32 + ((player.height() / 2) as i32)) - ((CAM_H / 2) as i32)).clamp(0, (BG_H - CAM_H) as i32),
			CAM_W,
			CAM_H,
		);
		self.core.wincan.set_draw_color(Color::BLACK);
		self.core.wincan.clear();

		// Draw subset of bg
		self.core.wincan.copy(background.texture(), cur_bg, None).unwrap();
		Ok(())
	}

	//draw weapon
	pub fn display_weapon(&mut self, player: &mut Player) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();
		let sword = texture_creator.load_texture("images/player/sword_l.png")?;
		let rotation_point;
		let pos;
		let mut angle;

		// weapon animation
		if player.is_attacking {
			angle = (player.get_attack_timer() * 60 / 250 ) as f64 - 60.0;
		} else { angle = - 60.0; }
		// display weapon
		if player.facing_right{
			pos = Rect::new(player.get_cam_pos().x() + TILE_SIZE as i32, player.get_cam_pos().y()+(TILE_SIZE/2) as i32, ATTACK_LENGTH, TILE_SIZE);
			rotation_point = Point::new(0, (TILE_SIZE/2) as i32); //rotation center
		} else{
			pos = Rect::new(player.get_cam_pos().x() - ATTACK_LENGTH as i32, player.get_cam_pos().y()+(TILE_SIZE/2) as i32, ATTACK_LENGTH, TILE_SIZE);
			rotation_point = Point::new(ATTACK_LENGTH as i32,  (TILE_SIZE/2)  as i32); //rotation center
			angle = -angle;
		}

		self.core.wincan.copy_ex(&sword, None, pos, angle, rotation_point, player.facing_right, false).unwrap();
		Ok(())
	}

	//update background
	pub fn update_ui(&mut self, player: &Player) -> Result<(), String> {
		// set ui bar
		let texture_creator = self.core.wincan.texture_creator();
		let src = Rect::new(0, 0, CAM_W, TILE_SIZE*2);
		let pos = Rect::new(0, (CAM_H - TILE_SIZE) as i32 - 16, CAM_W, TILE_SIZE*3/2);
		let ui = texture_creator.load_texture("images/ui/bb_wide_yellow.png")?;
		self.core.wincan.copy(&ui, src, pos)?;
		let pos = Rect::new(0, (CAM_H - TILE_SIZE) as i32 - 8, CAM_W, TILE_SIZE*3/2);
		let ui = texture_creator.load_texture("images/ui/bb_wide.png")?;
		self.core.wincan.copy(&ui, src, pos)?;

		//create hearts
		let mut i=0;
		while i+10 < player.get_hp() {
			let heart = ui::UI::new(
				Rect::new(
					(i/10) as i32 *(TILE_SIZE as f64 *1.2) as i32,
					(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
					(TILE_SIZE as f64 *1.2) as u32,
					(TILE_SIZE as f64 *1.2) as u32,
				), 
				texture_creator.load_texture("images/ui/heart.png")?,
			);
			self.core.wincan.copy(heart.texture(), heart.src(), heart.pos())?;
			i+=10;
		}
		
		let mut texture = texture_creator.load_texture("images/ui/heart.png")? ;
		if  player.get_hp()%10 != 0  {
			texture = texture_creator.load_texture("images/ui/half_heart.png")?;
		}
			let half_heart = ui::UI::new(
				Rect::new(
					(i/10) as i32 * (TILE_SIZE as f64 *1.2) as i32,
					(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
					(TILE_SIZE as f64 *1.2) as u32,
					(TILE_SIZE as f64 *1.2) as u32,
				),
				texture,
			);
		self.core.wincan.copy(half_heart.texture(), half_heart.src(), half_heart.pos())?;

		//display mana
		let mut mana = ui::UI::new(
			Rect::new(
				(CAM_W-(TILE_SIZE*4)) as i32,
				(CAM_H-(TILE_SIZE)) as i32,
				(TILE_SIZE as f64 / 1.2) as u32,
				(TILE_SIZE as f64 / 1.2) as u32,
			),
			texture_creator.load_texture("images/ui/mana.png")?,
		);
		let mut cur_mana = 0;
		match player.get_mana() {
			0 => cur_mana = 32 * 4,
			1 => cur_mana = 32 * 3,
			2 => cur_mana = 32 * 2,
			3 => cur_mana = 32 * 1,
			4 => cur_mana = 32 * 0,
			_ => cur_mana = 32 * 0,
		}
		let mana_src = Rect::new(cur_mana, 0, TILE_SIZE / 2, TILE_SIZE / 2);
		mana.set_src(mana_src);
		self.core.wincan.copy(mana.texture(), mana.src(), mana.pos())?;

		//get current mana as a string
		let mana = player.get_mana();
		let max_mana = player.get_max_mana();
		let mut s: String = mana.to_string();
		let mut a: String = max_mana.to_string();
		s += "/";
		s += &a;

		//display string next to mana




		//display equipped waepon
		if player.get_curr_meele() == "sword_l"
		{
			let weapon = ui::UI::new(
				Rect::new(
					(CAM_W-((TILE_SIZE as f64 * 1.2) as u32)*8) as i32,
					(CAM_H-(TILE_SIZE as f64 * 1.2) as u32) as i32,
					(TILE_SIZE as f64 * 1.2) as u32,
					(TILE_SIZE as f64 * 1.2) as u32,
				),
				texture_creator.load_texture("images/player/sword_l.png")?,
			);
			self.core.wincan.copy(weapon.texture(), weapon.src(),weapon.pos())?;
		}


		if player.get_curr_ability() == "bullet"
		{
			let ability = ui::UI::new(
				Rect::new(
					(CAM_W-((TILE_SIZE as f64 * 1.2) as u32)*6) as i32,
					(CAM_H-(TILE_SIZE as f64 * 1.2) as u32) as i32,
					(TILE_SIZE as f64 * 1.2) as u32,
					(TILE_SIZE as f64 * 1.2) as u32,
				),
				texture_creator.load_texture("images/abilities/bullet.png")?,
			);
			self.core.wincan.copy(ability.texture(), ability.src(),ability.pos())?;
		}
		
		

		// create coins
		let coin = ui::UI::new(
			Rect::new(
				(CAM_W-(TILE_SIZE as f64 *1.2) as u32) as i32,
				(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
				(TILE_SIZE as f64 *1.2) as u32,
				(TILE_SIZE as f64 *1.2) as u32,
			),
			texture_creator.load_texture("images/ui/gold_coin.png")?,
		);
		self.core.wincan.copy(coin.texture(), coin.src(), coin.pos())?;
		// x 
		/* let x = ui::UI::new(
			Rect::new(
				(CAM_W-((2*TILE_SIZE) as f64 /1.2) as u32) as i32,
				(CAM_H-(TILE_SIZE as f64) as u32) as i32,
				(TILE_SIZE as f64 /1.2) as u32,
				(TILE_SIZE as f64 /1.2) as u32,
			),
			texture_creator.load_texture("images/ui/x.png")?,
		);
		self.core.wincan.copy(x.texture(), x.src(), x.pos())?; */
		// number of coins
		Ok(())
	}

	// draw player
	pub fn draw_player(&mut self, count: &i32, f_display: &i32, player: &mut Player, cur_bg: &Rect) {
		player.set_cam_pos(cur_bg.x(), cur_bg.y());
		player.get_frame_display(count, f_display);
		self.core.wincan.copy_ex(player.texture_all(), player.src(), player.get_cam_pos(), 0.0, None, player.facing_right, false).unwrap();
	}

	pub fn draw_projectile(&mut self, bullet: &Texture, player: &Player, angle: f64) -> Result<(), String> {
		let p = Point::new(0, (TILE_SIZE/2) as i32);
		for projectile in self.game_data.player_projectiles.iter_mut() {
			if projectile.is_active(){
				let pos = Rect::new(projectile.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
									projectile.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE, TILE_SIZE);
				self.core.wincan.copy_ex(&bullet, None, pos, angle,p,player.facing_right,false)?; // rotation center
			}
		}
		let p = Point::new(0, (TILE_SIZE/2) as i32);
		for projectile in self.game_data.enemy_projectiles.iter_mut() {
			if projectile.is_active(){
				let pos = Rect::new(projectile.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
									projectile.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE, TILE_SIZE);
				self.core.wincan.copy_ex(&bullet, None, pos, angle, p, false, false)?; // rotation center
			}
		}
		Ok(())
	}

	// force enemy movement
	pub fn check_edge(xbounds: (i32,i32), ybounds: (i32,i32), enemy: &enemy::Enemy) -> bool{
		if  enemy.x() <= xbounds.0 as f64 ||
		enemy.x() >=  xbounds.1 as f64 ||
		enemy.y() <= ybounds.0 as f64||
		enemy.y() >= ybounds.1 as f64
		{return true;}
		else {return false;}
	}
}
