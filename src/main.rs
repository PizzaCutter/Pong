extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

use sdl2::render::{WindowCanvas, Texture};
use sdl2::image::{self, LoadTexture, InitFlag};

fn render(canvas: &mut WindowCanvas, texture: &Texture, src: Option<Rect>, dest: Rect) -> Result<(), String> {
    canvas.copy(texture, src, dest);
    Ok(())
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    let mut paddleXPos = 25.0;
    let mut paddleYPos = 25.0;
    let mut paddleWidth = 25.0;
    let mut paddleHeight = 100.0;
    let mut paddleMovementSpeed = 10.0;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("resources/T_White.jpg").unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    paddleYPos -= paddleMovementSpeed; 
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    paddleYPos += paddleMovementSpeed;
                },
                _ => {}
            }
        }

        // Updating gameplay
        let mut dst = Rect::new(paddleXPos as i32, paddleYPos as i32, paddleWidth as u32, paddleHeight as u32);

        // Rendering
        render(&mut canvas, &texture, None, dst);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}