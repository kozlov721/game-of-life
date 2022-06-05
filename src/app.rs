use crate::game::Game;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};

use std::time::Duration;
use std::thread;

pub struct App {
    game: Game,
    cell_size: u32,
    canvas: Canvas::<Window>,
    event_pump: EventPump,
    paused: bool,
    is_running: bool,
    draw_mode: bool,
    n_alive: u32,
}

impl App {
    pub fn new(name: &str, width: u32, height: u32, cell_size: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();

        let mut s = Self {
            game: Game::new(width / cell_size, height / cell_size, true),
            cell_size,
            canvas: sdl_context
                .video()
                .unwrap()
                .window(name, width, height)
                .position_centered()
                .build()
                .unwrap()
                .into_canvas()
                .present_vsync()
                .build()
                .unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            paused: false,
            is_running: true,
            draw_mode: false,
            n_alive: 0
        };
        s.n_alive = s.game.next_generation();
        s
    }

    pub fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.is_running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    self.paused = !self.paused;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if self.paused {
                        self.game.next_generation();
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if self.paused {
                        self.game.step_back();
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    self.game.randomize();
                    self.n_alive = self.game.next_generation();
                }
                Event::MouseButtonDown { x, y, .. } => {
                    let i = y / self.cell_size as i32;
                    let j = x / self.cell_size as i32;
                    self.game.set_cell(i, j, true);
                    self.draw_mode = true;
                }
                Event::MouseButtonUp { .. } => {
                    self.draw_mode = false;
                }
                Event::MouseMotion { x, y, .. } if self.draw_mode => {
                    let i = y / self.cell_size as i32;
                    let j = x / self.cell_size as i32;
                    self.game.set_cell(i, j, true);
                }
                _ => {}
            }
        }
    }

    pub fn redraw(&mut self) {
        let (width, height) = self.canvas.window().drawable_size();
        // rgb(70, 70, 70)
        self.canvas.set_draw_color(Color::RGB(100, 100, 100));
        self.canvas.clear();

        // rgb(200,200,200)
        self.canvas.set_draw_color(Color::RGB(210, 210, 210));
        for x in (0..width).step_by(self.cell_size as usize) {
            for y in (0..height).step_by(self.cell_size as usize) {
                if self.game.get_cell((y / self.cell_size) as i32, (x / self.cell_size) as i32).get_state() {
                    self.canvas.fill_rect(Rect::new(x as i32, y as i32, self.cell_size, self.cell_size)).unwrap();
                }
                if x != 0 && y != 0 {
                self.canvas.draw_line(Point::new(x as i32, 0),
                    Point::new(x as i32, height as i32)).unwrap();
                self.canvas.draw_line(Point::new(0, y as i32),
                    Point::new(width as i32, y as i32)).unwrap();
                }
            }
        }

        self.canvas.present();
    }

    pub fn run(&mut self) {
        while self.is_running {
            if !self.paused && !self.draw_mode {
                self.n_alive = self.game.next_generation();
            }
            self.handle_events();
            self.redraw();
            thread::sleep(Duration::from_millis(70));
        }
    }
}
