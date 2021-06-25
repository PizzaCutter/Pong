extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::{Point, Rect};

use sdl2::render::{WindowCanvas, Texture};
use sdl2::image::{self, LoadTexture, InitFlag};
use rand::Rng;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Div;
use std::ops::DivAssign;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Default,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Vector2
{
    x: f32,
    y: f32,
}

impl Vector2
{
    fn new(in_x : f32, in_y : f32) -> Vector2
    {
        return Vector2{x: in_x, y: in_y};
    }

    fn magnitude(self) -> f32{
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    fn normalize(&mut self) 
    {
        let magnitude =  self.magnitude();
        *self /= magnitude;
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

#[derive(Debug)]
struct Paddle{
    position: Vector2,
    sprite: Rect,
    last_input_direction: Direction,
    movement_speed: f32,
}


impl Paddle {
    fn update(&mut self)
    {
        match self.last_input_direction{
            Direction::Default => { },
            Direction::Up => {
                self.position += Vector2::new(0.0, -self.movement_speed);
            },
            Direction::Down => {
                self.position += Vector2::new(0.0, self.movement_speed);
            },
        }

        let half_player_size = (self.sprite.width()) as f32;
        if self.position.y <= half_player_size {
            self.position.y = half_player_size;
        }
        if self.position.y >= WINDOW_HEIGHT as f32 - half_player_size {
            self.position.y = WINDOW_HEIGHT as f32 - half_player_size;
        }
    }
}

struct Ball {
   position: Vector2, 
   radius: i32,
   movement_speed: f32,
   movement_direction: Vector2 
}

impl Ball {
    fn update(&mut self, _left_paddle: &Paddle, _right_paddle: &Paddle, canvas: &WindowCanvas) 
    {
        let (width, height) = canvas.output_size().unwrap();
        if ((self.position.x - self.radius as f32) < 0.0 || self.position.x >= width as f32)
        {
            if(self.movement_direction.x > 0.0)
            {
                self.movement_direction.x *= -1.0;
            }
        }
        if ((self.position.y - self.radius as f32) < 0.0 || self.position.y >= height as f32) 
        {
            self.movement_direction.y *= -1.0;
        }

        // CHECK COLLISION WITH LEFT PADDLE
        // TODO[rsmekens]: should only check this if on next movement update we are behind the paddle
        if self.movement_direction.x < 0.0 
        {
            if (self.position.x - self.radius as f32) < (_left_paddle.position.x + _left_paddle.sprite.width() as f32 * 0.5)
            {
                if (self.position.y < _left_paddle.position.y + _left_paddle.sprite.height() as f32 * 0.5) 
                && (self.position.y > _left_paddle.position.y - _left_paddle.sprite.height() as f32 * 0.5)
                {
                    self.movement_direction.x *= -1.0;
                }
            }
        }

        //CHECK COLLISION WITH RIGHT PADDLE
        if self.movement_direction.x > 0.0
        {
            if (self.position.x + self.radius as f32) > (_right_paddle.position.x + _right_paddle.sprite.width() as f32 * 0.5)
            {
                if (self.position.y < _right_paddle.position.y + _right_paddle.sprite.height() as f32 * 0.5) 
                && (self.position.y > _right_paddle.position.y - _right_paddle.sprite.height() as f32 * 0.5)
                {
                    self.movement_direction.x *= -1.0;
                }
            }
        }

        self.position += self.movement_direction * self.movement_speed;
    }
}

fn render_paddle(canvas: &mut WindowCanvas, texture: &Texture, paddle: &Paddle) -> Result<(), String> {
    let paddle_width = paddle.sprite.width() as i32;
    let paddle_height = paddle.sprite.height() as i32;
    let render_destination = Rect::new(paddle.position.x as i32 - paddle_width / 2, paddle.position.y as i32 - paddle_height / 2, paddle_width as u32, paddle_height as u32);
    canvas.copy(texture, None, render_destination).unwrap();
    Ok(())
}

fn render_ball(canvas: &mut WindowCanvas, texture: &Texture, ball: &Ball) -> Result<(), String> {
    let render_destination = Rect::new((ball.position.x as i32 - ball.radius) as i32, (ball.position.y as i32 - ball.radius) as i32, ball.radius as u32, ball.radius as u32);
    canvas.copy(texture, None, render_destination).unwrap();
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
    let mut left_paddle = Paddle {
        position: Vector2::new(25.0, height as f32 * 0.5),
        sprite: Rect::new(0, 0, 25, 100),
        last_input_direction: Direction::Default,
        movement_speed: 3.0,
    };
    let mut right_paddle = Paddle {
        position: Vector2::new(WINDOW_WIDTH as f32 - 25.0, height as f32 * 0.5),
        sprite: Rect::new(0, 0, 25, 100),
        last_input_direction: Direction::Default,
        movement_speed: 3.0,
    };
    let mut ball = Ball {
        position: Vector2::new(width as f32 * 0.5, height as f32 * 0.5),
        radius: 32,
        movement_speed: 3.0,
        //movement_direction: Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
        movement_direction: Vector2::new(-1.0, 0.1),
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
                    right_paddle.last_input_direction = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    right_paddle.last_input_direction = Direction::Down;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    left_paddle.last_input_direction = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    left_paddle.last_input_direction = Direction::Down;
                },
                _ => {}
            }
        }

        // Updating gameplay
        left_paddle.update();
        right_paddle.update();
        ball.update(&left_paddle, &right_paddle, &canvas);

        // Rendering
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        render_paddle(&mut canvas, &texture, &left_paddle).unwrap();
        render_paddle(&mut canvas, &texture, &right_paddle).unwrap();
        render_ball(&mut canvas, &ball_texture, &ball).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}