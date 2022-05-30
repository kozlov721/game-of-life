mod game;
use crate::game::Game;
use std::{thread, time};

fn main() {
    let mut game = Game::new(40, 30, true);
    loop {
        println!("{}", game.to_string());
        game.next_generation();
        thread::sleep(time::Duration::from_millis(100));
        print!("{}[2J", 27 as char);
    }
}
