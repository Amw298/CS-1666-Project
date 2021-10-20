extern crate rogue_sdl;
mod enemy;
mod background;
mod player;
mod ranged_attack;
mod credits;
mod top_down_scroll;

use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
use rand::Rng;
use crate::enemy::*;
use crate::ranged_attack::*;
use crate::player::*;
use crate::background::*;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::mouse::MouseButtonIterator;
use sdl2::mouse::PressedMouseButtonIterator;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
//use sdl2::render::Texture;

use rogue_sdl::{Game, SDLCore};

// window globals
const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TILE_SIZE: u32 = 64;
const ATTACK_LENGTH: u32 = TILE_SIZE + (TILE_SIZE / 2);

const START_W: i32 = (CAM_W / 2 - TILE_SIZE / 2) as i32;
const START_H: i32 = (CAM_H / 2 - TILE_SIZE / 2) as i32;

//background globals
const BG_W: u32 = 2400;
const BG_H: u32 = 1440;

// game globals
const SPEED_LIMIT: i32 = 3;
const ACCEL_RATE: i32 = 3;

const XWALLS: (i32, i32) = (1,19);
const YWALLS: (i32, i32) = (1,9);
const XBOUNDS: (i32,i32) = ((XWALLS.0*TILE_SIZE as i32), ( (XWALLS.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);
const YBOUNDS: (i32,i32) = ((YWALLS.0*TILE_SIZE as i32), ( (YWALLS.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);



pub struct ROGUELIKE {
	core: SDLCore,
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
		Ok(ROGUELIKE{ core })
	}

	fn run(&mut self) -> Result<(), String> {
        // reset frame
        let texture_creator = self.core.wincan.texture_creator();
		let screen_width = 25;
		
		let mut rng = rand::thread_rng();
		let mut count = 0;
		let f_display = 15;

		// CREATE PLAYER SHOULD BE MOVED TO player.rs
		let mut player = player::Player::new(
			Rect::new(
				START_W,
				START_H,
				TILE_SIZE,
				TILE_SIZE,
			),
			texture_creator.load_texture("images/player/Slime l.png")?,
			texture_creator.load_texture("images/player/Slime r.png")?,
			texture_creator.load_texture("images/player/Slime left.png")?,
			texture_creator.load_texture("images/player/Slime right.png")?,
		);

		// INITIALIZE ARRAY OF ENEMIES (SHOULD BE MOVED TO room.rs WHEN CREATED)
		let mut enemies: Vec<Enemy> = Vec::with_capacity(2);	// Size is max number of enemies
		let mut rngt = vec![0; enemies.capacity()+1]; // rngt[0] is the timer for the enemys choice of movement
		let mut i=1; 
		for _ in 0..enemies.capacity(){
			let e = enemy::Enemy::new(
				Rect::new(
					(CAM_W/2 - TILE_SIZE/2 + 100) as i32,
					(CAM_H/2 - TILE_SIZE/2 + 100) as i32,
					TILE_SIZE,
					TILE_SIZE,
				),
				texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
			);
			enemies.push(e);
			rngt[i] = rng.gen_range(1..5); // decides if an enemy moves
			i+=1;
		}
		
		// CREATE FIREBALL (SHOULD BE MOVED TO fireball.rs WHEN CREATED)
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
            texture_creator.load_texture("images/fireball/fireball.png")?,
		);

		// MAIN GAME LOOP
		'gameloop: loop {
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
					println!("\nx:{} y:{} ", enemies[0].x() as i32, enemies[0].y() as i32);
					println!("{} {} {} {}", enemies[0].x() as i32, enemies[0].x() as i32 + (enemies[0].width() as i32), enemies[0].y() as i32, enemies[0].y() as i32 + (enemies[0].height() as i32)); 
				}
			// CLEAR BACKGROUND
            let background = texture_creator.load_texture("images/background/bb.png")?;
            self.core.wincan.copy(&background, None, None)?;


			ROGUELIKE::check_inputs(&mut fireball, keystate, mousestate, &mut player);
			ROGUELIKE::update_player(&screen_width, &mut player);
			//ROGUELIKE::update_background(self, &mut player);
			ROGUELIKE::check_collisions(&mut player, &mut enemies);
			if player.is_dead(){
				break 'gameloop;
			}

			// SET BACKGROUND
			let cur_bg = Rect::new(
				((player.x() + ((player.width() / 2) as i32)) - ((CAM_W / 2) as i32)),
				((player.y() + ((player.height() / 2) as i32)) - ((CAM_H / 2) as i32)),
				CAM_W,
				CAM_H,
			);

			ROGUELIKE::create_map(self, &cur_bg, player.x(), player.y());

			// UPDATE ENEMIES
			rngt = ROGUELIKE::update_enemies(self, rngt, &mut enemies, &mut player);

			// Should be switched to take in array of active fireballs, bullets, etc.
			self.update_projectiles(&mut fireball);
			self.draw_player(&count, &f_display, &mut player, &cur_bg);
			count = count + 1;
			if count > f_display * 5 {
				count = 0;
			}

			if player.is_attacking {
				if *player.facing_right() {
					let r = Rect::new(START_W + TILE_SIZE as i32, START_H, ATTACK_LENGTH, TILE_SIZE);
					let sword_r = texture_creator.load_texture("images/player/sword_r.png")?;
					self.core.wincan.copy(&sword_r, None, r);
					//self.core.wincan.set_draw_color(Color::RED);
					//self.core.wincan.fill_rect(r);
				} else {
					let r = Rect::new(START_W - ATTACK_LENGTH as i32, START_H, ATTACK_LENGTH, TILE_SIZE);
					let sword_l = texture_creator.load_texture("images/player/sword_l.png")?;
					self.core.wincan.copy(&sword_l, None, r);
					//self.core.wincan.set_draw_color(Color::RED);
					//self.core.wincan.fill_rect(r);
				}
			}

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
	pub fn create_map(&mut self, cur_bg: &Rect, player_pos_x: i32, player_pos_y: i32) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();
		for i in XWALLS.0..XWALLS.1 {
			for j in YWALLS.0..YWALLS.1 {
				if( i==XWALLS.0 || i==XWALLS.1-1 || j==YWALLS.0 || j==YWALLS.1-1 ){
					let num = rand::thread_rng().gen_range(0..2);
					let texture;
					match num {
						0 => { texture = texture_creator.load_texture("images/background/floor_tile_1.png")? }
						// TODO: change below to floor tile 2 to allow for random tiling
						1 => { texture = texture_creator.load_texture("images/background/floor_tile_1.png")? }
						_ => { texture = texture_creator.load_texture("images/background/floor_tile_1.png")? }
					}
					let src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
					let pos = Rect::new(i * TILE_SIZE as i32 + (START_W - player_pos_x),
										j * TILE_SIZE as i32 + (START_H - player_pos_y),
										TILE_SIZE, TILE_SIZE);
					self.core.wincan.copy(&texture, src, pos)?;
				}
			}
		}
		Ok(())
	}
// update enemies
	pub fn update_enemies(&mut self, mut rngt: Vec<i32>, enemies: &mut Vec<Enemy>, player: &mut Player) -> Vec<i32>{
		let mut i = 1;
		let mut rng = rand::thread_rng();
		for enemy in enemies {
			if enemy.is_alive() {
				if rngt[0] > 47 || ROGUELIKE::check_edge(&enemy){
					rngt[i] = rng.gen_range(1..5);
					rngt[0] = 0;
				}
				let x_d = (enemy.x() as i32 - player.x()).pow(2);
				let y_d = (enemy.y() as i32 - player.y()).pow(2);
				let distance = ((x_d + y_d) as f64).sqrt();
				if enemy.get_stun_timer() > 1000 {
					enemy.set_stunned(false);
				} else {
					enemy.slow_vel(0.1);
					let angle = enemy.angle();
					let mut x = (enemy.get_vel() * -1.0) * angle.sin();
					if(enemy.x_flipped) {
						x *= -1.0;
					}
					let mut y = (enemy.get_vel() * -1.0) * angle.cos();
					if(enemy.y_flipped) {
						y *= -1.0;
					}
					enemy.pos.set_x(((enemy.x() + x) as i32).clamp(XBOUNDS.0, XBOUNDS.1));
					enemy.pos.set_y(((enemy.y() + y) as i32).clamp(YBOUNDS.0, YBOUNDS.1));
				}
				if distance > 300.0 {
					enemy.update_pos(rngt[i], XBOUNDS, YBOUNDS);
				} else {
					enemy.aggro(player.x().into(), player.y().into(), XBOUNDS, YBOUNDS);
				}
				let pos = Rect::new(enemy.x() as i32 + (START_W - player.x()),
									enemy.y() as i32 + (START_H - player.y()),
									TILE_SIZE, TILE_SIZE);
				self.core.wincan.copy(enemy.txtre(), enemy.src(), pos).unwrap();
				i += 1;
			}
		}
		rngt[0] += 1;
		return rngt;
	}


// check input values

	pub fn check_inputs(fireball: &mut RangedAttack, keystate: HashSet<Keycode>, mousestate: MouseState, mut player: &mut Player) {
		// move up
		if keystate.contains(&Keycode::W) {
			player.set_y_delta(player.y_delta() - ACCEL_RATE);
			player.is_still = false;
		}
		// move left
		if keystate.contains(&Keycode::A) {
			player.set_x_delta(player.x_delta() - ACCEL_RATE);
			player.facing_left = true;
			player.facing_right = false;
			player.is_still = false;
		}
		// move down
		if keystate.contains(&Keycode::S) {
			player.set_y_delta(player.y_delta() + ACCEL_RATE);
			player.is_still = false;
		}
		// move right
		if keystate.contains(&Keycode::D) {
			player.set_x_delta(player.x_delta() + ACCEL_RATE);
			player.facing_left = false;
			player.facing_right = true;
			player.is_still = false;
		}
		// basic attack
		if mousestate.left() || keystate.contains(&Keycode::Space) {
			if !(player.is_attacking()) {
				/*println!(
					"X = {:?}, Y = {:?}",
					mousestate.x(),
					mousestate.y(),
				);*/
				// player.base_attack(mousestate.x(), mousestate.y());
				player.attack();
			}
		}
		// shoot fireball
		if keystate.contains(&Keycode::F) && fireball.frame() == 0 {
			fireball.set_use(true);
			fireball.start_pos(player.x(), player.y(), player.facing_left);
		}
	}


	// update projectiles
	pub fn update_projectiles(&mut self, fireball: &mut RangedAttack) {
		if fireball.in_use() {
			fireball.set_frame(fireball.frame() + 1);
			fireball.update_pos((0, (CAM_W - TILE_SIZE) as i32));
			if fireball.frame() == 28 {
				fireball.set_use(false);
				fireball.set_frame(0);
			}
			// this needs to be mirrored
			self.core.wincan.copy(fireball.texture(), fireball.src(4, 7), fireball.pos()).unwrap();
		}
	}


	// check collisions
	fn check_collisions(player: &mut Player, enemies: &mut Vec<Enemy>) {
		for enemy in enemies {
			if check_collision(&player.pos(), &enemy.pos())
			{
				player.minus_hp(0.2);
			}

			if check_collision(&player.pos(), &enemy.pos())
			{
				player.minus_hp(0.2);
			}

			if player.is_attacking
			{
				if check_collision(&player.get_attack_box(), &enemy.pos())
				{
					enemy.knockback(player.x().into(), player.y().into(), XBOUNDS, YBOUNDS);
					enemy.minus_hp(1.0);
				}
			}
		}
	}

	// update player
	fn update_player(w: &u32, mut player: &mut Player) {
		// Slow down to 0 vel if no input and non-zero velocity
		player.set_x_delta(resist(player.x_vel(), player.x_delta()));
		player.set_y_delta(resist(player.y_vel(), player.y_delta()));

		//when player is not moving
		if player.x_vel() == 0 && player.y_vel() == 0 { player.is_still = true; } // What does this line do?

		// Don't exceed speed limit
		player.set_x_vel((player.x_vel() + player.x_delta()).clamp(-SPEED_LIMIT, SPEED_LIMIT));
		player.set_y_vel((player.y_vel() + player.y_delta()).clamp(-SPEED_LIMIT, SPEED_LIMIT));

		// Stay inside the viewing window
		player.set_x((player.x() + player.x_vel()).clamp(0, (CAM_W - w) as i32));
		player.set_y((player.y() + player.y_vel()).clamp(0, (CAM_H - w) as i32));

		player.update_pos(XBOUNDS, YBOUNDS);

		player.set_attack_box(player.x(), player.y());

		if player.get_attack_timer() > player.get_cooldown() {
			player.cooldown();
		}
	}

	//update background
	pub fn update_background(&mut self, mut player: &mut Player) -> Result<(), String> {
		let cur_bg = Rect::new(
			((player.x() + ((player.width() / 2) as i32)) - ((CAM_W / 2) as i32)).clamp(0, (BG_W - CAM_W) as i32),
			((player.y() + ((player.height() / 2) as i32)) - ((CAM_H / 2) as i32)).clamp(0, (BG_H - CAM_H) as i32),
			CAM_W,
			CAM_H,
		);

		// Convert player's map position to be camera-relative
		let player_cam_pos = Rect::new(
			player.x() - cur_bg.x(),
			player.y() - cur_bg.y(),
			TILE_SIZE,
			TILE_SIZE,
		);

		let texture_creator = self.core.wincan.texture_creator();
		let background = background::Background::new(
			texture_creator.load_texture("images/background/floor_tile_1.png")?,
			// temp files bc i didn't feel like editing >>>>>
			texture_creator.load_texture("images/background/floor_tile_2.png")?,	
			texture_creator.load_texture("images/background/floor_tile_1.png")?,
			1, 
			1, 
		);

		self.core.wincan.set_draw_color(Color::BLACK);
		self.core.wincan.clear();

		// Draw subset of bg
		self.core.wincan.copy(background.texture(), cur_bg, None).unwrap();
		Ok(())
	}


// draw player
	pub fn draw_player(&mut self, count: &i32, f_display: &i32, player: &mut Player, cur_bg: &Rect) {
		let player_cam_pos = Rect::new(
			player.x() - cur_bg.x(),
			player.y() - cur_bg.y(),
			TILE_SIZE,
			TILE_SIZE,
		);
		
	
		if *(player.is_still()) {
			if *(player.facing_right()) {
				self.core.wincan.copy(player.texture_a_r(), player.src(), player_cam_pos).unwrap();
			} else {
				self.core.wincan.copy(player.texture_a_l(), player.src(), player_cam_pos).unwrap();
			}

			//display animation when not moving
			match count {
				count if count < f_display => { player.set_src(0 as i32, 0 as i32); }
				count if count < &(f_display * 2) => { player.set_src(64 as i32, 0 as i32); }
				count if count < &(f_display * 3) => { player.set_src(128 as i32, 0 as i32); }
				count if count < &(f_display * 4) => { player.set_src(0 as i32, 64 as i32); }
				count if count < &(f_display * 5) => { player.set_src(64 as i32, 64 as i32); }
				count if count < &(f_display * 6) => { player.set_src(128 as i32, 64 as i32); }
				count if count < &(f_display * 7) => { player.set_src(0 as i32, 128 as i32); }
				count if count < &(f_display * 8) => { player.set_src(64 as i32, 128 as i32); }
				count if count < &(f_display * 9) => { player.set_src(128 as i32, 128 as i32); }
				count if count < &(f_display * 10) => { player.set_src(0 as i32, 192 as i32); }
				count if count < &(f_display * 11) => { player.set_src(64 as i32, 192 as i32); }
				count if count < &(f_display * 12) => { player.set_src(128 as i32, 192 as i32); }


				_ => { player.set_src(0, 0); }
			}
		} else {
			player.set_src(0, 0);
			if *(player.facing_right()) {
				self.core.wincan.copy(player.texture_r(), player.src(), player_cam_pos).unwrap();
			} else {
				self.core.wincan.copy(player.texture_l(), player.src(), player_cam_pos).unwrap();
			}
		}
		//println!("\nx:{} y:{} ", player.x(), player.y());
		
	}


// force enemy movement

	pub fn check_edge(enemy: &enemy::Enemy) -> bool{
		if  enemy.x() <= XBOUNDS.0 as f64 || 
		enemy.x() >=  XBOUNDS.1 as f64 ||
		enemy.y() <= YBOUNDS.0 as f64|| 
		enemy.y() >= YBOUNDS.1 as f64
		{return true;}
		else {return false;}
	}
}