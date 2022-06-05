mod app;
mod game;
use crate::app::App;

pub fn main() {
    App::new("Game of Life", 1600, 1200, 20).run();
}
