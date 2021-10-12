extern crate rogue_sdl;
mod enemy;
mod player;
mod RangedAttack;
mod credits;
use rand::Rng;

use std::time::Duration;
use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
//use sdl2::render::Texture;

use rogue_sdl::SDLCore;
use rogue_sdl::Game;
const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TILE_SIZE: u32 = 64;

const SPEED_LIMIT: i32 = 3;
const ACCEL_RATE: i32 = 3;

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


impl Game for ROGUELIKE {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(ROGUELIKE{ core })
	}

	fn run(&mut self) -> Result<(), String> {
        // reset frame
        let texture_creator = self.core.wincan.texture_creator();
		let w = 25;
		
		let mut rng = rand::thread_rng();
		let mut roll = rng.gen_range(1..4);
		let mut t = 0;// this is just a timer for the enemys choice of movement
		
		let mut count = 0;
		let f_display = 15;

		// create sprites
		let mut p = player::Player::new(
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
			texture_creator.load_texture("images/player/blue_slime_l.png")?,
			texture_creator.load_texture("images/player/blue_slime_r.png")?,
			texture_creator.load_texture("images/player/slime_animation_l.png")?,
			texture_creator.load_texture("images/player/slime_animation_r.png")?,
		);
		
        let mut e = enemy::Enemy::new(
            Rect::new(
                (CAM_W/2 - TILE_SIZE/2 + 100) as i32,
                (CAM_H/2 - TILE_SIZE/2 + 100) as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
            texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
        );
	
        let mut fireball = RangedAttack::RangedAttack::new(
			Rect::new(
				0, 0, TILE_SIZE, TILE_SIZE,
			),
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
            texture_creator.load_texture("images/fireball/fireball.png")?,
			false,
			0,
		);

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}
			
			

			p.set_x_delta(0);
			p.set_y_delta(0);

			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

			// move up
			if keystate.contains(&Keycode::W) {
				p.set_y_delta(p.y_delta()-ACCEL_RATE);
				p.is_still = false;
			}
            // move left
			if keystate.contains(&Keycode::A) {
				p.set_x_delta(p.x_delta()-ACCEL_RATE);
                p.facing_left = true;
				p.facing_right = false;
				p.is_still = false;
			}
            // move down
			if keystate.contains(&Keycode::S) {
				p.set_y_delta(p.y_delta()+ACCEL_RATE);
				p.is_still = false;
			}
            // move right
			if keystate.contains(&Keycode::D) {
				p.set_x_delta(p.x_delta()+ACCEL_RATE);
                p.facing_left = false;
				p.facing_right = true;
				p.is_still = false;
			}

	
			

            // shoot fireball
            if keystate.contains(&Keycode::F) && fireball.frame()==0{
				fireball.set_use(true);
				fireball.start_pos(p.x(), p.y());
				println!("{}", fireball.x());
			}
			if fireball.in_use() {
				fireball.set_frame(fireball.frame()+1); 
				fireball.update_RangedAttack_pos((0, (CAM_W - TILE_SIZE) as i32));
				if fireball.frame()==28 {
					fireball.set_use(false);
					fireball.set_frame(0);
				}
			}

			// Slow down to 0 vel if no input and non-zero velocity
			p.set_x_delta(resist(p.x_vel(), p.x_delta()));
			p.set_y_delta(resist(p.y_vel(), p.y_delta()));
			
			//when player is not moving
			//print!("{},{}", p.x_vel(), p.y_vel());
			if(p.x_vel() == 0  && p.y_vel() == 0 ){p.is_still = true;}//?

			// Don't exceed speed limit
			p.set_x_vel((p.x_vel() + p.x_delta()).clamp(-SPEED_LIMIT, SPEED_LIMIT));
			p.set_y_vel((p.y_vel() + p.y_delta()).clamp(-SPEED_LIMIT, SPEED_LIMIT));

			// Stay inside the viewing window
			p.set_x((p.x() + p.x_vel()).clamp(0, (CAM_W - w) as i32));
			p.set_y((p.y() + p.y_vel()).clamp(0, (CAM_H - w) as i32));

            p.update_pos((0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));

			if check_collision(&p.pos(), &e.pos())
				|| p.pos().left() < 0
				|| p.pos().right() > CAM_W as i32
			{
				p.set_x(  (p.x() - p.x_vel())  );
				p.minus_hp(0.2);
				println!("hp = {}", p.get_hp());
			}
			
			if check_collision(&p.pos(), &e.pos())
				|| p.pos().top() < 0
				|| p.pos().bottom() > CAM_H as i32
			{
				p.set_y(  (p.y() - p.y_vel())   );
				p.minus_hp(0.2);
				println!("hp = {}", p.get_hp());
			}


			
		
			

			if(t >50){
				roll = rng.gen_range(1..5);
				t=0;
			}
			e.update_pos(roll, (0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));

			
			
			//self.core.wincan.set_draw_color(Color::BLACK);
			//self.core.wincan.clear();

            let background = texture_creator.load_texture("images/background/bb.png")?;
            self.core.wincan.copy(&background, None, None)?;

            /* let cur_bg = Rect::new(
				((p.x() + ((p.width() / 2) as i32)) - ((CAM_W / 2) as i32)).clamp(0, (CAM_W * 2 - CAM_W) as i32),
				((p.y() + ((p.height() / 2) as i32)) - ((CAM_H / 2) as i32)).clamp(0, (CAM_H * 2 - CAM_H) as i32),
				CAM_W,
				CAM_H,
			); */

			self.core.wincan.copy(e.txtre(), e.src(), e.pos())?;

			
			/*
			if*(p.facing_left()) {
			p.set_src(0, 0);
                self.core.wincan.copy(p.texture_l(), p.src(), p.pos())?;
                if fireball.in_use() {self.core.wincan.copy(fireball.txtre(), fireball.src(4, 7), fireball.pos())?;}
            
			}*/
			if *(p.is_still()){

				if*(p.facing_right()) {
					self.core.wincan.copy(p.texture_a_r(), p.src(), p.pos())?;
				}
				else {
					self.core.wincan.copy(p.texture_a_l(), p.src(), p.pos())?;
				}
			    
				//display animation when not moving
				match count{
					count if count < f_display => {p.set_src(0 as i32, 0 as i32);}
					count if count < f_display*2 => {p.set_src(TILE_SIZE as i32, 0 as i32);}
					count if count < f_display*3 => {p.set_src(0 as i32, TILE_SIZE as i32);}
					count if count < f_display*4 => {p.set_src(TILE_SIZE as i32, TILE_SIZE as i32);}
					_ =>{p.set_src(0, 0);}	
				}
				count = count + 1;
				if(count > f_display * 5){
					count = 0;
				}
				if fireball.in_use() {self.core.wincan.copy(fireball.txtre(), fireball.src(4, 7), fireball.pos())?;}
			}  
			
			else {
			p.set_src(0, 0);
				if*(p.facing_right()) {
					self.core.wincan.copy(p.texture_r(), p.src(), p.pos())?;
				}
				else {
					self.core.wincan.copy(p.texture_l(), p.src(), p.pos())?;
				}
                
                if fireball.in_use() {self.core.wincan.copy(fireball.txtre(), fireball.src(4, 7), fireball.pos())?;}
            }
			self.core.wincan.present();

			t +=1 ;

			if p.get_hp() <= 0.0 {
				break 'gameloop		
			}
		}
		// Out of game loop, return Ok
		Ok(())
	}
}

pub fn main() -> Result<(), String> {
    rogue_sdl::runner(TITLE, ROGUELIKE::init);
     //credits::run_credits();
    Ok(())
}
