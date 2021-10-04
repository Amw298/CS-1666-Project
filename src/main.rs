extern crate sdl2;

use sdl2::event::Event;
use std::time::Duration;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;

const TITLE: &str = "Roguelike Credits";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TIMEOUT: u64 = 4500;

fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(TITLE, CAM_W, CAM_H)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut i = 0;
    'credits_loop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'credits_loop,
                Event::KeyDown{keycode: Some(Keycode::Q), ..} => break 'credits_loop,
                _ => {},
            }
        }
        // Copy image texture to canvas, present, timeout

	let mut texture = texture_creator.load_texture("images/credits/credits_title.png")?;

        // Title
        match i {
            i if i < 200 => {
                texture = texture_creator.load_texture("images/credits/credits_title.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < 400 => {
                texture = texture_creator.load_texture("images/credits/credits_davon.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < 600 => {
                texture = texture_creator.load_texture("images/credits/credits_daniel.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < 800 => {
                texture = texture_creator.load_texture("images/credits/credits_victor.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < 1000 => {
                texture = texture_creator.load_texture("images/credits/credits_adam.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < 1200 => {
                texture = texture_creator.load_texture("images/credits/Yihua_credit.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < 1400 => {
                texture = texture_creator.load_texture("images/credits/zih_credit.jpg")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            _ => {
                texture = texture_creator.load_texture("images/credits/credits_title.png")?;
            }
        }

	i += 1;

        if(i > 1400) {
            i = 0;
        }
    }
    Ok(())
}

fn main() {
    run();
}
