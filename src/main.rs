mod game;
use crate::game::Game;


extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use std::time::Duration;
use std::thread;

static WIDTH: u32 = 1600;
static HEIGHT: u32 = 1200;
static CELL_SIZE: u32 = 20;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game = Game::new(
        (WIDTH / CELL_SIZE) as usize, (HEIGHT / CELL_SIZE) as usize, true);


    'running: loop {
        game.next_generation();
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(100, 100, 100)); // rgb(70, 70, 70)
        canvas.clear();

        canvas.set_draw_color(Color::RGB(210, 210, 210)); // rgb(200,200,200)
        for x in (0..WIDTH).step_by(CELL_SIZE as usize) {
            for y in (0..HEIGHT).step_by(CELL_SIZE as usize) {
                if game.get_cell((y / CELL_SIZE) as i32, (x / CELL_SIZE) as i32).get_state() {
                    canvas.fill_rect(Rect::new(x as i32, y as i32, CELL_SIZE, CELL_SIZE)).unwrap();
                }
                if x != 0 && y != 0 {
                canvas.draw_line(Point::new(x as i32, 0), Point::new(x as i32, HEIGHT as i32)).unwrap();
                canvas.draw_line(Point::new(0, y as i32), Point::new(WIDTH as i32, y as i32)).unwrap();
                }
            }
        }

        canvas.present();
        thread::sleep(Duration::from_millis(70));
    }
}
