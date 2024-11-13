extern crate game_engine;

use game_engine::start_window_and_game_loop;

fn main() {
    start_window_and_game_loop!("Test Simple Game Loop", 800, 600, 16, {});
}