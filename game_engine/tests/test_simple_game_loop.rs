extern crate game_engine;

use game_engine::{create_game_window, update_game_window, window_should_close};

fn main() {
    create_game_window("C Test Game", 800, 600);
    while !window_should_close() {
        update_game_window();
        std::thread::sleep(std::time::Duration::from_millis(16)); // Optional: Sleep to reduce CPU usage
    }
}