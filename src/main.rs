extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::{Point, Rect};

use sdl2::render::{WindowCanvas, Texture};
use sdl2::image::{self, LoadTexture, InitFlag};
use rand::Rng;

const PADDLE_MOVEMENT_SPEED: i32 = 5;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Default,
    Up,
    Down,
}

#[derive(Debug)]
struct Paddle{
    position: Point,
    sprite: Rect,
    lastInputDirection: Direction,
}

impl Paddle {
    fn update(&mut self)
    {
        match self.lastInputDirection{
            Direction::Default => { },
            Direction::Up => {
                self.position = self.position.offset(0, -PADDLE_MOVEMENT_SPEED);
            },
            Direction::Down => {
                self.position = self.position.offset(0, PADDLE_MOVEMENT_SPEED);
            },
        }

        let halfPlayerSize = (self.sprite.width()) as i32;
        if(self.position.y <= halfPlayerSize) {
            self.position.y = halfPlayerSize;
        }
        if(self.position.y >= WINDOW_HEIGHT as i32 - halfPlayerSize) {
            self.position.y = WINDOW_HEIGHT as i32 - halfPlayerSize;
        }
    }
}

struct Ball {
   position: Point, 
   radius: i32,
   movementSpeed: i32,
   movementDirection: Point
}

impl Ball {
    fn update(&mut self, leftPaddle: &Paddle, rightPaddle: &Paddle, canvas: &WindowCanvas) 
    {
        let (width, height) = canvas.output_size().unwrap();
        if(self.position.x - self.radius < 0 || self.position.x >= width as i32)
        {
            self.movementDirection.x *= -1;
        }
        if(self.position.y - self.radius < 0 || self.position.y >= height as i32) {
            self.movementDirection.y *= -1;
        }

        self.position += self.movementDirection * self.movementSpeed;
    }
}

fn render_paddle(canvas: &mut WindowCanvas, texture: &Texture, paddle: &Paddle) -> Result<(), String> {
    let paddleWidth = paddle.sprite.width() as i32;
    let paddleHeight = paddle.sprite.height() as i32;
    let renderRect = Rect::new(paddle.position.x - paddleWidth / 2, paddle.position.y - paddleHeight / 2, paddleWidth as u32, paddleHeight as u32);
    canvas.copy(texture, None, renderRect);
    Ok(())
}

fn render_ball(canvas: &mut WindowCanvas, texture: &Texture, ball: &Ball) -> Result<(), String> {
    let renderRect = Rect::new((ball.position.x - ball.radius) as i32, (ball.position.y - ball.radius) as i32, ball.radius as u32, ball.radius as u32);
    canvas.copy(texture, None, renderRect);
    Ok(())
}


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Pong", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let (width, height) = canvas.output_size().unwrap();

    let mut rng = rand::thread_rng();
    let mut leftPaddle = Paddle {
        position: Point::new(25, height as i32 / 2),
        sprite: Rect::new(0, 0, 25, 100),
        lastInputDirection: Direction::Default,
    };
    let mut rightPaddle = Paddle {
        position: Point::new(WINDOW_WIDTH as i32 - 25, height as i32 / 2),
        sprite: Rect::new(0, 0, 25, 100),
        lastInputDirection: Direction::Default,
    };
    let mut ball = Ball {
        position: Point::new(width as i32 / 2, height as i32 / 2),
        radius: 32,
        movementSpeed: 3,
        movementDirection: Point::new(rng.gen_range(-1..1), (rng.gen_range(-1..1))),
    };

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("resources/T_White.jpg").unwrap();
    let ball_texture = texture_creator.load_texture("resources/T_Circle.png").unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    rightPaddle.lastInputDirection = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    rightPaddle.lastInputDirection = Direction::Down;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    leftPaddle.lastInputDirection = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    leftPaddle.lastInputDirection = Direction::Down;
                },
                _ => {}
            }
        }

        // Updating gameplay
        leftPaddle.update();
        rightPaddle.update();
        ball.update(&leftPaddle, &rightPaddle, &canvas);

        // Rendering
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        render_paddle(&mut canvas, &texture, &leftPaddle);
        render_paddle(&mut canvas, &texture, &rightPaddle);
        render_ball(&mut canvas, &ball_texture, &ball);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}